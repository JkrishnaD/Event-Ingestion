# Event-Ingestion

Event Ingestion is a system where client-side events such as cursor movements, keyboard clicks, and other interactions are collected and stored in a database.

## Main Purpose

The main purpose of this project is to monitor high request volumes hitting the backend and observe how the system responds.

## Architectural Flow

```bash
Client → Axum → JSON Parsing → Channel (mpsc) → Batching → Bluk Insert (SQLx) → Postgres Database
```

## Local Testing

The main purpose of this project is to monitor high request volumes hitting the backend and observe how the system responds.

Docker cmd:

```bash
docker run -d \
  --name postgres-events \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=event_ingestion \
  -p 5432:5432 \
  postgres:16
```

Now paste the url in your env file and then

```bash
cargo run
```

## Api Endpoints

Currently, This project consists of two endpoints:

- `GET /health` -> This endpoint provides details about the server status and database connections.
- `POST /event` -> This is the main endpoint used for performance testing. Right now this is a simple endpoint where we just insert the event triggered to the database.

## Performance

Most of the system actions are traced using `tracing` and `tracing-subscriber` crates, You will see them used throughout the repository.

Now the api endpoint performances are calculated using `oha` to test this just run `cargo install oha` and then run this cmd:

```bash
oha -z 10s -c 10 --no-tui \
  -m POST \
  -H "Content-Type: application/json" \
  -d '{"app_id":1,"user_id":42,"event_type":"click","data":{"page":"home"}}' \
  http://127.0.0.1:3000/event
```

Based on your requirments you can change the `z` and `c` values and you can change the request which you are testing

I have run a few performances tests so check them out in here [PERFORMANCES](./PERFORMANCE.md)
