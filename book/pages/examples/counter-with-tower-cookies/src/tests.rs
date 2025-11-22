use super::*;
use axum::{body::Body, http::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::{Service, ServiceExt};

// Actually we don't need the multi-step test here, but it's a good demonstration of how to do it
#[tokio::test]
async fn test_counter_multi_step() {
    let mut svc = app().into_service();

    let response = svc
        .call(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let cookie_header = response.headers().get("set-cookie").unwrap();
    let cookie_value = cookie_header.to_str().unwrap().to_string();
    assert_eq!(cookie_value, "visited=1");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(html.contains(r#"You've been here 0 times before."#));

    let response = svc
        .call(
            Request::builder()
                .uri("/")
                .header("cookie", cookie_value)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let cookie_header = response.headers().get("set-cookie").unwrap();
    let cookie_value = cookie_header.to_str().unwrap().to_string();
    assert_eq!(cookie_value, "visited=2");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(r#"You've been here 1 times before."#));
}

#[tokio::test]
async fn without_cookie() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let cookie_header = response.headers().get("set-cookie").unwrap();
    let cookie_value = cookie_header.to_str().unwrap().to_string();
    assert_eq!(cookie_value, "visited=1");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(r#"You've been here 0 times before."#));
}

#[tokio::test]
async fn with_cookie() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .header("cookie", "visited=41")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let cookie_header = response.headers().get("set-cookie").unwrap();
    let cookie_value = cookie_header.to_str().unwrap().to_string();
    assert_eq!(cookie_value, "visited=42");

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert!(html.contains(r#"You've been here 41 times before."#));
}

#[tokio::test]
async fn remove_cookie() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/remove")
                .header("cookie", "visited=23")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let cookie_header = response.headers().get("set-cookie").unwrap();
    let cookie_value = cookie_header.to_str().unwrap().to_string();
    assert!(cookie_value.starts_with("visited=;"));

    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(html, r#"Counter has been reset. <a href="/">Go back</a>"#);
}
