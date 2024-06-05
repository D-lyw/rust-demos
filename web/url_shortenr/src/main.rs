use anyhow::Result;
use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::info;

const SERVICE_ADDR: &str = "127.0.0.1:3001";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let pool = PgPoolOptions::new().connect("postgres://default:e9yE7CAMaYIk@ep-shiny-wildflower-a1brpvkz.ap-southeast-1.aws.neon.tech:5432/verceldb?sslmode=require").await?;

    let app = Router::new()
        .route("/ping", get(service_ping))
        .route("/shorten", post(shorten_url))
        .route("/:id", get(redirect_url))
        .with_state(pool);

    let listener = TcpListener::bind(SERVICE_ADDR).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn redirect_url(Path(short_id): Path<String>) -> impl IntoResponse {
    // Json(short_id)

    info!("Resolve short url {}", short_id);
    Redirect::permanent("https://baidu.com")
}

#[derive(Debug, Deserialize)]
struct ShortParam {
    url: String,
}

#[derive(Debug, Serialize)]
struct ServiceResponse {
    success: bool,
    message: String,
    data: serde_json::Value,
}

impl Default for ServiceResponse {
    fn default() -> Self {
        Self {
            success: true,
            message: "success".to_string(),
            data: serde_json::Value::Null,
        }
    }
}

async fn shorten_url(Json(params): Json<ShortParam>) -> impl IntoResponse {
    info!("Shorten url: {:?}", params.url);
    let shorted_url = format!("http://{}/{}", SERVICE_ADDR, nanoid!(8));

    // way one
    // Json(json!({"status": true, "message": "success", "data": shorted_url}))

    // way two
    Json(ServiceResponse {
        data: serde_json::Value::String(shorted_url),
        ..Default::default()
    })
}
async fn service_ping() -> &'static str {
    "pong"
}
