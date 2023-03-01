use std::{
    fs,
    marker::PhantomData,
    net::{AddrParseError, SocketAddr},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use apibara_node::db::{
    default_data_dir,
    libmdbx::{self, Environment, EnvironmentKind},
    MdbxEnvironmentExt,
};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::{
    db::tables,
    healer::{Healer, HealerError},
    ingestion::{BlockIngestion, BlockIngestionConfig, BlockIngestionError},
    provider::{HttpProviderError, Provider},
    server::{RequestObserver, Server, ServerError, SimpleRequestObserver},
    HttpProvider,
};

pub struct StarkNetNode<G, O, E>
where
    G: Provider + Send + Sync + 'static,
    O: RequestObserver,
    E: EnvironmentKind,
{
    db: Arc<Environment<E>>,
    sequencer_provider: Arc<G>,
    request_span: O,
}

#[derive(Debug, thiserror::Error)]
pub enum StarkNetNodeError {
    #[error("failed while ingesting blocks")]
    BlockIngestion(BlockIngestionError),
    #[error("database operation failed")]
    Database(#[from] libmdbx::Error),
    #[error("server error")]
    Server(#[from] ServerError),
    #[error("healer error")]
    Healer(#[from] HealerError),
    #[error("error parsing server address")]
    AddressParseError(#[from] AddrParseError),
}

impl<G, O, E> StarkNetNode<G, O, E>
where
    G: Provider + Send + Sync + 'static,
    O: RequestObserver,
    E: EnvironmentKind,
{
    /// Creates a new builder, used to configure the node.
    pub fn builder(
        url: &str,
    ) -> Result<StarkNetNodeBuilder<SimpleRequestObserver, E>, StarkNetNodeBuilderError> {
        StarkNetNodeBuilder::<SimpleRequestObserver, E>::new(url)
    }

    pub(crate) fn new(db: Environment<E>, sequencer_provider: G, request_span: O) -> Self {
        let db = Arc::new(db);
        let sequencer_provider = Arc::new(sequencer_provider);
        StarkNetNode {
            db,
            sequencer_provider,
            request_span,
        }
    }

    /// Starts the node.
    pub async fn start(self, ct: CancellationToken) -> Result<(), StarkNetNodeError> {
        info!("starting starknet node");
        self.ensure_tables()?;

        // TODO: config from command line
        let (block_ingestion_client, block_ingestion) = BlockIngestion::new(
            self.sequencer_provider.clone(),
            self.db.clone(),
            BlockIngestionConfig::default(),
        );

        let mut block_ingestion_handle = tokio::spawn({
            let ct = ct.clone();
            async move {
                block_ingestion
                    .start(ct)
                    .await
                    .map_err(StarkNetNodeError::BlockIngestion)
            }
        });

        let (healer_client, healer) = Healer::new(self.sequencer_provider.clone(), self.db.clone());

        let mut healer_handle = tokio::spawn({
            let ct = ct.clone();
            async move { healer.start(ct).await.map_err(StarkNetNodeError::Healer) }
        });

        // TODO: configure from command line
        let server_addr: SocketAddr = "0.0.0.0:7171".parse()?;
        let server = Server::<E, O>::new(self.db.clone(), block_ingestion_client, healer_client)
            .with_request_observer(self.request_span);
        let mut server_handle = tokio::spawn({
            let ct = ct.clone();
            async move {
                server
                    .start(server_addr, ct)
                    .await
                    .map_err(StarkNetNodeError::Server)
            }
        });

        // TODO: based on which handles terminates first, it needs to wait
        // for the other handle to terminate too.
        tokio::select! {
            ret = &mut block_ingestion_handle => {
                warn!(result = ?ret, "block ingestion terminated");
            }
            ret = &mut server_handle => {
                warn!(result = ?ret, "server terminated");
            }
            ret = &mut healer_handle => {
                warn!(result = ?ret, "healer terminated");
            }
        }

        info!("terminated. bye");
        Ok(())
    }

    fn ensure_tables(&self) -> Result<(), StarkNetNodeError> {
        let txn = self.db.begin_rw_txn()?;
        tables::ensure(&txn)?;
        txn.commit()?;
        Ok(())
    }
}

pub struct StarkNetNodeBuilder<O: RequestObserver, E: EnvironmentKind> {
    datadir: PathBuf,
    provider: HttpProvider,
    poll_interval: Duration,
    request_observer: O,
    _phantom: PhantomData<E>,
}

#[derive(Debug, thiserror::Error)]
pub enum StarkNetNodeBuilderError {
    #[error("failed to create datadir")]
    CreateDatadir(std::io::Error),
    #[error("failed to open mdbx database")]
    DatabaseOpen(libmdbx::Error),
    #[error("failed to parse provider url")]
    ProviderUrl(#[from] url::ParseError),
    #[error("failed to create sequencer")]
    Provider(#[from] HttpProviderError),
}

impl<O, E> StarkNetNodeBuilder<O, E>
where
    O: RequestObserver,
    E: EnvironmentKind,
{
    pub(crate) fn new(
        url: &str,
    ) -> Result<StarkNetNodeBuilder<SimpleRequestObserver, E>, StarkNetNodeBuilderError> {
        let datadir = default_data_dir()
            .map(|d| d.join("starknet"))
            .expect("no datadir");
        let url = url.parse()?;
        let sequencer = HttpProvider::new(url);
        let poll_interval = Duration::from_millis(5_000);
        let request_observer = SimpleRequestObserver::default();
        let builder = StarkNetNodeBuilder {
            datadir,
            provider: sequencer,
            poll_interval,
            request_observer,
            _phantom: Default::default(),
        };
        Ok(builder)
    }

    pub fn with_datadir(&mut self, datadir: PathBuf) {
        self.datadir = datadir;
    }

    pub fn with_poll_interval(&mut self, poll_interval: Duration) {
        self.poll_interval = poll_interval;
    }

    pub fn with_request_observer<N: RequestObserver>(
        self,
        request_observer: N,
    ) -> StarkNetNodeBuilder<N, E> {
        StarkNetNodeBuilder {
            datadir: self.datadir,
            provider: self.provider,
            poll_interval: self.poll_interval,
            request_observer,
            _phantom: self._phantom,
        }
    }

    pub fn build(self) -> Result<StarkNetNode<HttpProvider, O, E>, StarkNetNodeBuilderError> {
        fs::create_dir_all(&self.datadir).map_err(StarkNetNodeBuilderError::CreateDatadir)?;

        let db = Environment::<E>::builder()
            .with_size_gib(10, 100)
            .with_growth_step_gib(2)
            .open(&self.datadir)
            .map_err(StarkNetNodeBuilderError::DatabaseOpen)?;

        Ok(StarkNetNode::new(db, self.provider, self.request_observer))
    }
}
