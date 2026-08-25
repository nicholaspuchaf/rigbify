use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<MediaPlayer<R>> {
    Ok(MediaPlayer(app.clone()))
}

/// Access to the media-player APIs.
pub struct MediaPlayer<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MediaPlayer<R> {
    fn unsupported(&self) -> crate::Result<PlayerSnapshot> {
        Err(crate::Error::UnsupportedPlatform)
    }
    pub fn set_queue(&self, _: QueueRequest) -> crate::Result<PlayerSnapshot> {
        self.unsupported()
    }
    pub fn seek_to(&self, _: SeekRequest) -> crate::Result<PlayerSnapshot> {
        self.unsupported()
    }
    pub fn play(&self) -> crate::Result<PlayerSnapshot> {
        self.unsupported()
    }
    pub fn pause(&self) -> crate::Result<PlayerSnapshot> {
        self.unsupported()
    }
    pub fn stop(&self) -> crate::Result<PlayerSnapshot> {
        self.unsupported()
    }
    pub fn skip_next(&self) -> crate::Result<PlayerSnapshot> {
        self.unsupported()
    }
    pub fn skip_previous(&self) -> crate::Result<PlayerSnapshot> {
        self.unsupported()
    }
    pub fn get_state(&self) -> crate::Result<PlayerSnapshot> {
        self.unsupported()
    }
}
