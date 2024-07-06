mod models;
mod error;
mod handlers;

use dotenv::dotenv;
use std::{env, ops::Deref, sync::Arc};

use axum::{routing::{get, post}, Router};
use error::AppError;
use handlers::*;

pub const SERVICE_ADDR: &str = "127.0.0.1:3001";

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &AppStateInner {
        &self.inner
    }
}

#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub pool: sqlx::PgPool,
    // ...
}

impl AppState {
    pub async fn new() -> anyhow::Result<AppState, AppError> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").map_err(|_| AppError::UnknownError )?;
        // let db_url = env::var("DATABASE_URL")?;

        let pool = sqlx::PgPool::connect(&db_url).await?;

        Ok(AppState {
            inner: Arc::new(AppStateInner { pool })
        })
    }
}

pub async fn get_router() -> anyhow::Result<Router, AppError> {
    let pool = AppState::new().await?;

    let app = Router::new()
    .route("/ping", get(service_ping))
    .route("/shorten", post(shorten_url))
    .route("/:id", get(redirect_url))
    .with_state(pool);

    Ok(app)
}