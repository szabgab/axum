//! Run with
//!
//! ```not_rust
//! cargo run -p example-templates-minijinja
//! ```
//! Demo for the MiniJinja templating engine.
//! Exposes three pages all sharing the same layout with a minimal nav menu.

use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::Html, routing::get, Router};
use minijinja::{context, Environment};
use std::sync::Arc;

struct AppState {
    env: Environment<'static>,
}

#[tokio::main]
async fn main() {
    let app = app();
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    // init template engine and add templates
    let mut env = Environment::new();
    env.add_template("layout", include_str!("../templates/layout.jinja"))
        .unwrap();
    env.add_template("home", include_str!("../templates/home.jinja"))
        .unwrap();
    env.add_template("content", include_str!("../templates/content.jinja"))
        .unwrap();
    env.add_template("about", include_str!("../templates/about.jinja"))
        .unwrap();

    // pass env to handlers via state
    let app_state = Arc::new(AppState { env });

    // define routes
    Router::new()
        .route("/", get(handler_home))
        .route("/content", get(handler_content))
        .route("/about", get(handler_about))
        .with_state(app_state)
}

async fn handler_home(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("home").unwrap();

    let rendered = template
        .render(context! {
            title => "Home",
            welcome_text => "Hello World!",
        })
        .unwrap();

    Ok(Html(rendered))
}

async fn handler_content(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("content").unwrap();

    let some_example_entries = vec!["Data 1", "Data 2", "Data 3"];

    let rendered = template
        .render(context! {
            title => "Content",
            entries => some_example_entries,
        })
        .unwrap();

    Ok(Html(rendered))
}

async fn handler_about(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("about").unwrap();

    let rendered = template.render(context!{
        title => "About",
        about_text => "Simple demonstration layout for an axum project with minijinja as templating engine.",
    }).unwrap();

    Ok(Html(rendered))
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

        check_layout(&html);
        assert!(html.contains("<title>Website Name | Home </title>"));
        assert!(html.contains("<h1>Home</h1>"));
        assert!(html.contains("<p>Hello World!</p>"));
    }

    #[tokio::test]
    async fn test_content_page() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/content")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        check_layout(&html);
        assert!(html.contains("<title>Website Name | Content </title>"));
        assert!(html.contains("<h1>Content</h1>"));
        assert!(html.contains("<li>Data 1</li>"));
    }

    #[tokio::test]
    async fn test_about_page() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/about")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        check_layout(&html);
        assert!(html.contains("<title>Website Name | About </title>"));
        assert!(html.contains("<h1>About</h1>"));
        assert!(html.contains("<p>Simple demonstration layout for an axum project with minijinja as templating engine.</p>"));
    }

    fn check_layout(html: &str) {
        assert!(html.contains(r#"<li><a href="/">Home</a></li>"#));
    }
}
