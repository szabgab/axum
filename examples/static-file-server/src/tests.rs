use super::*;
use axum::{body::Body, http::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn get_page(app: Router, path: &str) -> (StatusCode, String) {
    let response = app
        .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();

    (status, html)
}

#[tokio::test]
async fn test_using_serve_dir() {
    let index_html = include_str!("../assets/index.html");
    let script_js = include_str!("../assets/script.js");

    let (status, html) = get_page(using_serve_dir(), "/assets/index.html").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(html, index_html);

    let (status, html) = get_page(using_serve_dir(), "/assets/script.js").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(html, script_js);

    let (status, html) = get_page(using_serve_dir(), "/assets/").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(html, index_html);
}
