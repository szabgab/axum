use axum::{extract::Path, response::Html, routing::get, Router};

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
    Router::new()
        .route("/", get(main_page))
        .route("/user/{name}", get(user_page))
}

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
    <a href="/user/foo">/user/foo</a><br>
    <a href="/user/bar">/user/bar</a><br>
    "#,
    )
}

async fn user_page(Path(name): Path<String>) -> Html<String> {
    println!("user: {}", name);
    Html(format!("Hello, {}!", name))
}

#[cfg(test)]
mod tests;
