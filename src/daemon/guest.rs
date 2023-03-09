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
            .route("/run/:name", post(run))
            .route("/stop/:name", post(stop))
}

async fn list(
    state: Extension<State>
    ) -> Result<Json<Vec<Labeled<GuestConfig>>>, Error>
{
    Ok(Json(state.storage.list(None, None).await?))
}

async fn create(
    Path(name): Path<String>,
    state: Extension<State>, 
    Json(config): Json<GuestConfig>
    ) -> Result<StatusCode, Error>
{
    state.storage.insert(&name, config).await?;
    Ok(StatusCode::OK)
}

async fn remove(
    Path(name): Path<String>,
    state: Extension<State>) -> Result<StatusCode, Error>
{
    state.storage.remove(&name).await?;
    Ok(StatusCode::OK)
}

async fn run(
    Path(name): Path<String>,
    state: Extension<State>) -> Result<StatusCode, Error>
{
    state.orchestrator.run(&name).await?;
    Ok(StatusCode::OK)
}

async fn stop(
    Path(name): Path<String>,
    state: Extension<State>) -> Result<StatusCode, Error>
{
    state.orchestrator.stop(&name).await?;
    Ok(StatusCode::OK)
}
