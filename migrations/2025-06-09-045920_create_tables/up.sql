-- Your SQL goes here
CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    sitename TEXT NOT NULL,
    siteurl TEXT NOT NULL,
    email TEXT,
    username TEXT,
    password TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS secrets (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    masterkey_hash TEXT NOT NULL,
    device_secret TEXT NOT NULL
);
