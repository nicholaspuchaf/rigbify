#[tauri::command]
fn app_status() -> &'static str {
    "Rigbyfy native core ready"
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_status])
        .run(tauri::generate_context!())
        .expect("error while running Rigbyfy");
}
