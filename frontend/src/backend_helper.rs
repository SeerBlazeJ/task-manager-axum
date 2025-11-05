use chrono::NaiveDateTime;
use reqwest::{get, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub due_by: NaiveDateTime,
    pub imp_lvl: u8,
    pub req_time: String,
    pub is_done: bool,
}

pub async fn get_todos() -> Vec<Todo> {
    get("http://localhost:3000/get_todos")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn add_todo(
    name: String,
    description: String,
    due_by: String,
    req_time: String,
    imp_lvl: String,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let body = json!(Todo {
        id: None,
        name,
        description,
        due_by: convert_to_datetime(&due_by),
        req_time,
        imp_lvl: imp_lvl.parse::<u8>()?,
        is_done: false
    });
    client
        .post("http://localhost:3000/add_todo")
        .json(&body)
        .send()
        .await?;
    Ok(())
}

pub async fn mark_done(id: String) {
    let client = Client::new();
    let body = json!(id);
    client
        .post("http://localhost:3000/mark_done")
        .json(&body)
        .send()
        .await
        .unwrap();
}
pub async fn mark_undone(id: String) {
    let client = Client::new();
    let body = json!(id);
    client
        .post("http://localhost:3000/mark_undone")
        .json(&body)
        .send()
        .await
        .unwrap();
}

pub async fn delete_todo(id: String) {
    let client = Client::new();
    let body = json!(id);
    client
        .post("http://localhost:3000/delete")
        .json(&body)
        .send()
        .await
        .unwrap();
}

pub async fn get_day_todos(day: &String) -> Vec<Todo> {
    get(format!("http://localhost:3000/get_todos/{}", day))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

fn convert_to_datetime(dt: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M").expect("Failed to parse datetime")
}

pub async fn add_sched(
    title: String,
    start_date: String,
    end_date: String,
    imp: String,
    start_time: String,
    end_time: String,
    weekdays: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
