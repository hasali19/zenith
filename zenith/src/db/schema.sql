CREATE TABLE IF NOT EXISTS media_items (
    id INTEGER PRIMARY KEY,
    parent_id INTEGER,
    item_type INTEGER NOT NULL,
    path TEXT,

    name TEXT,
    index_number INTEGER,
    release_date INTEGER,
    overview TEXT,

    duration REAL,

    primary_image TEXT,
    backdrop_image TEXT,

    tmdb_id INTEGER,

    added_at INTEGER DEFAULT (strftime('%s','now')),
    updated_at INTEGER,

    FOREIGN KEY (parent_id) REFERENCES media_items (id)
);

CREATE TABLE IF NOT EXISTS user_item_data (
    item_id INTEGER PRIMARY KEY,
    position REAL NOT NULL DEFAULT 0,
    is_watched BOOLEAN NOT NULL DEFAULT 0,

    FOREIGN KEY (item_id) REFERENCES media_items (id)
);
