CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE books (
                       id          UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
                       title       TEXT        NOT NULL,
                       file_type   TEXT        NOT NULL,
                       total_pages INTEGER     NOT NULL DEFAULT 0,
                       created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE pages (
                       id          UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
                       book_id     UUID        NOT NULL REFERENCES books(id) ON DELETE CASCADE,
                       page_number INTEGER     NOT NULL,
                       content     TEXT        NOT NULL,
                       created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       UNIQUE(book_id, page_number)
);