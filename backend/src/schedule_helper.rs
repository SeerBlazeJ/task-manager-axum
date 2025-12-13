use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Local, NaiveDate, NaiveTime, TimeDelta, Timelike};
use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Surreal, engine::local::Db};

use crate::task_helper::{Task, TaskDB};

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
pub struct RoutineDB {
    pub id: Option<RecordId>,
    pub title: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub imp: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub weekdays: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SchedItemDB {
    pub id: Option<RecordId>,
    pub date: NaiveDate,
    pub represented_hour_start: u8,
    pub has_time: bool,
    pub time_left_mins: u8,
    pub title: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SchedItem {
    pub id: Option<String>,
    pub date: NaiveDate,
    pub represented_hour_start: u8,
    pub has_time: bool,
    pub time_left_mins: u8,
    pub title: Vec<String>,
}

// Conversions
impl From<RoutineDB> for Routine {
    fn from(db_item: RoutineDB) -> Self {
        Self {
            id: db_item.id.map(|sid| sid.to_string()),
            title: db_item.title,
            start_date: db_item.start_date,
            end_date: db_item.end_date,
            imp: db_item.imp,
            start_time: db_item.start_time,
            end_time: db_item.end_time,
            weekdays: db_item.weekdays,
        }
    }
}

impl From<Routine> for RoutineDB {
    fn from(api: Routine) -> Self {
        Self {
            id: api.id.and_then(|s| s.parse().ok()),
            title: api.title,
            start_date: api.start_date,
            end_date: api.end_date,
            imp: api.imp,
            start_time: api.start_time,
            end_time: api.end_time,
            weekdays: api.weekdays,
        }
    }
}

impl From<SchedItem> for SchedItemDB {
    fn from(scheditem: SchedItem) -> Self {
        Self {
            id: scheditem.id.and_then(|s| s.parse().ok()),
            date: scheditem.date,
            represented_hour_start: scheditem.represented_hour_start,
            has_time: scheditem.has_time,
            time_left_mins: scheditem.time_left_mins,
            title: scheditem.title,
        }
    }
}

impl From<SchedItemDB> for SchedItem {
    fn from(value: SchedItemDB) -> Self {
        Self {
            id: value.id.map(|s| s.to_string()),
            date: value.date,
            represented_hour_start: value.represented_hour_start,
            has_time: value.has_time,
            time_left_mins: value.time_left_mins,
            title: value.title,
        }
    }
}

pub async fn add_schedule(
    State(conn): State<Surreal<Db>>,
    Json(new_routine): Json<Routine>,
) -> impl IntoResponse {
    conn.use_ns("core").use_db("main").await.unwrap();
    let db_scheditem = RoutineDB::from(new_routine);
    let _: Option<RoutineDB> = conn
        .create("static_schedule")
        .content(db_scheditem)
        .await
        .unwrap();
    StatusCode::CREATED
}

// Make day_tasks table dynamically updateable such that if user adds new task and there is extra space left in the table then it is added for the same day instead of keeping the schedule fixed (Should work only for future and no the past)
pub async fn get_schedule_by_day(
    State(conn): State<Surreal<Db>>,
    Path(day_str): Path<String>,
) -> impl IntoResponse {
    let date = NaiveDate::parse_from_str(&day_str, "%Y-%m-%d").unwrap();
    conn.use_ns("core").use_db("main").await.unwrap();
    let mut today_scheditems_dbresp = conn
        .query("SELECT * FROM day_schedule WHERE $date = date")
        .bind(("date", date))
        .await
        .unwrap();
    let today_scheditems_db: Vec<SchedItemDB> = today_scheditems_dbresp.take(0).unwrap();
    if !today_scheditems_db.is_empty() {
        let today_scheditems: Vec<SchedItem> = today_scheditems_db
            .into_iter()
            .map(SchedItem::from)
            .collect();
        Json(today_scheditems)
    } else {
        let schedule_today = create_day_sched(
            &conn,
            date,
            get_day_static_schedule(&conn, date).await,
            get_all_tasks_sorted(&conn).await,
        )
        .await;
        for val in schedule_today.clone() {
            let db_value = SchedItemDB::from(val);
            let _: Option<SchedItemDB> =
                conn.create("day_schedule").content(db_value).await.unwrap();
        }
        Json(schedule_today)
    }
}

async fn create_day_sched(
    conn: &Surreal<Db>,
    date: NaiveDate,
    static_sched_for_today: Vec<Routine>,
    mut tasks_sorted: Vec<Task>,
) -> Vec<SchedItem> {
    let mut day_sched: Vec<SchedItem> = Vec::new();
    for x in 00..24 {
        day_sched.push(SchedItem {
            id: None,
            date,
            represented_hour_start: x,
            has_time: true,
            time_left_mins: 60,
            title: Vec::new(),
        });
    }
    for scheditem in &mut day_sched {
        for routine in &static_sched_for_today {
            if routine.start_time.hour() == scheditem.represented_hour_start as u32 {
                scheditem.title.push(routine.title.clone());
                scheditem.time_left_mins = scheditem
                    .time_left_mins
                    .saturating_sub(routine.start_time.minute() as u8);
                if scheditem.time_left_mins == 0 {
                    scheditem.has_time = false
                }
            } else if (routine.start_time.hour() as u8) < scheditem.represented_hour_start
                && (routine.end_time.hour() as u8) > scheditem.represented_hour_start
            {
                scheditem.title.push(routine.title.clone());
                scheditem.has_time = false;
                scheditem.time_left_mins = 0;
            }
        }
    }
    'scheditem_loop: for scheditem in &mut day_sched {
        if scheditem.has_time {
            for task in &mut tasks_sorted {
                if !(task.is_done || task.time_alloted >= task.req_time) {
                    let time_alloted: u8 = scheditem.time_left_mins.min(
                        task.req_time
                            .signed_duration_since(task.time_alloted)
                            .num_minutes() as u8,
                    );
                    task.time_alloted +=
                        TimeDelta::try_minutes(time_alloted as i64).unwrap_or(TimeDelta::zero());
                    let task_db = TaskDB::from(task.to_owned());
                    let id = task.id.clone().unwrap_or("Task".to_string());
                    let (table, key) = id.split_once(':').unwrap_or(("Tasks", id.as_str()));
                    conn.use_ns("core").use_db("main").await.unwrap();
                    let _: Option<TaskDB> =
                        conn.update((table, key)).content(task_db).await.unwrap();
                    scheditem.time_left_mins =
                        scheditem.time_left_mins.saturating_sub(time_alloted);
                    scheditem.title.push(task.name.clone());
                    let low_remaining_time_threshold: u8 = 4;
                    if scheditem.time_left_mins <= low_remaining_time_threshold {
                        scheditem.has_time = false;
                        continue 'scheditem_loop;
                    }
                }
            }
        }
    }
    day_sched
}

async fn get_day_static_schedule(conn: &Surreal<Db>, date: NaiveDate) -> Vec<Routine> {
    conn.use_ns("core").use_db("main").await.unwrap();
    let sql = "SELECT * FROM static_schedule WHERE $date IN start_date..=end_date";
    let mut result = conn.query(sql).bind(("date", date)).await.unwrap();
    let schedule_db: Vec<RoutineDB> = result.take(0).unwrap();
    let schedule_today: Vec<Routine> = schedule_db.into_iter().map(Routine::from).collect();
    schedule_today
}

async fn get_all_tasks_sorted(conn: &Surreal<Db>) -> Vec<Task> {
    conn.use_ns("core").use_db("main").await.unwrap();
    let tasks: Vec<TaskDB> = conn.select("Tasks").await.unwrap();
    let tasks: Vec<Task> = tasks.into_iter().map(Task::from).collect();
    let mut tasks: Vec<Task> = tasks
        .into_iter()
        .filter(|task| task.due_by > Local::now().naive_local())
        .collect();
    tasks.sort_by_key(|task| {
        -((task.due_by - Local::now().naive_local()).num_minutes() as i32)
            + (task.imp_lvl * 10) as i32
    });
    tasks
}
