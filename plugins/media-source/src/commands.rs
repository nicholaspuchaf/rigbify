use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::MediaSourceExt;
use crate::Result;

#[command]
pub(crate) async fn pick_audio<R: Runtime>(app: AppHandle<R>) -> Result<Vec<SelectedTrack>> {
    app.media_source().pick_audio()
}
