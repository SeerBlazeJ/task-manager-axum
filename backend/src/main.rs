use axum::{
    Router,
    routing::{get, post},
    serve,
};
use surrealdb::{Surreal, engine::local::RocksDb};
use tokio::net::TcpListener;

mod task_helper;
use task_helper::{add_task, delete_task, get_task, get_task_by_id, mark_done, mark_undone};

mod schedule_helper;
use schedule_helper::{add_schedule, get_schedule_by_day};

#[tokio::main]
async fn main() {
    let db_conn = Surreal::new::<RocksDb>("TaskManagerApp").await.unwrap();

    // router for managing various requests
    let router = Router::new()
        .route("/get_tasks", get(get_task))
        .route("/get_task/{id}", get(get_task_by_id))
        .route("/add_task", post(add_task))
        .route("/mark_done", post(mark_done))
        .route("/mark_undone", post(mark_undone))
        .route("/delete", post(delete_task))
        .route("/add_sched", post(add_schedule))
        .route("/get_schedule/{day_str}", get(get_schedule_by_day))
        .with_state(db_conn)
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = TcpListener::bind("localhost:3000")
        .await
        .expect("Couldn't connect to port 3000");
    serve(addr, router).await.unwrap()
}
