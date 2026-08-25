use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use tauri::State;

use crate::{
    database::SqliteTrackRepository,
    media::{MediaLibrary, NewTrack, Track, TrackMetadataUpdate},
};

pub struct LibraryState(pub Mutex<MediaLibrary<SqliteTrackRepository>>);

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
pub fn list_tracks(state: State<'_, LibraryState>) -> Result<Vec<Track>, String> {
    with_library(&state, |library| library.list())
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
