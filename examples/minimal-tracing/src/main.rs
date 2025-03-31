//! Run with
//!
//! ```not_rust
//! cargo run -p example-minimal-tracing
//! ```

use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // minimal setup for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    tracing::debug!("in handler");
    Html("<h1>Hello, World!</h1>")
}
