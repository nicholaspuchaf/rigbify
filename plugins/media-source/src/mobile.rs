use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_media_source);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<MediaSource<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.rigbyfy.mediasource", "MediaSourcePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_media_source)?;
    Ok(MediaSource(handle))
}

/// Access to the media-source APIs.
pub struct MediaSource<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> MediaSource<R> {
    pub fn pick_audio(&self) -> crate::Result<Vec<SelectedTrack>> {
        self.0
            .run_mobile_plugin("pickAudio", ())
            .map_err(Into::into)
    }

    pub fn import_bundle(&self) -> crate::Result<BundleImport> {
        self.0.run_mobile_plugin("importBundle", ()).map_err(Into::into)
    }
}
