use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use chrono::NaiveDate;
use std::str::FromStr;
use surrealdb::{Surreal, engine::local::Db};

use crate::task_helper::{Task, TaskDB};

pub async fn get_tasks_by_day(
    State(conn): State<Surreal<Db>>,
    Path(day_str): Path<String>,
) -> impl IntoResponse {
    conn.use_ns("core").use_db("taks").await.unwrap();
    let date = NaiveDate::from_str(&day_str).unwrap();
    let all_recs: Vec<TaskDB> = conn.select("taks").await.unwrap();
    let req_recs: Vec<Task> = all_recs
        .into_iter()
        .filter(|task| task.due_by.date() == date)
        .map(Task::from)
        .collect();
    Json(req_recs)
}

pub async fn add_schedule(State(conn): State<Surreal<Db>>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
