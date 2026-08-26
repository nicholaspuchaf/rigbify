ALTER TABLE tracks ADD COLUMN source_page_url TEXT;
ALTER TABLE tracks ADD COLUMN external_id TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS idx_tracks_external_origin
ON tracks(source_type, external_id)
WHERE external_id IS NOT NULL;

INSERT INTO schema_migrations(version, applied_at)
VALUES (4, unixepoch('now') * 1000);

