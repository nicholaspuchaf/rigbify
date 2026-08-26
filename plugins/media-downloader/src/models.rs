use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfoRequest { pub url: String }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YtdlpUpdateRequest { pub channel: String }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YtdlpVersion { pub version: String, pub version_name: String }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YtdlpUpdateResult { pub status: String, pub version: String, pub version_name: String }

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaFormat {
    pub id: String,
    pub container: Option<String>,
    pub audio_codec: Option<String>,
    pub video_codec: Option<String>,
    pub bitrate: Option<f64>,
    pub filesize: Option<u64>,
    pub resolution: Option<String>,
    pub audio_only: bool,
    pub video_only: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub id: String,
    pub url: String,
    pub title: String,
    pub artist: Option<String>,
    pub channel: Option<String>,
    pub album: Option<String>,
    pub thumbnail: Option<String>,
    pub duration_ms: Option<u64>,
    pub formats: Vec<MediaFormat>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloadSourceKind { Single, Playlist }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadSource { pub kind: DownloadSourceKind, pub url: String }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat { Original, Mp3, M4a, Opus, Mp4 }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaKind { Audio, Video }

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    pub source: DownloadSource,
    pub output_format: OutputFormat,
    pub media_kind: MediaKind,
    pub quality: Option<String>,
    pub format_id: Option<String>,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub thumbnail: Option<String>,
    pub duration_ms: Option<u64>,
    pub source_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeDownloadRequest { pub job_id: String, pub download_id: String, pub options: DownloadOptions }

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadStatus {
    pub job_id: String,
    pub download_id: String,
    pub status: String,
    pub progress: f64,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub speed: Option<f64>,
    pub eta: Option<u64>,
    pub message: Option<String>,
    pub source_uri: Option<String>,
    pub file_size: Option<u64>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub thumbnail: Option<String>,
    pub duration_ms: Option<u64>,
    pub source_url: Option<String>,
    pub source_id: Option<String>,
}
