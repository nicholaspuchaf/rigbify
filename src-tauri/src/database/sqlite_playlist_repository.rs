use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, Row, Transaction};
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

use crate::{
    errors::{AppError, AppResult},
    media::{
        Playlist, PlaylistDetails, PlaylistOrderUpdate, PlaylistRepository, PlaylistSortMode,
        PlaylistUpdate, SortDirection, Track,
    },
};

use super::SqliteTrackRepository;

pub struct SqlitePlaylistRepository {
    connection: Connection,
}

impl SqlitePlaylistRepository {
    pub fn open(path: &Path) -> AppResult<Self> {
        let connection = Connection::open(path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        Ok(Self { connection })
    }

    #[cfg(test)]
    pub fn in_memory() -> AppResult<Self> {
        let mut track_repository = SqliteTrackRepository::in_memory()?;
        let connection = std::mem::replace(
            &mut track_repository.connection,
            Connection::open_in_memory()?,
        );
        Ok(Self { connection })
    }

    fn read_playlist(row: &Row<'_>) -> rusqlite::Result<Playlist> {
        let count: i64 = row.get("track_count")?;
        let duration: i64 = row.get("duration_ms")?;
        let version: i64 = row.get("version")?;
        Ok(Playlist {
            id: row.get("id")?,
            name: row.get("name")?,
            description: row.get("description")?,
            artwork_uri: row.get("artwork_uri")?,
            sort_mode: PlaylistSortMode::parse(&row.get::<_, String>("sort_mode")?),
            sort_direction: SortDirection::parse(&row.get::<_, String>("sort_direction")?),
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            version: version as u64,
            track_count: count as u64,
            duration_ms: duration as u64,
        })
    }

    fn base_query() -> &'static str {
        "SELECT p.*, COUNT(pt.track_id) AS track_count, COALESCE(SUM(t.duration_ms), 0) AS duration_ms
         FROM playlists p LEFT JOIN playlist_tracks pt ON pt.playlist_id = p.id
         LEFT JOIN tracks t ON t.id = pt.track_id"
    }

    fn get_playlist(connection: &Connection, id: &str) -> AppResult<Playlist> {
        connection
            .query_row(
                &format!("{} WHERE p.id = ?1 GROUP BY p.id", Self::base_query()),
                [id],
                Self::read_playlist,
            )
            .optional()?
            .ok_or(AppError::PlaylistNotFound)
    }

    fn ordered_tracks(&self, playlist: &Playlist) -> AppResult<Vec<Track>> {
        let mut statement = self.connection.prepare(
            "SELECT t.*, pt.position AS playlist_position, pt.added_at AS playlist_added_at FROM playlist_tracks pt JOIN tracks t ON t.id = pt.track_id WHERE pt.playlist_id = ?1",
        )?;
        let mut tracks = statement
            .query_map([&playlist.id], |row| {
                Ok((
                    SqliteTrackRepository::read_track(row)?,
                    row.get::<_, i64>("playlist_position")?,
                    row.get::<_, i64>("playlist_added_at")?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        tracks.sort_by(|left, right| compare_playlist_tracks(left, right, playlist));
        Ok(tracks.into_iter().map(|(track, _, _)| track).collect())
    }

    fn bump(
        transaction: &Transaction<'_>,
        id: &str,
        expected: Option<u64>,
        now: i64,
    ) -> AppResult<()> {
        let changed = if let Some(version) = expected {
            transaction.execute("UPDATE playlists SET updated_at = ?2, version = version + 1 WHERE id = ?1 AND version = ?3", params![id, now, version])?
        } else {
            transaction.execute(
                "UPDATE playlists SET updated_at = ?2, version = version + 1 WHERE id = ?1",
                params![id, now],
            )?
        };
        if changed == 0 {
            if expected.is_some() {
                Err(AppError::PlaylistConflict)
            } else {
                Err(AppError::PlaylistNotFound)
            }
        } else {
            Ok(())
        }
    }
}

fn sortable(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .nfkd()
                .filter(|character| !is_combining_mark(*character))
                .flat_map(char::to_lowercase)
                .collect()
        })
}

fn compare_optional_text(left: Option<&str>, right: Option<&str>) -> std::cmp::Ordering {
    match (sortable(left), sortable(right)) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

fn compare_playlist_tracks(
    left: &(Track, i64, i64),
    right: &(Track, i64, i64),
    playlist: &Playlist,
) -> std::cmp::Ordering {
    let ascending = match playlist.sort_mode {
        PlaylistSortMode::Custom => left.1.cmp(&right.1),
        PlaylistSortMode::RecentlyAdded => left.2.cmp(&right.2).then_with(|| left.1.cmp(&right.1)),
        PlaylistSortMode::Title => compare_optional_text(Some(&left.0.title), Some(&right.0.title))
            .then_with(|| left.0.id.cmp(&right.0.id)),
        PlaylistSortMode::Artist => {
            compare_optional_text(left.0.artist.as_deref(), right.0.artist.as_deref())
                .then_with(|| compare_optional_text(Some(&left.0.title), Some(&right.0.title)))
                .then_with(|| left.0.id.cmp(&right.0.id))
        }
        PlaylistSortMode::Album => {
            compare_optional_text(left.0.album.as_deref(), right.0.album.as_deref())
                .then_with(|| left.0.disc_number.cmp(&right.0.disc_number))
                .then_with(|| left.0.track_number.cmp(&right.0.track_number))
                .then_with(|| compare_optional_text(Some(&left.0.title), Some(&right.0.title)))
                .then_with(|| left.0.id.cmp(&right.0.id))
        }
    };
    if playlist.sort_direction == SortDirection::Descending
        && playlist.sort_mode != PlaylistSortMode::Custom
    {
        ascending.reverse()
    } else {
        ascending
    }
}

impl PlaylistRepository for SqlitePlaylistRepository {
    fn create(&mut self, id: &str, name: &str, now: i64) -> AppResult<Playlist> {
        self.connection.execute(
            "INSERT INTO playlists (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
            params![id, name, now],
        )?;
        Self::get_playlist(&self.connection, id)
    }
    fn list(&self) -> AppResult<Vec<Playlist>> {
        let mut statement = self.connection.prepare(&format!(
            "{} GROUP BY p.id ORDER BY p.updated_at DESC, p.name COLLATE NOCASE",
            Self::base_query()
        ))?;
        let playlists = statement
            .query_map([], Self::read_playlist)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(playlists)
    }
    fn details(&self, id: &str) -> AppResult<PlaylistDetails> {
        let playlist = Self::get_playlist(&self.connection, id)?;
        let tracks = self.ordered_tracks(&playlist)?;
        Ok(PlaylistDetails { playlist, tracks })
    }
    fn update(&mut self, id: &str, input: &PlaylistUpdate, now: i64) -> AppResult<Playlist> {
        let changed = self.connection.execute(
            "UPDATE playlists SET name=?2, description=?3, artwork_uri=?4, updated_at=?5, version=version+1 WHERE id=?1 AND version=?6",
            params![id, input.name, input.description, input.artwork_uri, now, input.expected_version],
        )?;
        if changed == 0 {
            return Err(AppError::PlaylistConflict);
        }
        Self::get_playlist(&self.connection, id)
    }
    fn delete(&mut self, id: &str) -> AppResult<()> {
        if self
            .connection
            .execute("DELETE FROM playlists WHERE id=?1", [id])?
            == 0
        {
            return Err(AppError::PlaylistNotFound);
        }
        Ok(())
    }
    fn add_tracks(
        &mut self,
        id: &str,
        track_ids: &[String],
        now: i64,
    ) -> AppResult<PlaylistDetails> {
        let transaction = self.connection.transaction()?;
        let mut position: i64 = transaction.query_row(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM playlist_tracks WHERE playlist_id=?1",
            [id],
            |row| row.get(0),
        )?;
        for track_id in track_ids {
            let changed = transaction.execute("INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position, added_at) VALUES (?1,?2,?3,?4)", params![id, track_id, position, now])?;
            position += changed as i64;
        }
        Self::bump(&transaction, id, None, now)?;
        transaction.commit()?;
        self.details(id)
    }
    fn replace_order(
        &mut self,
        id: &str,
        input: &PlaylistOrderUpdate,
        now: i64,
    ) -> AppResult<PlaylistDetails> {
        let transaction = self.connection.transaction()?;
        let existing: i64 = transaction.query_row(
            "SELECT COUNT(*) FROM playlist_tracks WHERE playlist_id=?1",
            [id],
            |row| row.get(0),
        )?;
        let added_at_by_track = {
            let mut statement = transaction
                .prepare("SELECT track_id, added_at FROM playlist_tracks WHERE playlist_id=?1")?;
            let values = statement
                .query_map([id], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
                })?
                .collect::<Result<std::collections::HashMap<_, _>, _>>()?;
            values
        };
        for track_id in &input.track_ids {
            let belongs: i64 = transaction.query_row(
                "SELECT COUNT(*) FROM playlist_tracks WHERE playlist_id=?1 AND track_id=?2",
                params![id, track_id],
                |row| row.get(0),
            )?;
            if belongs == 0 {
                return Err(AppError::InvalidPlaylist(
                    "música não pertence à playlist".into(),
                ));
            }
        }
        transaction.execute("DELETE FROM playlist_tracks WHERE playlist_id=?1", [id])?;
        for (position, track_id) in input.track_ids.iter().enumerate() {
            let added_at = added_at_by_track.get(track_id).copied().unwrap_or(now);
            transaction.execute("INSERT INTO playlist_tracks (playlist_id, track_id, position, added_at) VALUES (?1,?2,?3,?4)", params![id, track_id, position as i64, added_at])?;
        }
        if existing < input.track_ids.len() as i64 {
            return Err(AppError::InvalidPlaylist(
                "sequência contém músicas extras".into(),
            ));
        }
        transaction.execute(
            "UPDATE playlists SET sort_mode='custom', sort_direction='ascending' WHERE id=?1",
            [id],
        )?;
        Self::bump(&transaction, id, Some(input.expected_version), now)?;
        transaction.commit()?;
        self.details(id)
    }
    fn set_sort(
        &mut self,
        id: &str,
        mode: PlaylistSortMode,
        direction: SortDirection,
        now: i64,
    ) -> AppResult<PlaylistDetails> {
        let changed = self.connection.execute("UPDATE playlists SET sort_mode=?2, sort_direction=?3, updated_at=?4, version=version+1 WHERE id=?1", params![id, mode.as_str(), direction.as_str(), now])?;
        if changed == 0 {
            return Err(AppError::PlaylistNotFound);
        }
        self.details(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository() -> SqlitePlaylistRepository {
        let repository = SqlitePlaylistRepository::in_memory().expect("database");
        for (id, title, artist, album, added) in [
            ("t1", "Zulu", "Beta", "One", 1_i64),
            ("t2", "Alpha", "Alpha", "Two", 2_i64),
            ("t3", "Middle", "Gamma", "One", 3_i64),
        ] {
            repository.connection.execute(
                "INSERT INTO tracks (id,source_type,source_uri,title,artist,album,duration_ms,added_at,updated_at,favorite,availability)
                 VALUES (?1,'local_document',?2,?3,?4,?5,60000,?6,?6,0,'available')",
                params![id, format!("content://{id}"), title, artist, album, added],
            ).expect("track");
        }
        repository
    }

    #[test]
    fn creates_adds_and_prevents_duplicates() {
        let mut repository = repository();
        let created = repository
            .create("p1", "Minha playlist", 10)
            .expect("create");
        let details = repository
            .add_tracks("p1", &["t1".into(), "t2".into(), "t1".into()], 11)
            .expect("add");
        assert_eq!(created.name, "Minha playlist");
        assert_eq!(details.playlist.track_count, 2);
        assert_eq!(details.playlist.duration_ms, 120_000);
    }

    #[test]
    fn sorts_without_losing_custom_order() {
        let mut repository = repository();
        repository.create("p1", "Playlist", 10).expect("create");
        let custom = repository
            .add_tracks("p1", &["t1".into(), "t2".into(), "t3".into()], 11)
            .expect("add");
        assert_eq!(
            custom
                .tracks
                .iter()
                .map(|t| t.id.as_str())
                .collect::<Vec<_>>(),
            ["t1", "t2", "t3"]
        );
        let sorted = repository
            .set_sort("p1", PlaylistSortMode::Title, SortDirection::Ascending, 12)
            .expect("sort");
        assert_eq!(
            sorted
                .tracks
                .iter()
                .map(|t| t.id.as_str())
                .collect::<Vec<_>>(),
            ["t2", "t3", "t1"]
        );
        let restored = repository
            .set_sort("p1", PlaylistSortMode::Custom, SortDirection::Ascending, 13)
            .expect("custom");
        assert_eq!(
            restored
                .tracks
                .iter()
                .map(|t| t.id.as_str())
                .collect::<Vec<_>>(),
            ["t1", "t2", "t3"]
        );
    }

    #[test]
    fn sorts_text_without_case_or_accent_bias_and_keeps_empty_values_last() {
        let mut repository = repository();
        repository.connection.execute(
            "INSERT INTO tracks (id,source_type,source_uri,title,artist,album,duration_ms,added_at,updated_at,favorite,availability)
             VALUES ('t4','local_document','content://t4','Água','Érica','Álbum',60000,4,4,0,'available')",
            [],
        ).expect("accented track");
        repository.connection.execute(
            "INSERT INTO tracks (id,source_type,source_uri,title,artist,album,duration_ms,added_at,updated_at,favorite,availability)
             VALUES ('t5','local_document','content://t5','beta','','',60000,5,5,0,'available')",
            [],
        ).expect("empty metadata track");
        repository.create("p1", "Playlist", 10).expect("create");
        repository
            .add_tracks(
                "p1",
                &[
                    "t1".into(),
                    "t2".into(),
                    "t3".into(),
                    "t4".into(),
                    "t5".into(),
                ],
                11,
            )
            .expect("add");
        let by_title = repository
            .set_sort("p1", PlaylistSortMode::Title, SortDirection::Ascending, 12)
            .expect("title");
        assert_eq!(
            by_title
                .tracks
                .iter()
                .map(|track| track.id.as_str())
                .collect::<Vec<_>>(),
            ["t4", "t2", "t5", "t3", "t1"]
        );
        let by_artist = repository
            .set_sort("p1", PlaylistSortMode::Artist, SortDirection::Ascending, 13)
            .expect("artist");
        assert_eq!(
            by_artist
                .tracks
                .iter()
                .map(|track| track.id.as_str())
                .collect::<Vec<_>>(),
            ["t2", "t1", "t4", "t3", "t5"]
        );
        let by_album = repository
            .set_sort("p1", PlaylistSortMode::Album, SortDirection::Ascending, 14)
            .expect("album");
        assert_eq!(
            by_album
                .tracks
                .iter()
                .map(|track| track.id.as_str())
                .collect::<Vec<_>>(),
            ["t4", "t3", "t1", "t2", "t5"]
        );
    }

    #[test]
    fn replaces_order_removes_tracks_and_detects_conflict() {
        let mut repository = repository();
        repository.create("p1", "Playlist", 10).expect("create");
        let details = repository
            .add_tracks("p1", &["t1".into(), "t2".into(), "t3".into()], 11)
            .expect("add");
        let replaced = repository
            .replace_order(
                "p1",
                &PlaylistOrderUpdate {
                    track_ids: vec!["t3".into(), "t1".into()],
                    expected_version: details.playlist.version,
                },
                12,
            )
            .expect("replace");
        assert_eq!(
            replaced
                .tracks
                .iter()
                .map(|t| t.id.as_str())
                .collect::<Vec<_>>(),
            ["t3", "t1"]
        );
        let conflict = repository.replace_order(
            "p1",
            &PlaylistOrderUpdate {
                track_ids: vec!["t1".into()],
                expected_version: details.playlist.version,
            },
            13,
        );
        assert!(matches!(conflict, Err(AppError::PlaylistConflict)));
    }
}
