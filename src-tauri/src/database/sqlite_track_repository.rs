use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::{
    errors::{AppError, AppResult},
    media::{Availability, SourceType, Track, TrackMetadataUpdate, TrackRepository},
};

const MIGRATION_0001: &str = include_str!("migrations/0001_tracks.sql");
const MIGRATION_0002: &str = include_str!("migrations/0002_track_actions.sql");
const MIGRATION_0003: &str = include_str!("migrations/0003_playlist_details.sql");
const MIGRATION_0004: &str = include_str!("migrations/0004_download_origins.sql");

pub struct SqliteTrackRepository {
    pub(crate) connection: Connection,
}

impl SqliteTrackRepository {
    pub fn open(path: &Path) -> AppResult<Self> {
        let connection = Connection::open(path)?;
        Self::from_connection(connection)
    }

    pub fn in_memory() -> AppResult<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    fn from_connection(mut connection: Connection) -> AppResult<Self> {
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.execute_batch("CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY, applied_at INTEGER NOT NULL);")?;
        let version: i64 = connection.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )?;
        let transaction = connection.transaction()?;
        if version < 1 {
            transaction.execute_batch(MIGRATION_0001)?;
        }
        if version < 2 {
            transaction.execute_batch(MIGRATION_0002)?;
        }
        if version < 3 {
            transaction.execute_batch(MIGRATION_0003)?;
        }
        if version < 4 {
            transaction.execute_batch(MIGRATION_0004)?;
        }
        transaction.commit()?;
        Ok(Self { connection })
    }

    pub(crate) fn read_track(row: &Row<'_>) -> rusqlite::Result<Track> {
        let source_type_value: String = row.get("source_type")?;
        let availability_value: String = row.get("availability")?;
        let parse_error = |value: String| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(AppError::InvalidTrack(value)),
            )
        };

        Ok(Track {
            id: row.get("id")?,
            source_type: SourceType::parse(&source_type_value)
                .map_err(|_| parse_error(source_type_value))?,
            source_uri: row.get("source_uri")?,
            source_page_url: row.get("source_page_url")?,
            external_id: row.get("external_id")?,
            title: row.get("title")?,
            artist: row.get("artist")?,
            album: row.get("album")?,
            duration_ms: row.get("duration_ms")?,
            artwork_uri: row.get("artwork_uri")?,
            mime_type: row.get("mime_type")?,
            file_size: row.get("file_size")?,
            track_number: row.get("track_number")?,
            disc_number: row.get("disc_number")?,
            year: row.get("year")?,
            genre: row.get("genre")?,
            added_at: row.get("added_at")?,
            updated_at: row.get("updated_at")?,
            last_played_at: row.get("last_played_at")?,
            play_count: row.get("play_count")?,
            favorite: row.get("favorite")?,
            availability: Availability::parse(&availability_value)
                .map_err(|_| parse_error(availability_value))?,
        })
    }
}

impl TrackRepository for SqliteTrackRepository {
    fn upsert(&mut self, track: &Track) -> AppResult<Track> {
        self.connection.execute(
            "INSERT INTO tracks (id, source_type, source_uri, source_page_url, external_id, title, artist, album, duration_ms, artwork_uri, mime_type, file_size, track_number, disc_number, year, genre, added_at, updated_at, last_played_at, play_count, favorite, availability)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)
             ON CONFLICT(source_type, source_uri) DO UPDATE SET
               source_page_url = excluded.source_page_url, external_id = excluded.external_id,
               title = excluded.title, artist = excluded.artist, album = excluded.album,
               duration_ms = excluded.duration_ms, artwork_uri = excluded.artwork_uri,
               mime_type = excluded.mime_type, file_size = excluded.file_size,
               track_number = excluded.track_number, disc_number = excluded.disc_number,
               year = excluded.year, genre = excluded.genre, updated_at = excluded.updated_at,
               availability = excluded.availability",
            params![track.id, track.source_type.as_str(), track.source_uri, track.source_page_url, track.external_id,
                track.title, track.artist, track.album, track.duration_ms, track.artwork_uri, track.mime_type,
                track.file_size, track.track_number, track.disc_number, track.year, track.genre, track.added_at,
                track.updated_at, track.last_played_at, track.play_count, track.favorite, track.availability.as_str()],
        )?;

        self.connection
            .query_row(
                "SELECT * FROM tracks WHERE source_type = ?1 AND source_uri = ?2",
                params![track.source_type.as_str(), track.source_uri],
                Self::read_track,
            )
            .map_err(Into::into)
    }

    fn list(&self) -> AppResult<Vec<Track>> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM tracks ORDER BY added_at DESC, title COLLATE NOCASE")?;
        let tracks = statement
            .query_map([], Self::read_track)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tracks)
    }

    fn find_by_id(&self, id: &str) -> AppResult<Option<Track>> {
        self.connection
            .query_row("SELECT * FROM tracks WHERE id = ?1", [id], Self::read_track)
            .optional()
            .map_err(Into::into)
    }

    fn set_favorite(&mut self, id: &str, favorite: bool, updated_at: i64) -> AppResult<Track> {
        let changed = self.connection.execute(
            "UPDATE tracks SET favorite = ?2, updated_at = ?3 WHERE id = ?1",
            params![id, favorite, updated_at],
        )?;
        if changed == 0 {
            return Err(AppError::TrackNotFound);
        }
        self.find_by_id(id)?.ok_or(AppError::TrackNotFound)
    }

    fn update_metadata(
        &mut self,
        id: &str,
        metadata: &TrackMetadataUpdate,
        updated_at: i64,
    ) -> AppResult<Track> {
        let changed = self.connection.execute(
            "UPDATE tracks SET title = ?2, artist = ?3, album = ?4, updated_at = ?5 WHERE id = ?1",
            params![
                id,
                metadata.title.trim(),
                metadata.artist,
                metadata.album,
                updated_at
            ],
        )?;
        if changed == 0 {
            return Err(AppError::TrackNotFound);
        }
        self.find_by_id(id)?.ok_or(AppError::TrackNotFound)
    }

    fn add_to_playlist(
        &mut self,
        track_id: &str,
        playlist_id: &str,
        playlist_name: &str,
        now: i64,
    ) -> AppResult<()> {
        if self.find_by_id(track_id)?.is_none() {
            return Err(AppError::TrackNotFound);
        }
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT INTO playlists (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)
             ON CONFLICT(id) DO UPDATE SET name = excluded.name, updated_at = excluded.updated_at",
            params![playlist_id, playlist_name, now],
        )?;
        let position: i64 = transaction.query_row(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM playlist_tracks WHERE playlist_id = ?1",
            [playlist_id],
            |row| row.get(0),
        )?;
        transaction.execute(
            "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position, added_at) VALUES (?1, ?2, ?3, ?4)",
            params![playlist_id, track_id, position, now],
        )?;
        transaction.commit()?;
        Ok(())
    }

    fn remove(&mut self, id: &str) -> AppResult<()> {
        let changed = self
            .connection
            .execute("DELETE FROM tracks WHERE id = ?1", [id])?;
        if changed == 0 {
            return Err(AppError::TrackNotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::NewTrack;

    fn track(uri: &str, title: &str, now: i64) -> Track {
        NewTrack {
            source_type: SourceType::LocalDocument,
            source_uri: uri.into(),
            source_page_url: None,
            external_id: None,
            title: Some(title.into()),
            file_name: None,
            artist: Some("Artist".into()),
            album: Some("Album".into()),
            duration_ms: 120_000,
            artwork_uri: None,
            mime_type: Some("audio/mpeg".into()),
            file_size: Some(10),
            track_number: None,
            disc_number: None,
            year: None,
            genre: None,
        }
        .normalize(now)
        .expect("valid fixture")
    }

    #[test]
    fn persists_and_lists_tracks() {
        let mut repository = SqliteTrackRepository::in_memory().expect("database");
        repository
            .upsert(&track("content://1", "First", 1))
            .expect("insert");
        repository
            .upsert(&track("content://2", "Second", 2))
            .expect("insert");
        let tracks = repository.list().expect("list");
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].title, "Second");
    }

    #[test]
    fn deduplicates_by_source_and_uri_while_preserving_id() {
        let mut repository = SqliteTrackRepository::in_memory().expect("database");
        let first = repository
            .upsert(&track("content://same", "Old", 1))
            .expect("insert");
        let updated = repository
            .upsert(&track("content://same", "New", 2))
            .expect("update");
        assert_eq!(updated.id, first.id);
        assert_eq!(updated.title, "New");
        assert_eq!(repository.list().expect("list").len(), 1);
    }

    #[test]
    fn updates_favorite_and_removes_track() {
        let mut repository = SqliteTrackRepository::in_memory().expect("database");
        let saved = repository
            .upsert(&track("content://1", "First", 1))
            .expect("insert");
        let favorite = repository
            .set_favorite(&saved.id, true, 3)
            .expect("favorite");
        assert!(favorite.favorite);
        repository.remove(&saved.id).expect("remove");
        assert!(repository.list().expect("list").is_empty());
    }

    #[test]
    fn updates_metadata_and_adds_track_to_playlist() {
        let mut repository = SqliteTrackRepository::in_memory().expect("database");
        let saved = repository
            .upsert(&track("content://1", "Old", 1))
            .expect("insert");
        let updated = repository
            .update_metadata(
                &saved.id,
                &TrackMetadataUpdate {
                    title: "New".into(),
                    artist: Some("Other".into()),
                    album: None,
                },
                2,
            )
            .expect("metadata");
        assert_eq!(updated.title, "New");
        repository
            .add_to_playlist(&saved.id, "favorites-2026", "Favoritas", 3)
            .expect("playlist");
        let count: i64 = repository
            .connection
            .query_row("SELECT COUNT(*) FROM playlist_tracks", [], |row| row.get(0))
            .expect("count");
        assert_eq!(count, 1);
    }

    #[test]
    fn migrates_an_existing_version_two_database() {
        let connection = Connection::open_in_memory().expect("connection");
        connection.execute_batch(MIGRATION_0001).expect("v1");
        connection.execute_batch(MIGRATION_0002).expect("v2");
        let repository = SqliteTrackRepository::from_connection(connection).expect("migration");
        let version: i64 = repository
            .connection
            .query_row("SELECT MAX(version) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .expect("version");
        let sort_mode: String = repository
            .connection
            .query_row(
                "SELECT dflt_value FROM pragma_table_info('playlists') WHERE name='sort_mode'",
                [],
                |row| row.get(0),
            )
            .expect("column");
        assert_eq!(version, 4);
        assert_eq!(sort_mode, "'custom'");
    }
}
