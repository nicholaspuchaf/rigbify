use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MediaPlayer;
#[cfg(mobile)]
use mobile::MediaPlayer;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the media-player APIs.
pub trait MediaPlayerExt<R: Runtime> {
    fn media_player(&self) -> &MediaPlayer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MediaPlayerExt<R> for T {
    fn media_player(&self) -> &MediaPlayer<R> {
        self.state::<MediaPlayer<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("media-player")
        .invoke_handler(tauri::generate_handler![
            commands::set_queue,
            commands::play,
            commands::pause,
            commands::seek_to,
            commands::skip_next,
            commands::skip_previous,
            commands::stop,
            commands::get_state
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let media_player = mobile::init(app, api)?;
            #[cfg(desktop)]
            let media_player = desktop::init(app, api)?;
            app.manage(media_player);
            Ok(())
        })
        .build()
}
