use std::time::{SystemTime, UNIX_EPOCH};

use tauri::{AppHandle, Runtime, State};
use tauri_plugin_media_downloader::{
    DownloadOptions, DownloadStatus, MediaDownloaderExt, MediaInfo, MediaInfoRequest,
    NativeDownloadRequest, YtdlpUpdateRequest, YtdlpUpdateResult, YtdlpVersion,
};
use uuid::Uuid;

use crate::{
    commands::LibraryState,
    media::{validate_download_options, validate_youtube_url, NewTrack, SourceType},
};

#[tauri::command]
pub async fn get_download_media_info<R: Runtime>(
    url: String,
    app: AppHandle<R>,
) -> Result<MediaInfo, String> {
    validate_youtube_url(&url).map_err(|error| error.to_string())?;
    app.media_downloader()
        .get_media_info(MediaInfoRequest { url })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_media_downloader_version<R: Runtime>(app: AppHandle<R>) -> Result<YtdlpVersion, String> {
    app.media_downloader().get_ytdlp_version().map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn update_media_downloader<R: Runtime>(channel: String, app: AppHandle<R>) -> Result<YtdlpUpdateResult, String> {
    if channel != "stable" && channel != "nightly" { return Err("canal de atualização inválido".into()); }
    app.media_downloader().update_ytdlp(YtdlpUpdateRequest { channel }).map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn start_media_download<R: Runtime>(
    options: DownloadOptions,
    app: AppHandle<R>,
) -> Result<DownloadStatus, String> {
    validate_download_options(&options).map_err(|error| error.to_string())?;
    let request = NativeDownloadRequest {
        job_id: format!("job-{}", Uuid::new_v4()),
        download_id: format!("download-{}", Uuid::new_v4()),
        options,
    };
    app.media_downloader()
        .start_download(request)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn cancel_media_download<R: Runtime>(
    download_id: String,
    app: AppHandle<R>,
) -> Result<DownloadStatus, String> {
    validate_id(&download_id)?;
    app.media_downloader()
        .cancel_download(download_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_media_download_status<R: Runtime>(
    download_id: String,
    app: AppHandle<R>,
) -> Result<DownloadStatus, String> {
    validate_id(&download_id)?;
    app.media_downloader()
        .get_download_status(download_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn register_completed_download<R: Runtime>(
    download_id: String,
    app: AppHandle<R>,
    state: State<'_, LibraryState>,
) -> Result<crate::media::Track, String> {
    validate_id(&download_id)?;
    let status = app
        .media_downloader()
        .get_download_status(download_id)
        .map_err(|error| error.to_string())?;
    if status.status != "completed" {
        return Err("o download ainda não foi concluído".into());
    }
    let source_uri = status
        .source_uri
        .ok_or_else(|| "arquivo final ausente".to_string())?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_millis();
    let now = i64::try_from(now).map_err(|_| "data fora do intervalo".to_string())?;
    let input = NewTrack {
        source_type: SourceType::RemoteDownload,
        source_uri: source_uri.clone(),
        source_page_url: status.source_url,
        external_id: status.source_id,
        title: status.title,
        file_name: source_uri.rsplit('/').next().map(ToString::to_string),
        artist: status.artist,
        album: status.album,
        duration_ms: status.duration_ms.unwrap_or(0),
        artwork_uri: status.thumbnail,
        mime_type: mime_from_uri(&source_uri).map(ToString::to_string),
        file_size: status.file_size,
        track_number: None,
        disc_number: None,
        year: None,
        genre: None,
    };
    let mut library = state
        .0
        .lock()
        .map_err(|_| "biblioteca indisponível".to_string())?;
    library
        .import(input, now)
        .map_err(|error| error.to_string())
}

fn validate_id(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 100
        || !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
    {
        Err("identificador de download inválido".into())
    } else {
        Ok(())
    }
}

fn mime_from_uri(uri: &str) -> Option<&'static str> {
    match uri.rsplit('.').next()?.to_ascii_lowercase().as_str() {
        "mp3" => Some("audio/mpeg"),
        "m4a" => Some("audio/mp4"),
        "opus" => Some("audio/opus"),
        "webm" => Some("audio/webm"),
        "mp4" => Some("video/mp4"),
        _ => None,
    }
}
