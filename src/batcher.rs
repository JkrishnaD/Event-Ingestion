use std::time::Duration;

use sqlx::{PgPool, QueryBuilder};
use tokio::{sync::mpsc, time::interval};
use tracing::instrument;

use crate::handlers::events::IncomingEvent;

pub async fn batch_jobs(mut rx: mpsc::Receiver<IncomingEvent>, pool: PgPool) {
    // Buffer for collecting events before flushing
    let mut buffer: Vec<IncomingEvent> = Vec::with_capacity(1000);
    // Interval for periodic flushing of the buffer
    let mut flush_interval = interval(Duration::from_millis(500));

    // Batch size for flushing the buffer
    let batch_size = 500;
    tracing::info!("Batching Started...");

    loop {
        tokio::select! {
            // Incoming event from the channel
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
            // Periodic flush of the buffer
            _ = flush_interval.tick() => {
                if !buffer.is_empty() {
                    flush_buffer(&mut buffer, &pool).await;
                }
            }
        }
    }
}

// Flushes the buffer to the database
#[instrument(skip(buffer, pool) fields(batch_size = buffer.len()))]
pub async fn flush_buffer(buffer: &mut Vec<IncomingEvent>, pool: &PgPool) {
    if buffer.is_empty() {
        return;
    }

    let events = std::mem::take(buffer);
    let start = std::time::Instant::now();

    // Build the query for inserting events into the database
    let mut query_builder =
        QueryBuilder::new("INSERT INTO events (app_id, user_id, event_type, data) ");

    // Push values for each event into the query builder
    query_builder.push_values(events.iter(), |mut b, event| {
        b.push_bind(event.app_id)
            .push_bind(event.user_id)
            .push_bind(&event.event_type)
            .push_bind(&event.data);
    });

    // Build the final query
    let query = query_builder.build();
    // Execute the query and handle the result
    match query.execute(pool).await {
        Ok(_) => {
            let elapsed = start.elapsed();
            tracing::info!(
                events_processed = events.len(),
                elapsed_ms = elapsed.as_millis() as u64,
                "batch flushed"
            );
        }
        Err(e) => {
            tracing::error!(
                events_lost = events.len(),
                error = %e,
                "batch flush failed"
            );
        }
    }
}
