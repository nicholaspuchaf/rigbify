#[cfg(feature = "app")]
use tauri_plugin_media_downloader::{DownloadOptions, DownloadSourceKind};
use url::Url;

use crate::errors::{AppError, AppResult};

#[cfg(feature = "app")]
pub fn validate_download_options(options: &DownloadOptions) -> AppResult<()> {
    validate_youtube_url(&options.source.url)?;
    if !matches!(options.source.kind, DownloadSourceKind::Single) {
        return Err(AppError::InvalidDownload(
            "downloads de playlist ainda não estão habilitados".into(),
        ));
    }
    if options.title.trim().is_empty() || options.title.len() > 500 {
        return Err(AppError::InvalidDownload("título inválido".into()));
    }
    if options.source_id.trim().is_empty() || options.source_id.len() > 128 {
        return Err(AppError::InvalidDownload("ID da origem inválido".into()));
    }
    if let Some(format) = &options.format_id {
        if format.is_empty()
            || format.len() > 64
            || !format
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || ".-_".contains(c))
        {
            return Err(AppError::InvalidDownload("formato inválido".into()));
        }
    }
    const QUALITIES: &[&str] = &["best", "2160", "1440", "1080", "720", "480", "360"];
    if options
        .quality
        .as_deref()
        .is_some_and(|value| !QUALITIES.contains(&value))
    {
        return Err(AppError::InvalidDownload("qualidade inválida".into()));
    }
    Ok(())
}

pub fn validate_youtube_url(value: &str) -> AppResult<()> {
    let url =
        Url::parse(value.trim()).map_err(|_| AppError::InvalidDownload("URL inválida".into()))?;
    let host = url.host_str().unwrap_or_default().to_ascii_lowercase();
    if url.scheme() != "https"
        || !(host == "youtu.be" || host == "youtube.com" || host.ends_with(".youtube.com"))
    {
        return Err(AppError::InvalidDownload(
            "use uma URL HTTPS do YouTube ou YouTube Music".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_only_youtube_https_urls() {
        assert!(validate_youtube_url("https://youtu.be/abc").is_ok());
        assert!(validate_youtube_url("https://music.youtube.com/watch?v=abc").is_ok());
        assert!(validate_youtube_url("http://youtube.com/watch?v=abc").is_err());
        assert!(validate_youtube_url("https://youtube.com.evil.example/watch?v=abc").is_err());
    }
}
