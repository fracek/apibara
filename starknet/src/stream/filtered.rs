//! Filtered data stream.

use std::{
    pin::Pin,
    sync::Arc,
    task::{self, Poll, Waker},
};

use futures::Stream;
use tracing::debug;

use crate::{
    core::{
        pb::stream::{
            self,
            v1alpha2::{DataFinality, StreamDataResponse},
        },
        GlobalBlockId, IngestionMessage,
    },
    db::StorageReader,
};

use super::{
    block::{BlockDataFilter, DatabaseBlockDataFilter},
    configuration::StreamConfiguration,
    StreamError,
};

pub struct FilteredDataStream<R>
where
    R: StorageReader,
{
    storage: Arc<R>,
    waker: Option<Waker>,
    inner: Option<InnerDataStream<R>>,
}

#[derive(Debug, thiserror::Error)]
pub enum FilteredDataStreamError {
    #[error("no finalized block ingested yet")]
    NoFinalizedBlockIngested,
}

struct InnerDataStream<R: StorageReader> {
    stream_id: u64,
    batch_size: usize,
    data_finality: DataFinality,
    previous_iter_cursor: Option<GlobalBlockId>,
    finalized_cursor: GlobalBlockId,
    accepted_cursor: GlobalBlockId,
    filter: DatabaseBlockDataFilter<R>,
    storage: Arc<R>,
}

impl<R> FilteredDataStream<R>
where
    R: StorageReader,
{
    pub fn new(storage: Arc<R>) -> Self {
        FilteredDataStream {
            storage,
            inner: None,
            waker: None,
        }
    }

    pub fn reconfigure_data_stream(
        &mut self,
        configuration: StreamConfiguration,
    ) -> Result<(), StreamError> {
        // use finalized and accepted cursors from previous config, if any
        let (finalized_cursor, accepted_cursor) = if let Some(inner) = self.inner.take() {
            (inner.finalized_cursor, inner.accepted_cursor)
        } else {
            let finalized_cursor = self
                .storage
                .highest_finalized_block()
                .map_err(StreamError::internal)?
                .ok_or(FilteredDataStreamError::NoFinalizedBlockIngested)
                .map_err(StreamError::internal)?;
            // use finalized block if the node hasn't ingested an accepted block yet
            let accepted_cursor = self
                .storage
                .highest_accepted_block()
                .map_err(StreamError::internal)?
                .unwrap_or(finalized_cursor);
            (finalized_cursor, accepted_cursor)
        };

        let filter = DatabaseBlockDataFilter::new(self.storage.clone(), configuration.filter);

        let inner = InnerDataStream {
            stream_id: configuration.stream_id,
            batch_size: configuration.batch_size,
            data_finality: configuration.finality,
            previous_iter_cursor: configuration.starting_cursor,
            finalized_cursor,
            accepted_cursor,
            filter,
            storage: self.storage.clone(),
        };

        self.inner = Some(inner);
        self.wake();

        Ok(())
    }

    pub fn handle_ingestion_message(
        &mut self,
        message: IngestionMessage,
    ) -> Result<(), StreamError> {
        if let Some(inner) = &mut self.inner {
            match message {
                IngestionMessage::Accepted(block_id) => {
                    inner.accepted_cursor = block_id;
                    self.wake();
                }
                IngestionMessage::Finalized(block_id) => {
                    inner.finalized_cursor = block_id;
                    self.wake();
                }
            }
        }

        Ok(())
    }

    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

impl<R> InnerDataStream<R>
where
    R: StorageReader,
{
    pub fn advance_to_next_batch(&mut self) -> Result<Option<StreamDataResponse>, StreamError> {
        // if next block is still in the finalized range, send a batch
        // if it's between finalized and accepted, send a single block
        // otherwise just wait and connect waker
        debug!(
            previous_iter_cursor = ?self.previous_iter_cursor,
            finalized_cursor = ?self.finalized_cursor,
            accepted_cursor = ?self.accepted_cursor,
            "advance next batch"
        );

        let next_block_number = self
            .previous_iter_cursor
            .map(|c| c.number() + 1)
            .unwrap_or(0);

        let next_cursor = if let Some(cursor) = self
            .storage
            .canonical_block_id(next_block_number)
            .map_err(StreamError::internal)?
        {
            cursor
        } else {
            // next block not ingested. wait until it is.
            return Ok(None);
        };

        // send finalized data always
        if next_block_number <= self.finalized_cursor.number() {
            return self.send_finalized_batch(next_cursor);
        }

        // only send accepted data to the right streams
        if self.data_finality == DataFinality::DataStatusAccepted
            || self.data_finality == DataFinality::DataStatusPending
        {
            return self.send_accepted_batch(next_cursor);
        }

        // nothing to do
        Ok(None)
    }

    /// Send a batch of finalized data, starting from the given cursor (inclusive).
    fn send_finalized_batch(
        &mut self,
        first_cursor: GlobalBlockId,
    ) -> Result<Option<StreamDataResponse>, StreamError> {
        use stream::v1alpha2::stream_data_response::Message;

        let batch_start_cursor = self.previous_iter_cursor.map(|c| c.to_cursor());

        let mut batch = Vec::with_capacity(self.batch_size);
        let mut batch_end_cursor = None;
        let mut current_cursor = first_cursor;

        while batch.len() < self.batch_size {
            if let Some(data) = self
                .filter
                .data_for_block(&current_cursor)
                .map_err(StreamError::internal)?
            {
                batch_end_cursor = Some(current_cursor);
                batch.push(data);
            }

            match self
                .storage
                .canonical_block_id(current_cursor.number() + 1)
                .map_err(StreamError::internal)?
            {
                None => {
                    // reached the highest indexed block. return what we have
                    break;
                }
                Some(cursor) => {
                    // don't mix accepted and finalized data
                    if cursor.number() > self.finalized_cursor.number() {
                        break;
                    }
                    current_cursor = cursor;
                }
            }
        }

        // update iter cursor to the latest ingested block.
        self.previous_iter_cursor = batch_end_cursor;

        let data = stream::v1alpha2::Data {
            cursor: batch_start_cursor,
            end_cursor: batch_end_cursor.map(|c| c.to_cursor()),
            finality: DataFinality::DataStatusFinalized as i32,
            data: batch,
        };

        let response = StreamDataResponse {
            stream_id: self.stream_id,
            message: Some(Message::Data(data)),
        };

        Ok(Some(response))
    }

    /// Send a batch of accepted data, starting from the given cursor (inclusive).
    fn send_accepted_batch(
        &mut self,
        first_cursor: GlobalBlockId,
    ) -> Result<Option<StreamDataResponse>, StreamError> {
        use stream::v1alpha2::stream_data_response::Message;

        let batch_start_cursor = self.previous_iter_cursor.map(|c| c.to_cursor());
        self.previous_iter_cursor = Some(first_cursor);

        // read data at cursor
        let data = if let Some(data) = self
            .filter
            .data_for_block(&first_cursor)
            .map_err(StreamError::internal)?
        {
            data
        } else {
            return Ok(None);
        };

        let data = stream::v1alpha2::Data {
            cursor: batch_start_cursor,
            end_cursor: Some(first_cursor.to_cursor()),
            finality: DataFinality::DataStatusAccepted as i32,
            data: vec![data],
        };

        let response = StreamDataResponse {
            stream_id: self.stream_id,
            message: Some(Message::Data(data)),
        };

        Ok(Some(response))
    }
}

impl<R> Stream for FilteredDataStream<R>
where
    R: StorageReader,
{
    type Item = Result<StreamDataResponse, StreamError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        // connect wakers so that the stream is polled again when configuration or
        // state changes
        self.waker = Some(cx.waker().clone());

        // if `inner` is missing, then the block was never configured.
        // nothing to do.
        let inner = if let Some(inner) = &mut self.inner {
            inner
        } else {
            return Poll::Pending;
        };

        match inner.advance_to_next_batch() {
            Err(err) => Poll::Ready(Some(Err(err))),
            Ok(None) => Poll::Pending,
            Ok(Some(data)) => Poll::Ready(Some(Ok(data))),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(inner) = &self.inner {
            let current = inner.previous_iter_cursor.map(|c| c.number()).unwrap_or(0);
            let head = inner.accepted_cursor.number();
            let difference = (head - current) as usize;
            (difference, None)
        } else {
            (0, None)
        }
    }
}