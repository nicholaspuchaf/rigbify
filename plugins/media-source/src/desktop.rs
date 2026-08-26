use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<MediaSource<R>> {
    Ok(MediaSource(app.clone()))
}

/// Access to the media-source APIs.
pub struct MediaSource<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MediaSource<R> {
    pub fn pick_audio(&self) -> crate::Result<Vec<SelectedTrack>> {
        Err(crate::Error::UnsupportedPlatform)
    }

    pub fn import_bundle(&self) -> crate::Result<BundleImport> {
        Err(crate::Error::UnsupportedPlatform)
    }
}
