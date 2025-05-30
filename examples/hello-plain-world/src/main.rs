use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = app();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> &'static str {
    "<h1>Hello, World!</h1>"
}

#[cfg(test)]
mod tests;
