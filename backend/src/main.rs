use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    serve,
};
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::{
    Surreal,
    engine::local::{Db, RocksDb},
};
use tokio::net::TcpListener;

use surrealdb::RecordId;

// Custom deserializer to convert RecordId to String
fn recordid_to_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let record_id = RecordId::deserialize(deserializer)?;
    Ok(Some(record_id.to_string()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    #[serde(deserialize_with = "recordid_to_string")]
    id: Option<String>,
    name: String,
    description: String,
    due_by: String,
    imp_lvl: u8,
    req_time: String,
    is_done: bool,
}

#[tokio::main]
async fn main() {
    let db_conn = Surreal::new::<RocksDb>("TodosApp").await.unwrap();
    let router = Router::new()
        .route("/get_todos", get(get_todos))
        .route("/add_todo", post(add_todo))
        .route("/mark_done", post(mark_done))
        .route("/mark_undone", post(mark_undone))
        .with_state(db_conn)
        .layer(tower_http::cors::CorsLayer::permissive());
    let addr = TcpListener::bind("localhost:3000")
        .await
        .expect("Couldn't connect to port 3000");
    serve(addr, router).await.unwrap()
}

async fn get_todos(State(conn): State<Surreal<Db>>) -> impl IntoResponse {
    conn.use_ns("core").use_db("todos").await.unwrap();
    let values: Vec<Todo> = conn.select("todos").await.unwrap();
    Json(values)
}

async fn add_todo(
    State(conn): State<Surreal<Db>>,
    Json(new_task): Json<Todo>,
) -> impl IntoResponse {
    conn.use_ns("core").use_db("todos").await.unwrap();
    let _created_task: Vec<Todo> = conn.insert("todos").content(new_task).await.unwrap();
    StatusCode::CREATED
}

async fn mark_done(State(conn): State<Surreal<Db>>, Json(id): Json<String>) -> impl IntoResponse {
    conn.use_ns("core").use_db("todos").await.unwrap();
    let record_id: RecordId = id.parse().unwrap(); // Parse string to RecordId
    conn.query("UPDATE $id SET is_done = true")
        .bind(("id", record_id))
        .await
        .unwrap();
    StatusCode::ACCEPTED
}

async fn mark_undone(State(conn): State<Surreal<Db>>, Json(id): Json<String>) -> impl IntoResponse {
    conn.use_ns("core").use_db("todos").await.unwrap();
    let record_id: RecordId = id.parse().unwrap(); // Parse string to RecordId
    conn.query("UPDATE $id SET is_done = false")
        .bind(("id", record_id))
        .await
        .unwrap();
    StatusCode::ACCEPTED
}
