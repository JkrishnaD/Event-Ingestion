use crate::db::DbPool;
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::Row;

#[derive(Debug, Deserialize)]
pub struct IncomingEvent {
    app_id: i32,
    user_id: i32,
    event_type: String,
    data: serde_json::Value,
}

#[axum::debug_handler]
pub async fn insert_event(
    State(pool): State<DbPool>,
    Json(event): Json<IncomingEvent>,
) -> Result<Json<i64>, StatusCode> {
    tracing::info!("Inserting...");
    let row = sqlx::query(
        r#"INSERT INTO events (app_id, user_id, event_type, data)
        VALUES ($1, $2, $3, $4)
        RETURNING id"#,
    )
    .bind(event.app_id)
    .bind(event.user_id)
    .bind(event.event_type)
    .bind(event.data)
    .fetch_one(&pool.pool)
    .await
    .expect("Failed to insert");

    let id: i64 = row.get("id");

    tracing::info!("Inserted event with id: {}", id);

    Ok(axum::Json(id))
}
