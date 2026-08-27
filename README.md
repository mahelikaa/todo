# Rust Todo App

A Todo application built with Rust, Axum, and vanilla JavaScript.

## Features

- Create, view, update, and soft-delete todos
- Mark todos complete or incomplete
- Backend input validation
- Responsive Post-it-style interface
- JSON REST API
- In-memory, concurrency-safe storage

## Tech Stack

- Rust
- Axum
- Tokio
- Serde
- HTML, CSS, JavaScript

## Run Locally

```bash
git clone https://github.com/mahelikaa/todo.git
cd todo
cargo run
```

Open [http://127.0.0.1:3000](http://127.0.0.1:3000).

## API Routes

| Method | Route | Description |
|---|---|---|
| GET | `/health` | Check server health |
| GET | `/todos` | List active todos |
| GET | `/todos/{id}` | Get one todo |
| POST | `/todos` | Create a todo |
| PUT | `/todos/{id}` | Update a todo |
| PATCH | `/todos/{id}/complete` | Set completion status |
| DELETE | `/todos/{id}` | Soft-delete a todo |

## Live App

[https://rust-todo-app-ssc3.onrender.com](https://rust-todo-app-ssc3.onrender.com)

> [!NOTE]
> Todos are stored in memory and reset whenever the server restarts.
