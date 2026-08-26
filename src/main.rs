use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
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

type TodoStore = Arc<RwLock<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    let store: TodoStore = Arc::new(RwLock::new(Vec::new()));

    let app = Router::new()
        .route("/health", get(health))
        .route("/todos", get(list_todos).post(create_todo))
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
) -> (StatusCode, Json<Todo>) {
    let mut todos = store.write().await;

    let todo = Todo {
        id: todos.len() as u64 + 1,
        title: payload.title,
        description: payload.description,
        completed: false,
        deleted: false,
    };

    todos.push(todo.clone());

    (StatusCode::CREATED, Json(todo))
}

async fn list_todos(State(store): State<TodoStore>) -> Json<Vec<Todo>> {
    let todos = store.read().await;

    let active_todos = todos.iter().filter(|todo| !todo.deleted).cloned().collect();

    Json(active_todos)
}
