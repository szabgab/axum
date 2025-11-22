use super::*;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
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

    assert!(html.contains(r#"<form method="post" action="/echo">"#));
}

#[tokio::test]
async fn test_echo_with_data() {
    let response = app()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/echo")
                .header(
                    http::header::CONTENT_TYPE,
                    mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                )
                .body(Body::from("text=Hello+World!"))
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
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/echo")
                .header(
                    http::header::CONTENT_TYPE,
                    mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY); // 422
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(
        html,
        "Failed to deserialize form body: missing field `text`"
    );
}

#[tokio::test]
async fn test_echo_missing_value() {
    let response = app()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/echo")
                .header(
                    http::header::CONTENT_TYPE,
                    mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                )
                .body(Body::from("text="))
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
                .method(http::Method::POST)
                .uri("/echo")
                .header(
                    http::header::CONTENT_TYPE,
                    mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                )
                .body(Body::from("text=Hello&extra=123"))
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
