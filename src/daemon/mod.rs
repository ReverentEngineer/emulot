use std::{
    sync::Arc,
    net::ToSocketAddrs,
};
use serde::Deserialize;
use axum::{
    Extension,
    Json,
    Router,
    routing::get,
    Server,
    http::StatusCode,
};
use crate::{
    Error,
    error::ErrorKind,
    storage::ConfigStorage,
    orchestrator::Orchestrator
};
use url::Url;

mod unix;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self.kind() {
            ErrorKind::AlreadyRunning => StatusCode::NOT_MODIFIED,
            ErrorKind::AlreadyStopped => StatusCode::NOT_MODIFIED,
            ErrorKind::AlreadyExists => StatusCode::CONFLICT,
            ErrorKind::NoSuchEntity => StatusCode::NOT_FOUND,
            ErrorKind::Pending => StatusCode::SERVICE_UNAVAILABLE,
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
    #[serde(deserialize_with = "crate::de::deserialize_url")]
    listen: Url,

    /// Database url
    uri: String

}

fn default_storage_uri() -> String {
    format!("{}/storage.db", crate::runtime_dir())
}

impl Default for DaemonConfig {

    fn default() -> Self {
        Self {
            listen: crate::default_url(),
            uri: default_storage_uri()
        }
    }

}

async fn health(
    state: Extension<State>
    ) -> Result<Json<String>, Error>
{
    let text = match state.storage.list(None, None).is_ok() {
       true => "Ready".to_string(),
       false => return Err(Error::new(ErrorKind::Pending, "Unable to access database"))
    };
    Ok(Json(text))
}

fn app(state: State) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/guests", guest::router())
        .layer(Extension(state))
}

#[tokio::main(flavor = "current_thread")]
pub async fn run(config: &DaemonConfig) -> Result<(), Error> {
    let storage_uri = std::env::var("EMULOT_STORAGE_URI").ok()
        .unwrap_or(config.uri.clone());
    let config_storage = Arc::new(ConfigStorage::new(storage_uri)?);
    let state = State {
        storage: config_storage.clone(),
        orchestrator: Orchestrator::new(config_storage).into()
    };

    let listen = std::env::var("EMULOT_LISTEN").ok()
        .and_then(|listen| Url::parse(&listen).ok())
        .unwrap_or(config.listen.clone());
    match listen.scheme() {
        "tcp" => {
            let host = listen.host_str().unwrap();
            let port = listen.port().unwrap();
            if let Some(addr) = format!("{host}:{port}").to_socket_addrs().unwrap().next() {
                Server::bind(&addr)
                    .serve(app(state).into_make_service())
                    .await.map_err(|err| err.into())
            } else {
                todo!()
            }
        },
        "unix" => {
            let path = crate::de::percent_decode(listen.path()).expect("Failed to decode");
            if std::fs::metadata(&path).is_ok() {
                let _ = std::fs::remove_file(&path).expect("Failed to remove");
            }
            let acceptor = unix::UnixAcceptor::bind(&path).expect("Failed to listen");
            Server::builder(acceptor)
                .serve(app(state).into_make_service())
                .await.map_err(|err| err.into())
        }
        _ => todo!("Finish checking schemes")
    }
}
