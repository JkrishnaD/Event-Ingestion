-- events table
CREATE TABLE IF NOT EXISTS events (
    id BIGSERIAL UNIQUE PRIMARY KEY,
    app_id INT NOT NULL,
    user_id INT NOT NULL,
    event_type TEXT NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    ingested_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for faster querying
CREATE INDEX IF NOT EXISTS idx_events_user_id ON events(user_id);
CREATE INDEX IF NOT EXISTS idx_events_event_type ON events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_app_id_created_at ON events(app_id, created_at);
