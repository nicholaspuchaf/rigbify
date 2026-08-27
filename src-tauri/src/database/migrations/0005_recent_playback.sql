ALTER TABLE playlists ADD COLUMN last_played_at INTEGER;

CREATE INDEX IF NOT EXISTS idx_tracks_last_played_at
ON tracks(last_played_at DESC, id);

CREATE INDEX IF NOT EXISTS idx_playlists_last_played_at
ON playlists(last_played_at DESC, id);

INSERT INTO schema_migrations(version, applied_at)
VALUES (5, unixepoch('now') * 1000);
