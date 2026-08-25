const COMMANDS: &[&str] = &[
    "set_queue",
    "play",
    "pause",
    "seek_to",
    "skip_next",
    "skip_previous",
    "stop",
    "get_state",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
