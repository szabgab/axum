//! Run with
//!
//! ```not_rust
//! cargo run -p example-redirect
//! ```

use axum::{
    response::{Html, Redirect},
    routing::get,
    Router,
};

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
        .route("/try", get(try_page))
        .route("/land", get(land_page))
}

async fn main_page() -> Html<&'static str> {
    Html(
        r#"
        <h1>Redirect</h1>
        <a href="/try">Try</a><br>
        <a href="/land">Land</a>
    "#,
    )
}

async fn try_page() -> Redirect {
    Redirect::temporary("/land")
}

async fn land_page() -> Html<&'static str> {
    Html("Landed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_main() {
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(html.contains("<h1>Redirect</h1>"));
    }

    #[tokio::test]
    async fn test_landed() {
        let response = app()
            .oneshot(Request::builder().uri("/land").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(html, "Landed");
    }

    #[tokio::test]
    async fn test_try() {
        let response = app()
            .oneshot(Request::builder().uri("/try").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        let location = response.headers().get("location").unwrap();
        assert_eq!(location, "/land");
    }
}
