use crate::errors::AppResult;

use super::Track;

pub trait TrackRepository: Send {
    fn upsert(&mut self, track: &Track) -> AppResult<Track>;
    fn list(&self) -> AppResult<Vec<Track>>;
    fn find_by_id(&self, id: &str) -> AppResult<Option<Track>>;
    fn set_favorite(&mut self, id: &str, favorite: bool, updated_at: i64) -> AppResult<Track>;
    fn remove(&mut self, id: &str) -> AppResult<()>;
}
