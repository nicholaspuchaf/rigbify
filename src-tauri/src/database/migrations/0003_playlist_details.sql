ALTER TABLE playlists ADD COLUMN description TEXT;
ALTER TABLE playlists ADD COLUMN artwork_uri TEXT;
ALTER TABLE playlists ADD COLUMN sort_mode TEXT NOT NULL DEFAULT 'custom';
ALTER TABLE playlists ADD COLUMN sort_direction TEXT NOT NULL DEFAULT 'ascending';
ALTER TABLE playlists ADD COLUMN version INTEGER NOT NULL DEFAULT 1;

CREATE INDEX IF NOT EXISTS idx_playlist_tracks_position
ON playlist_tracks(playlist_id, position);

INSERT INTO schema_migrations(version, applied_at)
VALUES (3, unixepoch('now') * 1000);
