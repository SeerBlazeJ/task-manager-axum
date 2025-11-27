use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use reqwest::{get, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub due_by: NaiveDateTime,
    pub imp_lvl: u8,
    pub req_time: NaiveTime,
    pub time_alloted: NaiveTime,
    pub is_done: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Routine {
    pub id: Option<String>,
    pub title: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub imp: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub weekdays: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SchedItem {
    pub represented_hour_start: u8,
    pub has_time: bool,
    pub time_left_mins: u8,
    pub title: String,
}

pub async fn get_todos() -> Vec<Task> {
    get("http://localhost:3000/get_tasks")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn get_todo_by_id(id: String) -> Option<Task> {
    get(format!("http://localhost:3000/get_task/{}", id))
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
    let vec_time: Vec<&str> = req_time.split(":").collect();
    let hour = vec_time[0].parse::<u32>().unwrap();
    let min = vec_time[1].parse::<u32>().unwrap();
    let req_time = NaiveTime::from_hms_opt(hour, min, 00).unwrap();
    let body = json!(Task {
        id: None,
        name,
        description,
        due_by: convert_to_datetime(due_by),
        time_alloted: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        req_time,
        imp_lvl: imp_lvl.parse::<u8>()?,
        is_done: false
    });
    client
        .post("http://localhost:3000/add_task")
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

pub async fn get_day_schedule(day: &String) -> Vec<SchedItem> {
    get(format!("http://localhost:3000/get_schedule/{}", day))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub fn convert_to_datetime(dt: String) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&dt, "%Y-%m-%dT%H:%M").expect("Failed to parse datetime")
}

pub async fn add_sched(sched_item: Routine) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let body = json!(sched_item);
    client
        .post("http://localhost:3000/add_sched")
        .json(&body)
        .send()
        .await?;
    Ok(())
}
