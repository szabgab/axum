use std::collections::HashMap;

use super::*;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde::Deserialize;
use tower::ServiceExt;

#[derive(Debug, Deserialize, PartialEq)]
struct BadResponse {
    message: String,
    origin: String,
}

#[tokio::test]
async fn test_with_rejection_bad_content_type() {
    let response = create_router()
        .oneshot(
            Request::post("/with-rejection")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    let body = response.into_body();
    let content = body.collect().await.unwrap().to_bytes();

    let message: BadResponse = serde_json::from_slice(&content).unwrap();
    assert_eq!(
        message,
        BadResponse {
            message: String::from("Expected request with `Content-Type: application/json`"),
            origin: String::from("with_rejection"),
        }
    )
}

#[tokio::test]
async fn test_with_rejection_good_content_type_no_value() {
    let response = create_router()
        .oneshot(
            Request::post("/with-rejection")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body();
    let content = body.collect().await.unwrap().to_bytes();

    let message: BadResponse = serde_json::from_slice(&content).unwrap();
    assert_eq!(
        message,
        BadResponse {
            message: String::from("Failed to parse the request body as JSON: EOF while parsing a value at line 1 column 0"),
            origin: String::from("with_rejection"),
        }
    )
}

#[tokio::test]
async fn test_with_rejection_good_content_type_with_value() {
    let response = create_router()
        .oneshot(
            Request::post("/with-rejection")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    r#"{"welcome": "Hello World!", "name": "Foo Bar"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let content = body.collect().await.unwrap().to_bytes();

    let data: HashMap<String, String> = serde_json::from_slice(&content).unwrap();
    assert_eq!(
        data,
        HashMap::from([
            (String::from("welcome"), String::from("Hello World!")),
            (String::from("name"), String::from("Foo Bar")),
        ])
    );
}
