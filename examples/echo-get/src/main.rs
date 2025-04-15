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
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_main_page() {
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(html.contains(r#"<form method="get" action="/echo">"#));
    }

    #[tokio::test]
    async fn test_echo_with_data() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/echo?text=Hello+World!")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(html, "You said: <b>Hello World!</b>");
    }

    #[tokio::test]
    async fn test_echo_without_data() {
        let response = app()
            .oneshot(Request::builder().uri("/echo").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST); // 400
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(
            html,
            "Failed to deserialize query string: missing field `text`"
        );
    }

    #[tokio::test]
    async fn test_echo_missing_value() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/echo?text=")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(html, "You said: <b></b>");
    }

    #[tokio::test]
    async fn test_echo_extra_param() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/echo?text=Hello&extra=123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(html, "You said: <b>Hello</b>");
    }
}
