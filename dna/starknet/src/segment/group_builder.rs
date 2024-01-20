use apibara_dna_common::{
    error::Result,
    segment::{SegmentExt, SegmentOptions},
    storage::{StorageBackend, StorageWriter},
};
use flatbuffers::FlatBufferBuilder;
use tracing::info;

use crate::segment::store;

use super::builder::SegmentEvent;

/// A builder for a segment group.
///
/// A segment group is a collection of pointers to segments
/// and indices to quickly access data in them.
pub struct SegmentGroupBuilder<'a, S: StorageBackend> {
    storage: S,
    options: SegmentOptions,
    builder: FlatBufferBuilder<'a>,
    first_block_number: u64,
    segment_count: usize,
}

pub struct SegmentGroupSummary {
    pub first_block_number: u64,
}

pub enum SegmentGroupEvent {
    None,
    Flushed(SegmentGroupSummary),
}

impl<'a, S> SegmentGroupBuilder<'a, S>
where
    S: StorageBackend,
{
    pub fn new(storage: S, options: SegmentOptions) -> Self {
        Self {
            storage,
            options,
            builder: FlatBufferBuilder::new(),
            first_block_number: 0,
            segment_count: 0,
        }
    }

    pub async fn handle_segment_event(&mut self, event: SegmentEvent) -> Result<SegmentGroupEvent> {
        let summary = match event {
            SegmentEvent::None => return Ok(SegmentGroupEvent::None),
            SegmentEvent::Flushed(summary) => summary,
        };

        assert_eq!(summary.size, self.options.segment_size);

        let segment_start = summary.first_block_number.segment_start(&self.options);
        info!(segment_start, "flushed segment");

        if self.segment_count == 0 {
            self.first_block_number = segment_start;
        }
        self.segment_count += 1;

        if self.segment_count < self.options.group_size {
            return Ok(SegmentGroupEvent::None);
        }

        let mut writer = self.storage.writer("group").await?;

        self.flush(&mut writer).await
    }

    async fn flush<W: StorageWriter>(&mut self, writer: &mut W) -> Result<SegmentGroupEvent> {
        info!("flushing segment group");
        let first_block_number = self.first_block_number;
        let group_name = first_block_number.format_segment_group_name(&self.options);

        let mut group = store::SegmentGroupBuilder::new(&mut self.builder);
        group.add_first_block_number(self.first_block_number);
        group.add_segment_size(self.options.segment_size as u32);
        group.add_segment_count(self.segment_count as u32);

        let group = group.finish();
        self.builder.finish(group, None);

        writer
            .put(&group_name, self.builder.finished_data())
            .await?;

        self.segment_count = 0;
        self.first_block_number = 0;
        self.builder.reset();

        let summary = SegmentGroupSummary { first_block_number };
        Ok(SegmentGroupEvent::Flushed(summary))
    }
}