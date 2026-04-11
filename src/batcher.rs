use std::time::Duration;

use sqlx::PgPool;
use tokio::{sync::mpsc, time::interval};

use crate::handlers::events::IncomingEvent;

pub async fn batch_jobs(mut rx: mpsc::Receiver<IncomingEvent>, pool: PgPool) {
    let mut buffer: Vec<IncomingEvent> = Vec::with_capacity(500);
    let mut flush_interval = interval(Duration::from_millis(500));

    let batch_size = 500;
    tracing::info!("Batching started");

    loop {
        tokio::select! {
            event = rx.recv() => {
                match event {
                    Some(event) => {
                        buffer.push(event);
                        if buffer.len() >= batch_size {
                            flush_buffer(&mut buffer, &pool).await;
                        }
                    }
                    None => {
                        if !buffer.is_empty() {
                            flush_buffer(&mut buffer, &pool).await;
                        }
                        drop(rx);
                        tracing::info!("channel closed, shutting down batch writer");
                        break;
                    }
                }
            },
            _ = flush_interval.tick() => {
                if !buffer.is_empty() {
                    flush_buffer(&mut buffer, &pool).await;
                }
            }
        }
    }
}

pub async fn flush_buffer(buffer: &mut Vec<IncomingEvent>, pool: &PgPool) {
    if buffer.is_empty() {
        return;
    }

    let events = std::mem::take(buffer);

    let start = std::time::Instant::now();

    for event in &events {
        let _ = sqlx::query!(
            "INSERT INTO events (app_id, user_id, event_type, data) VALUES ($1, $2, $3, $4)",
            &event.app_id,
            &event.user_id,
            &event.event_type,
            &event.data,
        )
        .execute(pool)
        .await;
    }

    let elapsed = start.elapsed();
    tracing::info!("Flushed {} events in {:?}", events.len(), elapsed);
}
