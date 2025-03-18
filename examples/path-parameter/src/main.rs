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

        assert!(html.contains(r#"<a href="/user/foo">/user/foo</a><br>"#));
    }

    #[tokio::test]
    async fn test_user_page() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/user/qqrq")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(html, "Hello, qqrq!");
    }
}
