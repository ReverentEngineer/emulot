use std::sync::Arc;
use serde::Deserialize;
use axum::{
    Extension,
    Router,
    Server,
    http::StatusCode
};
use crate::{
    Error,
    error::ErrorKind,
    storage::ConfigStorage,
    orchestrator::Orchestrator
};


impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self.kind() {
            ErrorKind::AlreadyRunning => StatusCode::NOT_MODIFIED,
            ErrorKind::AlreadyStopped => StatusCode::NOT_MODIFIED,
            ErrorKind::AlreadyExists => StatusCode::CONFLICT,
            ErrorKind::NoSuchEntity => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, format!("{self}")).into_response()
    }
}

mod guest;

/// State of the daemon
#[derive(Clone)]
pub struct State {

    /// Persistent storage
    storage: Arc<ConfigStorage>,

    /// Handles guest management
    orchestrator: Arc<Orchestrator>
}

#[derive(Clone, Deserialize)]
pub struct DaemonConfig {

    /// Daemon listen interface
    listen: String,

    /// Database url
    url: String

}

fn app(state: State) -> Router {
    Router::new()
        .nest("/guests", guest::router())
        .layer(Extension(state))
}

#[tokio::main(flavor = "current_thread")]
pub async fn run(config: &DaemonConfig) {
    let config_storage = Arc::new(ConfigStorage::new(&config.url).await.expect("This shouldn't happen yet."));
    let state = State {
        storage: config_storage.clone(),
        orchestrator: Orchestrator::new(config_storage).into()
    };
    Server::bind(&config.listen.as_str().parse().unwrap())
        .serve(app(state).into_make_service())
        .await.expect("Failed to listen");
}
