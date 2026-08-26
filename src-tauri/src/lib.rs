#[cfg(feature = "app")]
mod commands;
pub mod database;
pub mod errors;
pub mod media;

#[cfg(feature = "app")]
use std::sync::Mutex;

#[cfg(feature = "app")]
use commands::{
    add_track_to_playlist, add_tracks_to_playlist, cancel_media_download, create_playlist,
    delete_playlist, get_download_media_info, get_media_downloader_version, get_media_download_status, get_playlist,
    import_track, list_playlists, list_tracks, register_completed_download, remove_track,
    replace_playlist_order, set_playlist_sort, set_track_favorite, start_media_download,
    update_media_downloader, update_playlist, update_track_metadata, LibraryState, PlaylistState,
};
#[cfg(feature = "app")]
use database::{SqlitePlaylistRepository, SqliteTrackRepository};
#[cfg(feature = "app")]
use media::{MediaLibrary, PlaylistLibrary};
#[cfg(feature = "app")]
use tauri::Manager;

#[cfg(feature = "app")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_media_source::init())
        .plugin(tauri_plugin_media_player::init())
        .plugin(tauri_plugin_media_downloader::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let repository = SqliteTrackRepository::open(&data_dir.join("rigbyfy.sqlite3"))
                .map_err(|error| std::io::Error::other(error.to_string()))?;
            app.manage(LibraryState(Mutex::new(MediaLibrary::new(repository))));
            let playlist_repository =
                SqlitePlaylistRepository::open(&data_dir.join("rigbyfy.sqlite3"))
                    .map_err(|error| std::io::Error::other(error.to_string()))?;
            app.manage(PlaylistState(Mutex::new(PlaylistLibrary::new(
                playlist_repository,
            ))));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            import_track,
            list_tracks,
            set_track_favorite,
            remove_track,
            update_track_metadata,
            add_track_to_playlist,
            create_playlist,
            list_playlists,
            get_playlist,
            update_playlist,
            delete_playlist,
            add_tracks_to_playlist,
            replace_playlist_order,
            set_playlist_sort,
            get_download_media_info,
            start_media_download,
            cancel_media_download,
            get_media_download_status,
            get_media_downloader_version,
            update_media_downloader,
            register_completed_download
        ])
        .run(tauri::generate_context!())
        .expect("error while running Rigbyfy");
}
