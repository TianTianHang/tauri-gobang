CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE TABLE IF NOT EXISTS rooms (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    host_id TEXT NOT NULL,
    player2_id TEXT,
    status TEXT NOT NULL DEFAULT 'waiting',
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    ended_at INTEGER,
    FOREIGN KEY (host_id) REFERENCES users(id),
    FOREIGN KEY (player2_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS games (
    id TEXT PRIMARY KEY NOT NULL,
    room_id TEXT NOT NULL,
    winner_id TEXT,
    reason TEXT,
    ended_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (room_id) REFERENCES rooms(id),
    FOREIGN KEY (winner_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_rooms_status ON rooms(status);
CREATE INDEX IF NOT EXISTS idx_rooms_created_at ON rooms(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_games_room_id ON games(room_id);
