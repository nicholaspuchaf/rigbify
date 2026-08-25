use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    LocalDocument,
    AppFile,
    RemoteDownload,
}

impl SourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalDocument => "local_document",
            Self::AppFile => "app_file",
            Self::RemoteDownload => "remote_download",
        }
    }

    pub fn parse(value: &str) -> AppResult<Self> {
        match value {
            "local_document" => Ok(Self::LocalDocument),
            "app_file" => Ok(Self::AppFile),
            "remote_download" => Ok(Self::RemoteDownload),
            _ => Err(AppError::InvalidTrack("origem desconhecida".into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    Available,
    Missing,
    Downloading,
    Failed,
}

impl Availability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Missing => "missing",
            Self::Downloading => "downloading",
            Self::Failed => "failed",
        }
    }

    pub fn parse(value: &str) -> AppResult<Self> {
        match value {
            "available" => Ok(Self::Available),
            "missing" => Ok(Self::Missing),
            "downloading" => Ok(Self::Downloading),
            "failed" => Ok(Self::Failed),
            _ => Err(AppError::InvalidTrack(
                "disponibilidade desconhecida".into(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: String,
    pub source_type: SourceType,
    pub source_uri: String,
    pub source_page_url: Option<String>,
    pub external_id: Option<String>,
    pub title: String,
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
    pub added_at: i64,
    pub updated_at: i64,
    pub last_played_at: Option<i64>,
    pub play_count: u64,
    pub favorite: bool,
    pub availability: Availability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTrack {
    pub source_type: SourceType,
    pub source_uri: String,
    pub source_page_url: Option<String>,
    pub external_id: Option<String>,
    pub title: Option<String>,
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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMetadataUpdate {
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
}

impl NewTrack {
    pub fn normalize(self, now_ms: i64) -> AppResult<Track> {
        let source_uri = required_text(self.source_uri, "URI da origem")?;
        let fallback_title = self.file_name.as_deref().map(title_from_file_name);
        let title = clean_optional(self.title)
            .or(fallback_title)
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "Música desconhecida".to_string());

        if self.duration_ms > 24 * 60 * 60 * 1_000 {
            return Err(AppError::InvalidTrack("duração maior que 24 horas".into()));
        }

        Ok(Track {
            id: Uuid::new_v4().to_string(),
            source_type: self.source_type,
            source_uri,
            source_page_url: clean_optional(self.source_page_url),
            external_id: clean_optional(self.external_id),
            title,
            artist: clean_optional(self.artist),
            album: clean_optional(self.album),
            duration_ms: self.duration_ms,
            artwork_uri: clean_optional(self.artwork_uri),
            mime_type: clean_optional(self.mime_type),
            file_size: self.file_size,
            track_number: self.track_number.filter(|value| *value > 0),
            disc_number: self.disc_number.filter(|value| *value > 0),
            year: self.year.filter(|value| (1000..=9999).contains(value)),
            genre: clean_optional(self.genre),
            added_at: now_ms,
            updated_at: now_ms,
            last_played_at: None,
            play_count: 0,
            favorite: false,
            availability: Availability::Available,
        })
    }
}

fn required_text(value: String, field: &str) -> AppResult<String> {
    clean_optional(Some(value)).ok_or_else(|| AppError::InvalidTrack(format!("{field} ausente")))
}

fn clean_optional(value: Option<String>) -> Option<String> {
    value
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

fn title_from_file_name(file_name: &str) -> String {
    let without_extension = file_name
        .rsplit_once('.')
        .map_or(file_name, |(name, _)| name);
    without_extension
        .replace(['_', '-'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> NewTrack {
        NewTrack {
            source_type: SourceType::LocalDocument,
            source_uri: " content://audio/1 ".into(),
            source_page_url: None,
            external_id: None,
            title: None,
            file_name: Some("midnight_drive.mp3".into()),
            artist: Some("  Neon Harbour  ".into()),
            album: Some("".into()),
            duration_ms: 180_000,
            artwork_uri: None,
            mime_type: Some("audio/mpeg".into()),
            file_size: Some(4_000_000),
            track_number: Some(0),
            disc_number: Some(1),
            year: Some(2026),
            genre: None,
        }
    }

    #[test]
    fn normalizes_metadata_and_uses_file_name_as_title() {
        let track = input().normalize(42).expect("valid track");
        assert_eq!(track.title, "midnight drive");
        assert_eq!(track.artist.as_deref(), Some("Neon Harbour"));
        assert_eq!(track.album, None);
        assert_eq!(track.track_number, None);
        assert_eq!(track.disc_number, Some(1));
        assert_eq!(track.added_at, 42);
    }

    #[test]
    fn rejects_empty_source_uri() {
        let mut value = input();
        value.source_uri = "   ".into();
        assert!(matches!(value.normalize(0), Err(AppError::InvalidTrack(_))));
    }

    #[test]
    fn rejects_unreasonable_duration() {
        let mut value = input();
        value.duration_ms = 24 * 60 * 60 * 1_000 + 1;
        assert!(matches!(value.normalize(0), Err(AppError::InvalidTrack(_))));
    }
}
