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
use desktop::MediaSource;
#[cfg(mobile)]
use mobile::MediaSource;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the media-source APIs.
pub trait MediaSourceExt<R: Runtime> {
    fn media_source(&self) -> &MediaSource<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MediaSourceExt<R> for T {
    fn media_source(&self) -> &MediaSource<R> {
        self.state::<MediaSource<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("media-source")
        .invoke_handler(tauri::generate_handler![commands::pick_audio])
        .setup(|app, api| {
            #[cfg(mobile)]
            let media_source = mobile::init(app, api)?;
            #[cfg(desktop)]
            let media_source = desktop::init(app, api)?;
            app.manage(media_source);
            Ok(())
        })
        .build()
}
