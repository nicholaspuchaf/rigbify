use tauri::{plugin::{Builder, TauriPlugin}, Manager, Runtime};
pub use models::*;
#[cfg(desktop)] mod desktop;
#[cfg(mobile)] mod mobile;
mod commands; mod error; mod models;
pub use error::{Error, Result};
#[cfg(desktop)] use desktop::MediaDownloader;
#[cfg(mobile)] use mobile::MediaDownloader;

pub trait MediaDownloaderExt<R: Runtime> { fn media_downloader(&self) -> &MediaDownloader<R>; }
impl<R: Runtime, T: Manager<R>> MediaDownloaderExt<R> for T { fn media_downloader(&self) -> &MediaDownloader<R> { self.state::<MediaDownloader<R>>().inner() } }

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("media-downloader")
        .invoke_handler(tauri::generate_handler![commands::get_media_info, commands::start_download, commands::cancel_download, commands::get_download_status, commands::get_ytdlp_version, commands::update_ytdlp])
        .setup(|app, api| {
            #[cfg(mobile)] let downloader = mobile::init(app, api)?;
            #[cfg(desktop)] let downloader = desktop::init(app, api)?;
            app.manage(downloader); Ok(())
        }).build()
}
