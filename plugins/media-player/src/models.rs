use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueTrack {
    pub id: String,
    pub source_uri: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub artwork_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueRequest {
    pub tracks: Vec<QueueTrack>,
    pub start_index: usize,
    pub position_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeekRequest {
    pub position_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSnapshot {
    pub status: String,
    pub current_track_id: Option<String>,
    pub position_ms: u64,
    pub duration_ms: u64,
    pub queue_index: usize,
    pub queue_size: usize,
    pub is_playing: bool,
}
