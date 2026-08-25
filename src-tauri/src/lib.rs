#[cfg(feature = "app")]
mod commands;
pub mod database;
pub mod errors;
pub mod media;

#[cfg(feature = "app")]
use std::sync::Mutex;

#[cfg(feature = "app")]
use commands::{import_track, list_tracks, remove_track, set_track_favorite, LibraryState};
#[cfg(feature = "app")]
use database::SqliteTrackRepository;
#[cfg(feature = "app")]
use media::MediaLibrary;
#[cfg(feature = "app")]
use tauri::Manager;

#[cfg(feature = "app")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_media_source::init())
        .plugin(tauri_plugin_media_player::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let repository = SqliteTrackRepository::open(&data_dir.join("rigbyfy.sqlite3"))
                .map_err(|error| std::io::Error::other(error.to_string()))?;
            app.manage(LibraryState(Mutex::new(MediaLibrary::new(repository))));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            import_track,
            list_tracks,
            set_track_favorite,
            remove_track
        ])
        .run(tauri::generate_context!())
        .expect("error while running Rigbyfy");
}
