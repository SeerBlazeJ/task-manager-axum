use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Surreal, engine::local::Db};

// Separate struct for database with RecordId
#[derive(Clone, Serialize, Deserialize)]
pub struct TaskDB {
    id: Option<RecordId>,
    name: String,
    description: String,
    pub due_by: NaiveDateTime,
    imp_lvl: u8,
    req_time: NaiveTime,
    time_alloted: NaiveTime,
    is_done: bool,
}

// API struct with String ID for frontend
#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    id: Option<String>,
    pub name: String,
    description: String,
    pub due_by: NaiveDateTime,
    pub imp_lvl: u8,
    pub req_time: NaiveTime,
    pub time_alloted: NaiveTime,
    pub is_done: bool,
}

// Conversions
impl From<TaskDB> for Task {
    fn from(db: TaskDB) -> Self {
        Self {
            id: db.id.map(|rid| rid.to_string()),
            name: db.name,
            description: db.description,
            due_by: db.due_by,
            imp_lvl: db.imp_lvl,
            req_time: db.req_time,
            time_alloted: db.time_alloted,
            is_done: db.is_done,
        }
    }
}

impl From<Task> for TaskDB {
    fn from(api: Task) -> Self {
        Self {
            id: api.id.and_then(|s| s.parse().ok()),
            name: api.name,
            description: api.description,
            due_by: api.due_by,
            imp_lvl: api.imp_lvl,
            req_time: api.req_time,
            time_alloted: api.time_alloted,
            is_done: api.is_done,
        }
    }
}

pub async fn get_task(State(conn): State<Surreal<Db>>) -> impl IntoResponse {
    conn.use_ns("core").use_db("main").await.unwrap();
    let values: Vec<TaskDB> = conn.select("Tasks").await.unwrap();
    let task: Vec<Task> = values.into_iter().map(Task::from).collect();
    Json(task)
}

pub async fn add_task(
    State(conn): State<Surreal<Db>>,
    Json(new_task): Json<Task>,
) -> impl IntoResponse {
    conn.use_ns("core").use_db("main").await.unwrap();
    let db_task = TaskDB::from(new_task);
    let _: Option<TaskDB> = conn.create("Tasks").content(db_task).await.unwrap();
    StatusCode::CREATED
}

pub async fn mark_done(
    State(conn): State<Surreal<Db>>,
    Json(id): Json<String>,
) -> impl IntoResponse {
    conn.use_ns("core").use_db("main").await.unwrap();
    let record_id: RecordId = id.parse().unwrap();
    let _: Option<TaskDB> = conn
        .update(record_id)
        .merge(serde_json::json!({"is_done": true}))
        .await
        .unwrap();
    StatusCode::ACCEPTED
}

pub async fn mark_undone(
    State(conn): State<Surreal<Db>>,
    Json(id): Json<String>,
) -> impl IntoResponse {
    conn.use_ns("core").use_db("main").await.unwrap();
    let record_id: RecordId = id.parse().unwrap();
    let _: Option<TaskDB> = conn
        .update(record_id)
        .merge(serde_json::json!({"is_done": false}))
        .await
        .unwrap();
    StatusCode::ACCEPTED
}

pub async fn delete_task(
    State(conn): State<Surreal<Db>>,
    Json(id): Json<String>,
) -> impl IntoResponse {
    conn.use_ns("core").use_db("main").await.unwrap();
    let record_id: RecordId = id.parse().unwrap();
    let _: Option<TaskDB> = conn.delete(record_id).await.unwrap();
    StatusCode::ACCEPTED
}

pub async fn get_task_by_id(
    State(conn): State<Surreal<Db>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    conn.use_ns("core").use_db("main").await.unwrap();
    let (table, key) = id.split_once(':').unwrap_or(("Tasks", id.as_str()));
    let task_db: Option<TaskDB> = conn.select((table, key)).await.unwrap();
    let task_frontend: Option<Task> = match task_db {
        Some(task) => Some(Task::from(task)),
        None => None,
    };
    Json(task_frontend)
}
