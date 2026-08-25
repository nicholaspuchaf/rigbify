const COMMANDS: &[&str] = &["get_media_info", "start_download", "cancel_download", "get_download_status"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).android_path("android").ios_path("ios").build();
}

