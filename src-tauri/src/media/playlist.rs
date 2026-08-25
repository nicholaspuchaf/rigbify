use serde::{Deserialize, Serialize};

use super::Track;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaylistSortMode {
    Custom,
    Title,
    Artist,
    Album,
    RecentlyAdded,
}

impl PlaylistSortMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Title => "title",
            Self::Artist => "artist",
            Self::Album => "album",
            Self::RecentlyAdded => "recently_added",
        }
    }

    pub fn parse(value: &str) -> Self {
        match value {
            "title" => Self::Title,
            "artist" => Self::Artist,
            "album" => Self::Album,
            "recently_added" => Self::RecentlyAdded,
            _ => Self::Custom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        }
    }
    pub fn parse(value: &str) -> Self {
        if value == "descending" {
            Self::Descending
        } else {
            Self::Ascending
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub artwork_uri: Option<String>,
    pub sort_mode: PlaylistSortMode,
    pub sort_direction: SortDirection,
    pub created_at: i64,
    pub updated_at: i64,
    pub version: u64,
    pub track_count: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistDetails {
    #[serde(flatten)]
    pub playlist: Playlist,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistUpdate {
    pub name: String,
    pub description: Option<String>,
    pub artwork_uri: Option<String>,
    pub expected_version: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistOrderUpdate {
    pub track_ids: Vec<String>,
    pub expected_version: u64,
}
