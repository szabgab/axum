use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(main_page))
        .route("/echo", get(echo))
}

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
    <form method="get" action="/echo">
    <input type="text" name="text">
    <input type="submit" value="Echo">
    </form>
    "#,
    )
}

async fn echo(Query(params): Query<Params>) -> Html<String> {
    println!("params: {:?}", params);
    Html(format!(r#"You said: <b>{}</b>"#, params.text))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    text: String,
}

#[cfg(test)]
mod tests;


