use super::*;
use axum::{body::Body, http::Request, http::StatusCode};
use headers::ContentType;
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_main_page() {
    let response = app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type.to_str().unwrap(), "text/plain; charset=utf-8");

    let content_type = response
        .headers()
        .get("content-type")
        .map(|header| header.to_str().unwrap().parse::<ContentType>().unwrap());
    assert_eq!(content_type, Some(ContentType::text_utf8()));

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, "<h1>Hello, World!</h1>");
}
