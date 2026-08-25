use crate::errors::{AppError, AppResult};

use super::{NewTrack, Track, TrackRepository};

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

    pub fn remove(&mut self, id: &str) -> AppResult<()> {
        if id.trim().is_empty() {
            return Err(AppError::InvalidTrack("id ausente".into()));
        }
        self.repository.remove(id)
    }
}
