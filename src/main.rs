use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

#[derive(Clone, Serialize)]
struct Todo {
    id: u64,
    title: String,
    description: Option<String>,
    completed: bool,
    deleted: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct CompletionUpdate {
    completed: bool,
}

type TodoStore = Arc<RwLock<Vec<Todo>>>;
type ApiError = (StatusCode, Json<ErrorResponse>);

#[tokio::main]
async fn main() {
    let store: TodoStore = Arc::new(RwLock::new(Vec::new()));

    let app = Router::new()
        .route("/health", get(health))
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .route("/todos/{id}/complete", axum::routing::patch(set_completion))
        .fallback_service(ServeDir::new("public"))
        .with_state(store);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server running at http://{address}");

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "healthy" })
}

async fn create_todo(
    State(store): State<TodoStore>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    let title = payload.title.trim().to_string();

    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Title cannot be empty",
            }),
        ));
    }

    let mut todos = store.write().await;

    let todo = Todo {
        id: todos.len() as u64 + 1,
        title,
        description: normalize_description(payload.description),
        completed: false,
        deleted: false,
    };

    todos.push(todo.clone());

    Ok((StatusCode::CREATED, Json(todo)))
}

async fn list_todos(State(store): State<TodoStore>) -> Json<Vec<Todo>> {
    let todos = store.read().await;

    let active_todos = todos.iter().filter(|todo| !todo.deleted).cloned().collect();

    Json(active_todos)
}

async fn get_todo(
    State(store): State<TodoStore>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, ApiError> {
    let todos = store.read().await;

    let todo = todos
        .iter()
        .find(|todo| todo.id == id && !todo.deleted)
        .cloned();

    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(todo_not_found()),
    }
}

async fn update_todo(
    State(store): State<TodoStore>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    let title = payload.title.trim().to_string();

    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Title cannot be empty",
            }),
        ));
    }

    let mut todos = store.write().await;

    let todo = todos
        .iter_mut()
        .find(|todo| todo.id == id && !todo.deleted)
        .ok_or_else(todo_not_found)?;

    todo.title = title;
    todo.description = normalize_description(payload.description);

    Ok(Json(todo.clone()))
}

async fn set_completion(
    State(store): State<TodoStore>,
    Path(id): Path<u64>,
    Json(payload): Json<CompletionUpdate>,
) -> Result<Json<Todo>, ApiError> {
    let mut todos = store.write().await;

    let todo = todos
        .iter_mut()
        .find(|todo| todo.id == id && !todo.deleted)
        .ok_or_else(todo_not_found)?;

    todo.completed = payload.completed;

    Ok(Json(todo.clone()))
}

async fn delete_todo(
    State(store): State<TodoStore>,
    Path(id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    let mut todos = store.write().await;

    let todo = todos
        .iter_mut()
        .find(|todo| todo.id == id && !todo.deleted)
        .ok_or_else(todo_not_found)?;

    todo.deleted = true;

    Ok(StatusCode::NO_CONTENT)
}

fn todo_not_found() -> ApiError {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "Todo not found",
        }),
    )
}

fn normalize_description(description: Option<String>) -> Option<String> {
    description.and_then(|description| {
        let description = description.trim();

        if description.is_empty() {
            None
        } else {
            Some(description.to_string())
        }
    })
}
