use crate::errors::AppResult;

use super::{Track, TrackMetadataUpdate};

pub trait TrackRepository: Send {
    fn upsert(&mut self, track: &Track) -> AppResult<Track>;
    fn list(&self) -> AppResult<Vec<Track>>;
    fn find_by_id(&self, id: &str) -> AppResult<Option<Track>>;
    fn set_favorite(&mut self, id: &str, favorite: bool, updated_at: i64) -> AppResult<Track>;
    fn record_playback(&mut self, id: &str, played_at: i64) -> AppResult<Track>;
    fn update_metadata(
        &mut self,
        id: &str,
        metadata: &TrackMetadataUpdate,
        updated_at: i64,
    ) -> AppResult<Track>;
    fn add_to_playlist(
        &mut self,
        track_id: &str,
        playlist_id: &str,
        playlist_name: &str,
        now: i64,
    ) -> AppResult<()>;
    fn remove(&mut self, id: &str) -> AppResult<()>;
}
