use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{fs, path::Path, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct Todo {
    id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    completed: bool,
    priority: String,
    category: String,
    #[serde(rename = "dueDate", skip_serializing_if = "Option::is_none")]
    due_date: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(skip)]
    position: i64,
}

#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "priority")]
    priority: Option<String>,
    #[serde(rename = "category")]
    category: Option<String>,
    #[serde(rename = "dueDate")]
    due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTodoRequest {
    #[serde(rename = "title")]
    title: Option<String>,
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "completed")]
    completed: Option<bool>,
    #[serde(rename = "priority")]
    priority: Option<String>,
    #[serde(rename = "category")]
    category: Option<String>,
    #[serde(rename = "dueDate")]
    due_date: Option<Option<String>>,
}

#[derive(Debug, Deserialize)]
struct ReorderRequest {
    #[serde(rename = "orderedIds")]
    ordered_ids: Vec<String>,
}

struct AppState {
    db: SqlitePool,
}

async fn init_database(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            completed BOOLEAN NOT NULL DEFAULT 0,
            priority TEXT NOT NULL,
            category TEXT NOT NULL,
            due_date TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            position INTEGER NOT NULL DEFAULT 0
        )",
    )
    .execute(pool)
    .await
    .expect("Failed to create todos table");

    // Check if table is empty, if so, seed data
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM todos")
        .fetch_one(pool)
        .await
        .unwrap();

    if count.0 == 0 {
        let now = Utc::now().to_rfc3339();
        let tomorrow = (Utc::now() + chrono::Duration::days(1))
            .format("%Y-%m-%d")
            .to_string();

        let initial = vec![
            (
                "1",
                "Welcome to your collaborative Todo List!",
                Some("This app is fully connected to an Axum backend. Any changes you make are saved in real-time across devices."),
                false,
                "high",
                "Personal",
                Some(tomorrow),
                &now,
                &now,
                1,
            ),
            (
                "2",
                "Explore filtering and sorting capabilities",
                Some("You can filter tasks by Category (Work, Personal, Shopping, etc.), search by text, or filter by Priority."),
                false,
                "medium",
                "Work",
                None,
                &now,
                &now,
                2,
            ),
            (
                "3",
                "Try out real-time sync",
                Some("Open this app in multiple browser tabs. Notice the sync indicator and watch how changes propagate smoothly."),
                true,
                "low",
                "Ideas",
                None,
                &now,
                &now,
                3,
            ),
        ];

        for (id, title, desc, completed, priority, category, due, created, updated, pos) in initial
        {
            sqlx::query(
                "INSERT INTO todos (id, title, description, completed, priority, category, due_date, created_at, updated_at, position)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(id)
            .bind(title)
            .bind(desc)
            .bind(completed)
            .bind(priority)
            .bind(category)
            .bind(due)
            .bind(created)
            .bind(updated)
            .bind(pos)
            .execute(pool)
            .await
            .unwrap();
        }
    }
}

async fn get_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY position ASC")
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(todos))
}

async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    if payload.title.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Title is required".to_string()));
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let title = payload.title.trim().to_string();
    let description = payload
        .description
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty());
    let priority = payload.priority.unwrap_or_else(|| "medium".to_string());
    let category = payload.category.unwrap_or_else(|| "General".to_string());
    let due_date = payload.due_date;

    let (min_pos,): (Option<i64>,) = sqlx::query_as("SELECT MIN(position) FROM todos")
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let position = min_pos.unwrap_or(0) - 1;

    let new_todo = Todo {
        id: id.clone(),
        title,
        description,
        completed: false,
        priority,
        category,
        due_date,
        created_at: now.clone(),
        updated_at: now,
        position,
    };

    sqlx::query(
        "INSERT INTO todos (id, title, description, completed, priority, category, due_date, created_at, updated_at, position)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&new_todo.id)
    .bind(&new_todo.title)
    .bind(&new_todo.description)
    .bind(new_todo.completed)
    .bind(&new_todo.priority)
    .bind(&new_todo.category)
    .bind(&new_todo.due_date)
    .bind(&new_todo.created_at)
    .bind(&new_todo.updated_at)
    .bind(new_todo.position)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(new_todo)))
}

async fn update_todo(
    State(state): State<Arc<AppState>>,
    AxumPath(id): AxumPath<String>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let existing = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ?")
        .bind(&id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let existing = match existing {
        Some(t) => t,
        None => return Err((StatusCode::NOT_FOUND, "Todo not found".to_string())),
    };

    let now = Utc::now().to_rfc3339();

    let updated = Todo {
        id: existing.id.clone(),
        title: payload
            .title
            .map(|t| t.trim().to_string())
            .unwrap_or_else(|| existing.title.clone()),
        description: match payload.description {
            Some(d) => {
                let trimmed = d.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            }
            None => existing.description.clone(),
        },
        completed: payload.completed.unwrap_or(existing.completed),
        priority: payload
            .priority
            .unwrap_or_else(|| existing.priority.clone()),
        category: payload
            .category
            .map(|c| c.trim().to_string())
            .unwrap_or_else(|| existing.category.clone()),
        due_date: payload
            .due_date
            .unwrap_or_else(|| existing.due_date.clone()),
        created_at: existing.created_at.clone(),
        updated_at: now,
        position: existing.position,
    };

    sqlx::query(
        "UPDATE todos SET title = ?, description = ?, completed = ?, priority = ?, category = ?, due_date = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&updated.title)
    .bind(&updated.description)
    .bind(updated.completed)
    .bind(&updated.priority)
    .bind(&updated.category)
    .bind(&updated.due_date)
    .bind(&updated.updated_at)
    .bind(&updated.id)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated))
}

async fn delete_todo_handler(
    State(state): State<Arc<AppState>>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Todo not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "success": true, "id": id })))
}

async fn clear_completed(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM todos WHERE completed = 1")
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(
        serde_json::json!({ "success": true, "clearedCount": result.rows_affected() }),
    ))
}

async fn reorder_todos(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ReorderRequest>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for (index, id) in payload.ordered_ids.iter().enumerate() {
        sqlx::query("UPDATE todos SET position = ? WHERE id = ?")
            .bind(index as i64)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY position ASC")
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(todos))
}

#[tokio::main]
async fn main() {
    let db_dir = Path::new("data");
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).ok();
    }

    let connection_options = SqliteConnectOptions::new()
        .filename("data/todos.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(connection_options)
        .await
        .expect("Failed to connect to database");

    init_database(&pool).await;

    let state = Arc::new(AppState { db: pool });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .route("/api/todos", get(get_todos).post(create_todo))
        .route("/api/todos/clear-completed", post(clear_completed))
        .route("/api/todos/reorder", post(reorder_todos))
        .route(
            "/api/todos/{id}",
            put(update_todo).delete(delete_todo_handler),
        );

    let app = Router::new()
        .merge(api_routes)
        .fallback_service(
            tower_http::services::ServeDir::new("dist")
                .fallback(tower_http::services::ServeFile::new("dist/index.html")),
        )
        .layer(cors)
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running on http://localhost:{}", port);
    axum::serve(listener, app).await.unwrap();
}
