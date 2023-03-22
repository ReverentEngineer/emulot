use axum::{
    Extension,
    http::StatusCode,
    Json,
    extract::Path,
    Router,
    routing::{post, delete, get}
};
use crate::{
    error::Error,
    daemon::State,
    storage::Labeled,
    GuestConfig
};

pub fn router() -> Router
{
        Router::new()
            .route("/create/:name", post(create))
            .route("/remove/:name", delete(remove))
            .route("/list", get(list))
            .route("/start/:name", post(run))
            .route("/shutdown/:name", post(shutdown))
}

async fn list(
    state: Extension<State>
    ) -> Result<Json<Vec<Labeled<isize>>>, Error>
{
    Ok(Json(state.storage.list(None, None)?))
}

async fn create(
    Path(name): Path<String>,
    state: Extension<State>, 
    Json(config): Json<GuestConfig>
    ) -> Result<StatusCode, Error>
{
    state.storage.insert(&name, config)?;
    Ok(StatusCode::OK)
}

async fn remove(
    Path(id): Path<isize>,
    state: Extension<State>) -> Result<StatusCode, Error>
{
    state.storage.remove(id)?;
    Ok(StatusCode::OK)
}

async fn run(
    Path(id): Path<usize>,
    state: Extension<State>) -> Result<StatusCode, Error>
{
    state.orchestrator.run(id).await?;
    Ok(StatusCode::OK)
}

async fn shutdown(
    Path(name): Path<String>,
    state: Extension<State>) -> Result<StatusCode, Error>
{
    state.orchestrator.shutdown(&name).await?;
    Ok(StatusCode::OK)
}
