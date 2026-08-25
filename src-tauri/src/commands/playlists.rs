use crate::{
    database::SqlitePlaylistRepository,
    media::{
        Playlist, PlaylistDetails, PlaylistLibrary, PlaylistOrderUpdate, PlaylistSortMode,
        PlaylistUpdate, SortDirection,
    },
};
use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::State;

pub struct PlaylistState(pub Mutex<PlaylistLibrary<SqlitePlaylistRepository>>);

fn now_ms() -> Result<i64, String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?;
    i64::try_from(duration.as_millis()).map_err(|_| "data fora do intervalo".to_string())
}
fn with_playlists<T>(
    state: &State<'_, PlaylistState>,
    operation: impl FnOnce(
        &mut PlaylistLibrary<SqlitePlaylistRepository>,
        i64,
    ) -> crate::errors::AppResult<T>,
) -> Result<T, String> {
    let mut playlists = state
        .0
        .lock()
        .map_err(|_| "playlists indisponíveis".to_string())?;
    operation(&mut playlists, now_ms()?).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn create_playlist(name: String, state: State<'_, PlaylistState>) -> Result<Playlist, String> {
    with_playlists(&state, |library, now| library.create(name, now))
}
#[tauri::command]
pub fn list_playlists(state: State<'_, PlaylistState>) -> Result<Vec<Playlist>, String> {
    with_playlists(&state, |library, _| library.list())
}
#[tauri::command]
pub fn get_playlist(
    id: String,
    state: State<'_, PlaylistState>,
) -> Result<PlaylistDetails, String> {
    with_playlists(&state, |library, _| library.details(&id))
}
#[tauri::command]
pub fn update_playlist(
    id: String,
    input: PlaylistUpdate,
    state: State<'_, PlaylistState>,
) -> Result<Playlist, String> {
    with_playlists(&state, |library, now| library.update(&id, input, now))
}
#[tauri::command]
pub fn delete_playlist(id: String, state: State<'_, PlaylistState>) -> Result<(), String> {
    with_playlists(&state, |library, _| library.delete(&id))
}
#[tauri::command]
pub fn add_tracks_to_playlist(
    id: String,
    track_ids: Vec<String>,
    state: State<'_, PlaylistState>,
) -> Result<PlaylistDetails, String> {
    with_playlists(&state, |library, now| {
        library.add_tracks(&id, track_ids, now)
    })
}
#[tauri::command]
pub fn replace_playlist_order(
    id: String,
    input: PlaylistOrderUpdate,
    state: State<'_, PlaylistState>,
) -> Result<PlaylistDetails, String> {
    with_playlists(&state, |library, now| {
        library.replace_order(&id, input, now)
    })
}
#[tauri::command]
pub fn set_playlist_sort(
    id: String,
    mode: PlaylistSortMode,
    direction: SortDirection,
    state: State<'_, PlaylistState>,
) -> Result<PlaylistDetails, String> {
    with_playlists(&state, |library, now| {
        library.set_sort(&id, mode, direction, now)
    })
}
