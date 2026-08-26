use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use crate::models::*;
pub fn init<R: Runtime, C: DeserializeOwned>(app: &AppHandle<R>, _api: PluginApi<R, C>) -> crate::Result<MediaDownloader<R>> { Ok(MediaDownloader(app.clone())) }
pub struct MediaDownloader<R: Runtime>(AppHandle<R>);
impl<R: Runtime> MediaDownloader<R> {
    pub fn get_media_info(&self, _: MediaInfoRequest) -> crate::Result<MediaInfo> { Err(crate::Error::UnsupportedPlatform) }
    pub fn start_download(&self, _: NativeDownloadRequest) -> crate::Result<DownloadStatus> { Err(crate::Error::UnsupportedPlatform) }
    pub fn cancel_download(&self, _: String) -> crate::Result<DownloadStatus> { Err(crate::Error::UnsupportedPlatform) }
    pub fn get_download_status(&self, _: String) -> crate::Result<DownloadStatus> { Err(crate::Error::UnsupportedPlatform) }
    pub fn get_ytdlp_version(&self) -> crate::Result<YtdlpVersion> { Err(crate::Error::UnsupportedPlatform) }
    pub fn update_ytdlp(&self, _: YtdlpUpdateRequest) -> crate::Result<YtdlpUpdateResult> { Err(crate::Error::UnsupportedPlatform) }
}
