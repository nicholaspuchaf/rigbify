use tauri::{command, AppHandle, Runtime};
use crate::{models::*, MediaDownloaderExt, Result};
#[command] pub(crate) async fn get_media_info<R: Runtime>(app: AppHandle<R>, request: MediaInfoRequest) -> Result<MediaInfo> { app.media_downloader().get_media_info(request) }
#[command] pub(crate) async fn start_download<R: Runtime>(app: AppHandle<R>, request: NativeDownloadRequest) -> Result<DownloadStatus> { app.media_downloader().start_download(request) }
#[command] pub(crate) async fn cancel_download<R: Runtime>(app: AppHandle<R>, download_id: String) -> Result<DownloadStatus> { app.media_downloader().cancel_download(download_id) }
#[command] pub(crate) async fn get_download_status<R: Runtime>(app: AppHandle<R>, download_id: String) -> Result<DownloadStatus> { app.media_downloader().get_download_status(download_id) }
#[command] pub(crate) async fn get_ytdlp_version<R: Runtime>(app: AppHandle<R>) -> Result<YtdlpVersion> { app.media_downloader().get_ytdlp_version() }
#[command] pub(crate) async fn update_ytdlp<R: Runtime>(app: AppHandle<R>, request: YtdlpUpdateRequest) -> Result<YtdlpUpdateResult> { app.media_downloader().update_ytdlp(request) }
