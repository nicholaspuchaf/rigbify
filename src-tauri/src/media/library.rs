use crate::errors::{AppError, AppResult};

use super::{NewTrack, Track, TrackMetadataUpdate, TrackRepository};

pub struct MediaLibrary<R> {
    repository: R,
}

impl<R: TrackRepository> MediaLibrary<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn import(&mut self, input: NewTrack, now_ms: i64) -> AppResult<Track> {
        let track = input.normalize(now_ms)?;
        self.repository.upsert(&track)
    }

    pub fn list(&self) -> AppResult<Vec<Track>> {
        self.repository.list()
    }

    pub fn set_favorite(&mut self, id: &str, favorite: bool, now_ms: i64) -> AppResult<Track> {
        if id.trim().is_empty() {
            return Err(AppError::InvalidTrack("id ausente".into()));
        }
        self.repository.set_favorite(id, favorite, now_ms)
    }

    pub fn record_playback(&mut self, id: &str, now_ms: i64) -> AppResult<Track> {
        if id.trim().is_empty() {
            return Err(AppError::InvalidTrack("id ausente".into()));
        }
        self.repository.record_playback(id, now_ms)
    }

    pub fn remove(&mut self, id: &str) -> AppResult<()> {
        if id.trim().is_empty() {
            return Err(AppError::InvalidTrack("id ausente".into()));
        }
        self.repository.remove(id)
    }

    pub fn update_metadata(
        &mut self,
        id: &str,
        mut metadata: TrackMetadataUpdate,
        now_ms: i64,
    ) -> AppResult<Track> {
        if id.trim().is_empty() || metadata.title.trim().is_empty() {
            return Err(AppError::InvalidTrack(
                "título e id são obrigatórios".into(),
            ));
        }
        metadata.title = metadata.title.trim().to_string();
        metadata.artist = metadata.artist.and_then(|value| {
            let value = value.trim().to_string();
            (!value.is_empty()).then_some(value)
        });
        metadata.album = metadata.album.and_then(|value| {
            let value = value.trim().to_string();
            (!value.is_empty()).then_some(value)
        });
        self.repository.update_metadata(id, &metadata, now_ms)
    }

    pub fn add_to_playlist(
        &mut self,
        track_id: &str,
        playlist_id: &str,
        playlist_name: &str,
        now_ms: i64,
    ) -> AppResult<()> {
        if track_id.trim().is_empty()
            || playlist_id.trim().is_empty()
            || playlist_name.trim().is_empty()
        {
            return Err(AppError::InvalidTrack(
                "música e playlist são obrigatórias".into(),
            ));
        }
        self.repository
            .add_to_playlist(track_id, playlist_id, playlist_name.trim(), now_ms)
    }
}
