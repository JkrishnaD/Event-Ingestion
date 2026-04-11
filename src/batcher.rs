use std::time::Duration;

use sqlx::{PgPool, QueryBuilder};
use tokio::{sync::mpsc, time::interval};

use crate::handlers::events::IncomingEvent;

pub async fn batch_jobs(mut rx: mpsc::Receiver<IncomingEvent>, pool: PgPool) {
    let mut buffer: Vec<IncomingEvent> = Vec::with_capacity(1000);
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

    let mut query_builder =
        QueryBuilder::new("INSERT INTO events (app_id, user_id, event_type, data) ");

    query_builder.push_values(events.iter(), |mut b, event| {
        b.push_bind(event.app_id)
            .push_bind(event.user_id)
            .push_bind(&event.event_type)
            .push_bind(&event.data);
    });

    let query = query_builder.build();
    let _result = query.execute(pool).await;

    let elapsed = start.elapsed();
    tracing::info!("Flushed {} events in {:?}", events.len(), elapsed);
}
