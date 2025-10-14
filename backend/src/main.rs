use std::str::FromStr;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    serve,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use surrealdb::{
    RecordId, Surreal,
    engine::local::{Db, RocksDb},
};
use tokio::net::TcpListener;

// Separate struct for database with RecordId
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoDB {
    id: Option<RecordId>,
    name: String,
    description: String,
    due_by: NaiveDateTime,
    imp_lvl: u8,
    req_time: String,
    is_done: bool,
}

// API struct with String ID for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: Option<String>,
    name: String,
    description: String,
    due_by: NaiveDateTime,
    imp_lvl: u8,
    req_time: String,
    is_done: bool,
}

// Conversions
impl From<TodoDB> for Todo {
    fn from(db: TodoDB) -> Self {
        Self {
            id: db.id.map(|rid| rid.to_string()),
            name: db.name,
            description: db.description,
            due_by: db.due_by,
            imp_lvl: db.imp_lvl,
            req_time: db.req_time,
            is_done: db.is_done,
        }
    }
}

impl From<Todo> for TodoDB {
    fn from(api: Todo) -> Self {
        Self {
            id: api.id.and_then(|s| s.parse().ok()),
            name: api.name,
            description: api.description,
            due_by: api.due_by,
            imp_lvl: api.imp_lvl,
            req_time: api.req_time,
            is_done: api.is_done,
        }
    }
}

#[tokio::main]
async fn main() {
    let db_conn = Surreal::new::<RocksDb>("TodosApp").await.unwrap();
    db_conn.use_ns("core").use_db("todos").await.unwrap();

    let router = Router::new()
        .route("/get_todos", get(get_todos))
        .route("/add_todo", post(add_todo))
        .route("/mark_done", post(mark_done))
        .route("/mark_undone", post(mark_undone))
        .route("/delete", post(delete_todo))
        .route("/get_todos/{day_str}", get(get_todos_by_day))
        .with_state(db_conn)
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = TcpListener::bind("localhost:3000")
        .await
        .expect("Couldn't connect to port 3000");
    serve(addr, router).await.unwrap()
}

async fn get_todos(State(conn): State<Surreal<Db>>) -> impl IntoResponse {
    let values: Vec<TodoDB> = conn.select("todos").await.unwrap();
    let todos: Vec<Todo> = values.into_iter().map(Todo::from).collect();
    Json(todos)
}

async fn add_todo(
    State(conn): State<Surreal<Db>>,
    Json(new_task): Json<Todo>,
) -> impl IntoResponse {
    let db_task = TodoDB::from(new_task);
    let _created_task: Option<TodoDB> = conn.create("todos").content(db_task).await.unwrap();
    StatusCode::CREATED
}

async fn mark_done(State(conn): State<Surreal<Db>>, Json(id): Json<String>) -> impl IntoResponse {
    let record_id: RecordId = id.parse().unwrap();
    let _: Option<TodoDB> = conn
        .update(record_id)
        .merge(serde_json::json!({"is_done": true}))
        .await
        .unwrap();
    StatusCode::ACCEPTED
}

async fn mark_undone(State(conn): State<Surreal<Db>>, Json(id): Json<String>) -> impl IntoResponse {
    let record_id: RecordId = id.parse().unwrap();
    let _: Option<TodoDB> = conn
        .update(record_id)
        .merge(serde_json::json!({"is_done": false}))
        .await
        .unwrap();
    StatusCode::ACCEPTED
}

async fn delete_todo(State(conn): State<Surreal<Db>>, Json(id): Json<String>) -> impl IntoResponse {
    let record_id: RecordId = id.parse().unwrap();
    let _: Option<TodoDB> = conn.delete(record_id).await.unwrap();
    StatusCode::ACCEPTED
}

async fn get_todos_by_day(
    State(conn): State<Surreal<Db>>,
    Path(day_str): Path<String>,
) -> impl IntoResponse {
    let date = NaiveDate::from_str(&day_str).unwrap();
    let all_recs: Vec<TodoDB> = conn.select("todos").await.unwrap();
    let req_recs: Vec<Todo> = all_recs
        .into_iter()
        .filter(|todo| todo.due_by.date() == date)
        .map(Todo::from)
        .collect();
    Json(req_recs)
}
