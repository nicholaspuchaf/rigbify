use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_media_player);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<MediaPlayer<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.rigbyfy.mediaplayer", "MediaPlayerPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_media_player)?;
    Ok(MediaPlayer(handle))
}

/// Access to the media-player APIs.
pub struct MediaPlayer<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> MediaPlayer<R> {
    pub fn set_queue(&self, payload: QueueRequest) -> crate::Result<PlayerSnapshot> {
        self.0
            .run_mobile_plugin("setQueue", payload)
            .map_err(Into::into)
    }
    pub fn seek_to(&self, payload: SeekRequest) -> crate::Result<PlayerSnapshot> {
        self.0
            .run_mobile_plugin("seekTo", payload)
            .map_err(Into::into)
    }
    pub fn play(&self) -> crate::Result<PlayerSnapshot> {
        self.run("play")
    }
    pub fn pause(&self) -> crate::Result<PlayerSnapshot> {
        self.run("pause")
    }
    pub fn stop(&self) -> crate::Result<PlayerSnapshot> {
        self.run("stop")
    }
    pub fn skip_next(&self) -> crate::Result<PlayerSnapshot> {
        self.run("skipNext")
    }
    pub fn skip_previous(&self) -> crate::Result<PlayerSnapshot> {
        self.run("skipPrevious")
    }
    pub fn get_state(&self) -> crate::Result<PlayerSnapshot> {
        self.run("getState")
    }
    fn run(&self, command: &str) -> crate::Result<PlayerSnapshot> {
        self.0.run_mobile_plugin(command, ()).map_err(Into::into)
    }
}
