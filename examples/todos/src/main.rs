//! Provides a RESTful web server managing some Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a JSON list of Todos.
//! - `POST /todos`: create a new Todo.
//! - `PATCH /todos/{id}`: update a specific Todo.
//! - `DELETE /todos/{id}`: delete a specific Todo.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-todos
//! ```

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Compose the routes
    let app = app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    let db = Db::default();

    Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/{id}", patch(todos_update).delete(todos_delete))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db)
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

async fn todos_index(pagination: Query<Pagination>, State(db): State<Db>) -> impl IntoResponse {
    let todos = db.read().unwrap();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

async fn todos_create(State(db): State<Db>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

async fn todos_update(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        todo.completed = completed;
    }

    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}

async fn todos_delete(Path(id): Path<Uuid>, State(db): State<Db>) -> impl IntoResponse {
    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        routing::RouterIntoService,
    };
    use http_body_util::BodyExt;
    use serde_json::json;
    use tower::{Service, ServiceExt};

    #[tokio::test]
    async fn test_empty_list_of_todos() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/todos")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();
        let todos = serde_json::from_str::<Vec<Todo>>(&html).unwrap();

        assert_eq!(todos, []);
    }

    #[tokio::test]
    async fn test_add_todo() {
        let mut app = app().into_service();
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/todos")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json!({"text": "Write more tests!"}).to_string()))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        let todo = serde_json::from_str::<Todo>(&html).unwrap();
        assert_eq!(todo.text, "Write more tests!");
        assert!(!todo.completed);

        let todos = get_todos(&mut app).await;
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0], todo);
    }

    #[tokio::test]
    async fn test_complex() {
        let mut app = app().into_service();
        let todos = get_todos(&mut app).await;
        assert_eq!(todos.len(), 0);

        let mut todo0 = add_todo(&mut app, "Add more tests!").await;
        let todo1 = add_todo(&mut app, "Some other thing to do.").await;
        let todo2 = add_todo(&mut app, "Write a book about axum.").await;

        let mut todos = get_todos(&mut app).await;
        assert_eq!(todos.len(), 3);

        // Ensure the order is correct for the tests
        todos.sort_by_key(|todo| todo.text.clone());

        assert_eq!(todos[0], todo0);
        assert_eq!(todos[1], todo1);
        assert_eq!(todos[2], todo2);

        let (status, res) = update_todo(&mut app, todo0.id, &todo0.text, true).await;
        assert_eq!(status, StatusCode::OK);
        todo0.completed = true;
        assert_eq!(res, Some(todo0.clone()));

        let status = delete_todo(&mut app, todo1.id).await;
        assert_eq!(status, StatusCode::NO_CONTENT);

        let mut todos = get_todos(&mut app).await;
        assert_eq!(todos.len(), 2);

        // Ensure the order is correct for the tests
        todos.sort_by_key(|todo| todo.text.clone());

        assert_eq!(todos[0], todo0);
        assert_eq!(todos[1], todo2);

        let status = delete_todo(&mut app, todo1.id).await;
        assert_eq!(status, StatusCode::NOT_FOUND);

        let (status, res) = update_todo(&mut app, todo1.id, "", true).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(res, None);
    }

    async fn get_todos(app: &mut RouterIntoService<Body>) -> Vec<Todo> {
        let request = Request::builder()
            .uri("/todos")
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();
        serde_json::from_str::<Vec<Todo>>(&html).unwrap()
    }

    async fn add_todo(app: &mut RouterIntoService<Body>, text: &str) -> Todo {
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/todos")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json!({"text": text}).to_string()))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        serde_json::from_str::<Todo>(&html).unwrap()
    }

    async fn update_todo(
        app: &mut RouterIntoService<Body>,
        id: Uuid,
        text: &str,
        completed: bool,
    ) -> (StatusCode, Option<Todo>) {
        let request = Request::builder()
            .method(http::Method::PATCH)
            .uri(format!("/todos/{id}"))
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({"text": text, "completed": completed}).to_string(),
            ))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        let status = response.status();
        if status != StatusCode::OK {
            return (status, None);
        }
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        (status, Some(serde_json::from_str::<Todo>(&html).unwrap()))
    }

    async fn delete_todo(app: &mut RouterIntoService<Body>, id: Uuid) -> StatusCode {
        let request = Request::builder()
            .method(http::Method::DELETE)
            .uri(format!("/todos/{id}"))
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        response.status()
    }
}
