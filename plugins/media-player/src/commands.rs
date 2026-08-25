use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::MediaPlayerExt;
use crate::Result;

macro_rules! player_command {
    ($name:ident) => {
        #[command]
        pub(crate) async fn $name<R: Runtime>(app: AppHandle<R>) -> Result<PlayerSnapshot> {
            app.media_player().$name()
        }
    };
}

player_command!(play);
player_command!(pause);
player_command!(stop);
player_command!(skip_next);
player_command!(skip_previous);
player_command!(get_state);

#[command]
pub(crate) async fn set_queue<R: Runtime>(
    app: AppHandle<R>,
    payload: QueueRequest,
) -> Result<PlayerSnapshot> {
    app.media_player().set_queue(payload)
}

#[command]
pub(crate) async fn seek_to<R: Runtime>(
    app: AppHandle<R>,
    payload: SeekRequest,
) -> Result<PlayerSnapshot> {
    app.media_player().seek_to(payload)
}
