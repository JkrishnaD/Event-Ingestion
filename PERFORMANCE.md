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

## Next Optimization Phase

### Planned improvements:

- Batch inserts
- Background worker
- Queue buffering
- Async ingestion
