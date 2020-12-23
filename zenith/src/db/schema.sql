CREATE TABLE IF NOT EXISTS movies (
    id INTEGER PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    year INTEGER,
    overview TEXT,
    poster TEXT,
    backdrop TEXT,
    tmdb_id INTEGER,
    video_path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tv_shows (
    id INTEGER PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    overview TEXT,
    poster TEXT,
    backdrop TEXT,
    tmdb_id INTEGER
);

CREATE TABLE IF NOT EXISTS tv_episodes (
    id INTEGER PRIMARY KEY,
    show_id INTEGER NOT NULL,
    season INTEGER NOT NULL,
    episode INTEGER NOT NULL,
    name TEXT,
    overview TEXT,
    thumbnail TEXT,
    tmdb_id INTEGER,
    video_path TEXT NOT NULL,

    FOREIGN KEY (show_id) REFERENCES tv_shows (id)
);
