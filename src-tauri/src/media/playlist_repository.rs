use crate::errors::AppResult;

use super::{
    Playlist, PlaylistDetails, PlaylistOrderUpdate, PlaylistSortMode, PlaylistUpdate, SortDirection,
};

pub trait PlaylistRepository: Send {
    fn create(&mut self, id: &str, name: &str, now: i64) -> AppResult<Playlist>;
    fn list(&self) -> AppResult<Vec<Playlist>>;
    fn record_playback(&mut self, id: &str, played_at: i64) -> AppResult<()>;
    fn details(&self, id: &str) -> AppResult<PlaylistDetails>;
    fn update(&mut self, id: &str, input: &PlaylistUpdate, now: i64) -> AppResult<Playlist>;
    fn delete(&mut self, id: &str) -> AppResult<()>;
    fn add_tracks(
        &mut self,
        id: &str,
        track_ids: &[String],
        now: i64,
    ) -> AppResult<PlaylistDetails>;
    fn replace_order(
        &mut self,
        id: &str,
        input: &PlaylistOrderUpdate,
        now: i64,
    ) -> AppResult<PlaylistDetails>;
    fn set_sort(
        &mut self,
        id: &str,
        mode: PlaylistSortMode,
        direction: SortDirection,
        now: i64,
    ) -> AppResult<PlaylistDetails>;
}
