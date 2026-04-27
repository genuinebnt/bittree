CREATE TABLE users (
    id               UUID        NOT NULL PRIMARY KEY DEFAULT uuidv7(),
    username         VARCHAR(255) NOT NULL UNIQUE,
    email            VARCHAR(255) NOT NULL UNIQUE,
    password_hash    TEXT        NOT NULL,
    email_verified   BOOL        NOT NULL DEFAULT FALSE,
    is_active        BOOL        NOT NULL DEFAULT TRUE,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
