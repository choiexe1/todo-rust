use axum::{extract::{Path, State}, response::Redirect, routing::{get, post}, Form, Json, Router};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/delete/:id", post(delete))
        .route("/update", get(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    Ok(axum::serve(listener, app).await.unwrap())
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct NewTodo {
    description: String,
}

async fn list(State(pool): State<SqlitePool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await.unwrap();

    Json(todos)
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Redirect {
    sqlx::query!("INSERT INTO todos (description) VALUES (?)", todo.description).execute(&pool).await.unwrap();

    Redirect::to(env!("FRONTEND_URL"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Redirect {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await.unwrap();

    Redirect::to(env!("FRONTEND_URL"))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Redirect {
    sqlx::query!("UPDATE todos SET description = ?, done = ? WHERE id = ?", todo.description, todo.done, todo.id).execute(&pool).await.unwrap();

    Redirect::to(env!("FRONTEND_URL"))
}