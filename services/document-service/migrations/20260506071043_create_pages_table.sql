-- Add migration script here
CREATE TABLE docs.pages (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL,
    parent_id UUID,
    created_by UUID NOT NULL,
    last_edited_by UUID NOT NULL,
    title VARCHAR NOT NULL,
    icon VARCHAR,
    cover_url VARCHAR,
    is_database BOOL NOT NULL DEFAULT FALSE,
    visibility VARCHAR NOT NULL CHECK (
        visibility IN ('private', 'workspace', 'custom', 'public')
    ),
    locked BOOL NOT NULL DEFAULT FALSE,
    locked_by UUID,
    version INTEGER NOT NULL DEFAULT 0,
    published_slug VARCHAR UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ
);
