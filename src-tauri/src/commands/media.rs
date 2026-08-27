use std::{
    fs,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime, State};
use tauri_plugin_media_source::MediaSourceExt;

use super::playlists::PlaylistState;
use crate::{
    database::SqliteTrackRepository,
    media::{MediaLibrary, NewTrack, SourceType, Track, TrackMetadataUpdate},
};

pub struct LibraryState(pub Mutex<MediaLibrary<SqliteTrackRepository>>);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleImportResult {
    imported_tracks: usize,
    playlists_updated: usize,
    missing_files: usize,
}

fn now_ms() -> Result<i64, String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("relógio do sistema inválido: {error}"))?;
    i64::try_from(duration.as_millis()).map_err(|_| "data fora do intervalo suportado".to_string())
}

fn with_library<T>(
    state: &State<'_, LibraryState>,
    operation: impl FnOnce(
        &mut MediaLibrary<SqliteTrackRepository>,
    ) -> Result<T, crate::errors::AppError>,
) -> Result<T, String> {
    let mut library = state
        .0
        .lock()
        .map_err(|_| "biblioteca indisponível".to_string())?;
    operation(&mut library).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn import_track(input: NewTrack, state: State<'_, LibraryState>) -> Result<Track, String> {
    with_library(&state, |library| {
        library.import(input, now_ms().map_err(crate::errors::AppError::Storage)?)
    })
}

#[tauri::command]
pub fn import_bundle<R: Runtime>(
    app: AppHandle<R>,
    library_state: State<'_, LibraryState>,
    playlist_state: State<'_, PlaylistState>,
) -> Result<BundleImportResult, String> {
    let bundle = app
        .media_source()
        .import_bundle()
        .map_err(|error| error.to_string())?;
    let now = now_ms()?;
    let mut track_ids = std::collections::HashMap::new();
    let mut library = library_state
        .0
        .lock()
        .map_err(|_| "biblioteca indisponível".to_string())?;
    for input in bundle.tracks {
        let source_type =
            SourceType::parse(&input.source_type).map_err(|error| error.to_string())?;
        let external_id = input.external_id.clone();
        let stored = library
            .import(
                NewTrack {
                    source_type,
                    source_uri: input.source_uri,
                    source_page_url: None,
                    external_id,
                    title: Some(input.title),
                    file_name: input.file_name,
                    artist: input.artist,
                    album: input.album,
                    duration_ms: input.duration_ms,
                    artwork_uri: input.artwork_uri,
                    mime_type: input.mime_type,
                    file_size: input.file_size,
                    track_number: input.track_number,
                    disc_number: input.disc_number,
                    year: input.year,
                    genre: input.genre,
                },
                now,
            )
            .map_err(|error| error.to_string())?;
        if let Some(key) = stored.external_id.clone() {
            track_ids.insert(key, stored.id);
        }
    }
    drop(library);

    let mut playlists = playlist_state
        .0
        .lock()
        .map_err(|_| "playlists indisponíveis".to_string())?;
    let existing = playlists.list().map_err(|error| error.to_string())?;
    let mut playlists_updated = 0;
    for source_playlist in bundle.playlists {
        let target = existing
            .iter()
            .find(|playlist| {
                playlist
                    .name
                    .eq_ignore_ascii_case(source_playlist.name.trim())
            })
            .cloned()
            .map(Ok)
            .unwrap_or_else(|| playlists.create(source_playlist.name, now))
            .map_err(|error| error.to_string())?;
        let ids: Vec<String> = source_playlist
            .track_keys
            .iter()
            .filter_map(|key| track_ids.get(key).cloned())
            .collect();
        if !ids.is_empty() {
            playlists
                .add_tracks(&target.id, ids, now)
                .map_err(|error| error.to_string())?;
            playlists_updated += 1;
        }
    }
    Ok(BundleImportResult {
        imported_tracks: track_ids.len(),
        playlists_updated,
        missing_files: bundle.missing_files,
    })
}

#[tauri::command]
pub fn list_tracks(state: State<'_, LibraryState>) -> Result<Vec<Track>, String> {
    with_library(&state, |library| library.list())
}

#[tauri::command]
pub fn record_playback(
    id: String,
    playlist_id: Option<String>,
    library_state: State<'_, LibraryState>,
    playlist_state: State<'_, PlaylistState>,
) -> Result<Track, String> {
    let played_at = now_ms()?;
    let track = with_library(&library_state, |library| {
        library.record_playback(&id, played_at)
    })?;
    if let Some(playlist_id) = playlist_id.filter(|value| !value.trim().is_empty()) {
        let mut playlists = playlist_state
            .0
            .lock()
            .map_err(|_| "playlists indisponíveis".to_string())?;
        playlists
            .record_playback(&playlist_id, played_at)
            .map_err(|error| error.to_string())?;
    }
    Ok(track)
}

#[tauri::command]
pub fn load_artwork_data_uri<R: Runtime>(
    app: AppHandle<R>,
    uri: String,
) -> Result<Option<String>, String> {
    let Ok(url) = url::Url::parse(&uri) else {
        return Ok(None);
    };
    let Ok(path) = url.to_file_path() else {
        return Ok(None);
    };
    let artwork_dir = app
        .path()
        .app_cache_dir()
        .map_err(|error| format!("cache de capas indisponível: {error}"))?
        .join("artwork");
    let Ok(artwork_dir) = artwork_dir.canonicalize() else {
        return Ok(None);
    };
    let Ok(path) = path.canonicalize() else {
        return Ok(None);
    };
    if !path.starts_with(&artwork_dir) {
        return Ok(None);
    }
    let bytes = fs::read(path).map_err(|error| format!("não foi possível ler a capa: {error}"))?;
    if bytes.is_empty() || bytes.len() > 10 * 1024 * 1024 {
        return Ok(None);
    }
    Ok(Some(format!(
        "data:{};base64,{}",
        artwork_mime_type(&bytes),
        STANDARD.encode(bytes)
    )))
}

fn artwork_mime_type(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        "image/jpeg"
    } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        "image/png"
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        "image/gif"
    } else if bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP") {
        "image/webp"
    } else {
        "application/octet-stream"
    }
}

#[tauri::command]
pub fn set_track_favorite(
    id: String,
    favorite: bool,
    state: State<'_, LibraryState>,
) -> Result<Track, String> {
    with_library(&state, |library| {
        library.set_favorite(
            &id,
            favorite,
            now_ms().map_err(crate::errors::AppError::Storage)?,
        )
    })
}

#[tauri::command]
pub fn remove_track(id: String, state: State<'_, LibraryState>) -> Result<(), String> {
    with_library(&state, |library| library.remove(&id))
}

#[tauri::command]
pub fn update_track_metadata(
    id: String,
    metadata: TrackMetadataUpdate,
    state: State<'_, LibraryState>,
) -> Result<Track, String> {
    with_library(&state, |library| {
        library.update_metadata(
            &id,
            metadata,
            now_ms().map_err(crate::errors::AppError::Storage)?,
        )
    })
}

#[tauri::command]
pub fn add_track_to_playlist(
    track_id: String,
    playlist_id: String,
    playlist_name: String,
    state: State<'_, LibraryState>,
) -> Result<(), String> {
    with_library(&state, |library| {
        library.add_to_playlist(
            &track_id,
            &playlist_id,
            &playlist_name,
            now_ms().map_err(crate::errors::AppError::Storage)?,
        )
    })
}
