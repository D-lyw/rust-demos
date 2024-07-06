use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Json,
};
use nanoid::nanoid;
use serde::Deserialize;
use tracing::info;

use crate::{error::AppError, models::Url, AppState, ServiceResponse, SERVICE_ADDR};

pub(crate) async fn redirect_url(
    State(state): State<AppState>,
    Path(short_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!("Resolve short url {}", short_id);
    let url_record = Url::find_url(&short_id, &state.pool).await?;
    match url_record {
        Some(record) => Ok(Redirect::permanent(&record.url)),
        None => Err(AppError::NotFoundRecord(short_id)),
    }
}

#[derive(Debug, Deserialize)]
pub struct ShortParam {
    url: String,
}

pub(crate) async fn shorten_url(
    State(state): State<AppState>,
    Json(params): Json<ShortParam>,
) -> Result<impl IntoResponse, AppError> {
    info!("Shorten url: {:?}", params.url);
    let shorted_url = nanoid!(8);
    let _record = Url::create(&params.url, &shorted_url, &state.pool).await?;

    let shorted_url = format!("http://{}/{}", SERVICE_ADDR, shorted_url);

    // way one
    // Json(json!({"status": true, "message": "success", "data": shorted_url}))

    // way two
    Ok(Json(ServiceResponse {
        data: serde_json::Value::String(shorted_url),
        ..Default::default()
    }))
}
pub(crate) async fn service_ping() -> &'static str {
    "pong"
}
