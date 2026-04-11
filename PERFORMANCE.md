# Event Ingestion Performance Report

## Overview

This document captures performance benchmarks for the Event Ingestion service.

The service currently follows a **single insert per request** architecture:

Request Flow:

Client → Axum → JSON Parsing → SQLx → PostgreSQL → Response

Each HTTP request performs:

- JSON parsing
- Database insert
- Response serialization

---

## Test Environment

- Language: Rust
- Framework: Axum
- Runtime: Tokio
- Database: PostgreSQL (Docker)
- Driver: SQLx
- Load Tool: oha
- Duration: 10 seconds per test

---

## Load Test Command

```bash
oha -z 10s -c <concurrency> --no-tui \
  -m POST \
  -H "Content-Type: application/json" \
  -d '{"app_id":1,"user_id":42,"event_type":"click","data":{"page":"home"}}' \
  http://127.0.0.1:3000/event
```

## Single Insert Performance

| Concurrency | Req/sec | p50      | p95      | p99      | Avg Latency |
| ----------- | ------- | -------- | -------- | -------- | ----------- |
| 10          | 811     | 9.57ms   | 20.43ms  | 60.49ms  | 12.29ms     |
| 50          | 1539    | 30.05ms  | 49.48ms  | 111.95ms | 32.49ms     |
| 100         | 1650    | 58.92ms  | 90.29ms  | 101.35ms | 60.70ms     |
| 200         | 1783    | 109.91ms | 148.94ms | 194.47ms | 112.61ms    |

## Summary

## Throughput:

- Peak throughput observed: 1783 req/sec
- Throughput increases with concurrency
- Throughput stabilizes after concurrency 100

## Latency

Latency increases with concurrency:
| Concurrency | Avg Latency |
| ----------- | ----------- |
| 10 | 12ms |
| 50 | 32ms |
| 100 | 60ms |
| 200 | 112ms |

This indicates:

- CPU contention
- Database bottleneck
- Connection pool saturation

## Observations

## Good

- 100% success rate
- Stable throughput
- No server crashes
- Predictable latency growth

## Bottlenecks Observed

- Latency increases with concurrency
- Database insert becoming bottleneck
- Throughput plateaus after concurrency 100

## Environment Considerations

Benchmarks ran against local Docker PostgreSQL (~0.1ms network latency).
In production with a remote database (e.g., Neon, AWS RDS), expect:

- 50-200ms additional latency per query from network round trips
- Higher p99 variance due to network jitter
- Connection timeouts under high concurrency
- Pool exhaustion happening at lower concurrency levels

The batching optimization becomes even more critical with remote databases, since each round trip is expensive — reducing 1000 individual INSERTs to 1 bulk INSERT saves ~999 network round trips.

## Batch Insert Performance (Channel + QueryBuilder)

Architecture:
Client → Axum → Channel (mpsc) → Background Batch Writer → Bulk INSERT → PostgreSQL

The handler sends events into an in-memory channel and returns 202 immediately. A background task collects events and flushes them to Postgres in bulk using sqlx::QueryBuilder when the buffer hits 500 events or 500ms elapses.

| Concurrency | Req/sec | p50    | p95     | p99     | Avg Latency |
| ----------- | ------- | ------ | ------- | ------- | ----------- |
| 10          | 20,992  | 0.39ms | 0.86ms  | 1.73ms  | 0.47ms      |
| 50          | 21,760  | 1.74ms | 5.90ms  | 13.51ms | 2.29ms      |
| 100         | 21,633  | 3.40ms | 13.50ms | 18.33ms | 4.62ms      |
| 200         | 20,128  | 7.81ms | 21.06ms | 27.78ms | 9.92ms      |

## Comparison: Naive vs Batched

| Concurrency | Naive Req/sec | Batched Req/sec | Speedup | Naive p99 | Batched p99 |
| ----------- | ------------- | --------------- | ------- | --------- | ----------- |
| 10          | 811           | 20,992          | 25.8x   | 60.49ms   | 1.73ms      |
| 50          | 1,539         | 21,760          | 14.1x   | 111.95ms  | 13.51ms     |
| 100         | 1,650         | 21,633          | 13.1x   | 101.35ms  | 18.33ms     |
| 200         | 1,783         | 20,128          | 11.3x   | 194.47ms  | 27.78ms     |

Key takeaway: Decoupling the HTTP handler from database writes and using bulk INSERTs improved throughput by 11-25x and reduced p99 latency by 7-35x depending on concurrency level.

## Next Optimization Phase

### Planned improvements:

- graceful shutdown flushing the buffer
- Background worker
- Queue buffering
- Async ingestion
