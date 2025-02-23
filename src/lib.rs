pub mod entity;
pub mod server;
pub mod state;

use serde::Deserialize;
use state::AppState;
use tracing::{debug, trace};

#[derive(Deserialize, Clone, Copy)]
pub struct AppConfig {
    pub max_query_results: i32,
}

pub async fn run(state: AppState, tx: tokio::sync::oneshot::Sender<u16>) -> anyhow::Result<()> {
    server::serve(state, tx).await
}
