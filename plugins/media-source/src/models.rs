use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedTrack {
    pub source_type: String,
    pub source_uri: String,
    pub title: String,
    pub file_name: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_ms: u64,
    pub artwork_uri: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    #[serde(default)]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundlePlaylist {
    pub id: String,
    pub name: String,
    pub track_keys: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleImport {
    pub tracks: Vec<SelectedTrack>,
    pub playlists: Vec<BundlePlaylist>,
    #[serde(default)]
    pub missing_files: usize,
}
