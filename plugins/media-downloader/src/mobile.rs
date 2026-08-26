use serde::de::DeserializeOwned;
use tauri::{plugin::{PluginApi, PluginHandle}, AppHandle, Runtime};
use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_media_downloader);

pub fn init<R: Runtime, C: DeserializeOwned>(_app: &AppHandle<R>, api: PluginApi<R, C>) -> crate::Result<MediaDownloader<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.rigbyfy.mediadownloader", "MediaDownloaderPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_media_downloader)?;
    Ok(MediaDownloader(handle))
}

pub struct MediaDownloader<R: Runtime>(PluginHandle<R>);
impl<R: Runtime> MediaDownloader<R> {
    pub fn get_media_info(&self, request: MediaInfoRequest) -> crate::Result<MediaInfo> { self.0.run_mobile_plugin("getMediaInfo", request).map_err(Into::into) }
    pub fn start_download(&self, request: NativeDownloadRequest) -> crate::Result<DownloadStatus> { self.0.run_mobile_plugin("startDownload", request).map_err(Into::into) }
    pub fn cancel_download(&self, download_id: String) -> crate::Result<DownloadStatus> { self.0.run_mobile_plugin("cancelDownload", serde_json::json!({ "downloadId": download_id })).map_err(Into::into) }
    pub fn get_download_status(&self, download_id: String) -> crate::Result<DownloadStatus> { self.0.run_mobile_plugin("getDownloadStatus", serde_json::json!({ "downloadId": download_id })).map_err(Into::into) }
    pub fn get_ytdlp_version(&self) -> crate::Result<YtdlpVersion> { self.0.run_mobile_plugin("getYtdlpVersion", serde_json::json!({})).map_err(Into::into) }
    pub fn update_ytdlp(&self, request: YtdlpUpdateRequest) -> crate::Result<YtdlpUpdateResult> { self.0.run_mobile_plugin("updateYtdlp", request).map_err(Into::into) }
}
