use anyhow::Result;
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{debug, event, info, instrument, Level};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() -> Result<()> {
    let log_appender = tracing_appender::rolling::hourly(".", "log").with_max_level(Level::TRACE);
    tracing_subscriber::fmt().with_writer(log_appender).init();

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/query", get(query_handler))
        .nest_service("/static", ServeDir::new("data"))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument]
async fn root_handler() -> Html<&'static str> {
    info!("Hanlde request root handler");
    debug!("hello! I'm gonna shave a yak.");
    event!(Level::INFO, "inside my_function!");
    Html(r#"<h1>Hello, world!</h1>"#)
}

#[derive(Deserialize, Debug)]
struct ParamsStruct {
    name: String,
    age: usize,
    address: Option<String>,
}

async fn query_handler(Query(params): Query<ParamsStruct>) -> impl IntoResponse {
    info!("{:?}", params);
    Html("<div>Query handler</div>")
}
