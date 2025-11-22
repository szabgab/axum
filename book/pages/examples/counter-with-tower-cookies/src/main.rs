use axum::{Router, response::Html, routing::get};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

const COOKIE_NAME: &str = "visited";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(increment))
        .route("/remove", get(remove))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn increment(cookies: Cookies) -> Html<String> {
    let visited = cookies
        .get(COOKIE_NAME)
        .and_then(|c| c.value().parse().ok())
        .unwrap_or(0);
    cookies.add(Cookie::new(COOKIE_NAME, (visited + 1).to_string()));
    Html(format!(
        r#"You've been here {visited} times before.
    Reload the page to increment the counter or click here to remove the cookie and <a href="/remove">reset the counter</a>."#
    ))
}

async fn remove(cookies: Cookies) -> Html<&'static str> {
    cookies.remove(Cookie::new(COOKIE_NAME, ""));
    Html(r#"Counter has been reset. <a href="/">Go back</a>"#.into())
}
