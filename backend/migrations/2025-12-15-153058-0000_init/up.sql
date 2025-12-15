CREATE TABLE urls (
    id              TEXT NOT NULL PRIMARY KEY UNIQUE,
    long_url        TEXT NOT NULL,
    clicks          INTEGER NOT NULL DEFAULT 0,

    expires_at      BIGINT,
    max_clicks      INTEGER,
    disabled        BOOLEAN NOT NULL DEFAULT FALSE,

    last_clicked_at BIGINT,
    created_at      BIGINT NOT NULL
);
