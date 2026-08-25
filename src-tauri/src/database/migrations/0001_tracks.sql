CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS tracks (
    id TEXT PRIMARY KEY NOT NULL,
    source_type TEXT NOT NULL,
    source_uri TEXT NOT NULL,
    title TEXT NOT NULL,
    artist TEXT,
    album TEXT,
    duration_ms INTEGER NOT NULL CHECK (duration_ms >= 0),
    artwork_uri TEXT,
    mime_type TEXT,
    file_size INTEGER,
    track_number INTEGER,
    disc_number INTEGER,
    year INTEGER,
    genre TEXT,
    added_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    last_played_at INTEGER,
    play_count INTEGER NOT NULL DEFAULT 0 CHECK (play_count >= 0),
    favorite INTEGER NOT NULL DEFAULT 0 CHECK (favorite IN (0, 1)),
    availability TEXT NOT NULL DEFAULT 'available',
    UNIQUE(source_type, source_uri)
);

CREATE INDEX IF NOT EXISTS idx_tracks_title ON tracks(title COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_tracks_artist ON tracks(artist COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_tracks_added_at ON tracks(added_at DESC);

INSERT OR IGNORE INTO schema_migrations(version, applied_at)
VALUES (1, unixepoch('now') * 1000);
