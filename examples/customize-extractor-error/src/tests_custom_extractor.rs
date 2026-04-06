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
async fn test_custom_extractor_bad_content_type() {
    let response = create_router()
        .oneshot(
            Request::post("/custom-extractor")
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
            origin: String::from("custom_extractor"),
        }
    )
}

#[tokio::test]
async fn test_custom_extractor_good_content_type_no_value() {
    let response = create_router()
        .oneshot(
            Request::post("/custom-extractor")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                //                .body(Body::from(r#"{"value"="Hello World!"}"#))
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
            origin: String::from("custom_extractor"),
        }
    )
}

#[tokio::test]
async fn test_with_rejection_good_content_type_with_value() {
    let response = create_router()
        .oneshot(
            Request::post("/custom-extractor")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(r#"{"value": "Hello World!"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body();
    let content = body.collect().await.unwrap().to_bytes();

    // TODO: did the author of this example want to return an empty string here?
    assert_eq!(content, "");
}
