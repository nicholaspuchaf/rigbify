use std::collections::HashSet;

use uuid::Uuid;

use crate::errors::{AppError, AppResult};

use super::{
    Playlist, PlaylistDetails, PlaylistOrderUpdate, PlaylistRepository, PlaylistSortMode,
    PlaylistUpdate, SortDirection,
};

pub struct PlaylistLibrary<R> {
    repository: R,
}

impl<R: PlaylistRepository> PlaylistLibrary<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn create(&mut self, name: String, now: i64) -> AppResult<Playlist> {
        let name = required(name, "nome da playlist")?;
        self.repository
            .create(&format!("playlist-{}", Uuid::new_v4()), &name, now)
    }
    pub fn list(&self) -> AppResult<Vec<Playlist>> {
        self.repository.list()
    }

    pub fn record_playback(&mut self, id: &str, now: i64) -> AppResult<()> {
        if id.trim().is_empty() {
            return Err(AppError::InvalidPlaylist("id ausente".into()));
        }
        self.repository.record_playback(id, now)
    }
    pub fn details(&self, id: &str) -> AppResult<PlaylistDetails> {
        self.repository.details(&required(id.to_string(), "id")?)
    }
    pub fn update(&mut self, id: &str, mut input: PlaylistUpdate, now: i64) -> AppResult<Playlist> {
        input.name = required(input.name, "nome da playlist")?;
        input.description = clean(input.description);
        input.artwork_uri = clean(input.artwork_uri);
        self.repository
            .update(&required(id.to_string(), "id")?, &input, now)
    }
    pub fn delete(&mut self, id: &str) -> AppResult<()> {
        self.repository.delete(&required(id.to_string(), "id")?)
    }
    pub fn add_tracks(
        &mut self,
        id: &str,
        track_ids: Vec<String>,
        now: i64,
    ) -> AppResult<PlaylistDetails> {
        let mut seen = HashSet::new();
        let unique: Vec<_> = track_ids
            .into_iter()
            .filter(|id| !id.trim().is_empty() && seen.insert(id.clone()))
            .collect();
        self.repository
            .add_tracks(&required(id.to_string(), "id")?, &unique, now)
    }
    pub fn replace_order(
        &mut self,
        id: &str,
        input: PlaylistOrderUpdate,
        now: i64,
    ) -> AppResult<PlaylistDetails> {
        if input.track_ids.iter().collect::<HashSet<_>>().len() != input.track_ids.len() {
            return Err(AppError::InvalidPlaylist(
                "músicas duplicadas na sequência".into(),
            ));
        }
        self.repository
            .replace_order(&required(id.to_string(), "id")?, &input, now)
    }
    pub fn set_sort(
        &mut self,
        id: &str,
        mode: PlaylistSortMode,
        direction: SortDirection,
        now: i64,
    ) -> AppResult<PlaylistDetails> {
        self.repository
            .set_sort(&required(id.to_string(), "id")?, mode, direction, now)
    }
}

fn clean(value: Option<String>) -> Option<String> {
    value
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}
fn required(value: String, field: &str) -> AppResult<String> {
    let value = value.trim().to_string();
    if value.is_empty() {
        Err(AppError::InvalidPlaylist(format!("{field} ausente")))
    } else {
        Ok(value)
    }
}
