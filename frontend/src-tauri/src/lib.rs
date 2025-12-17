// src-tauri/src/lib.rs
#[tauri::command]
fn get_os_info() -> String {
    let os = std::env::consts::OS;
    format!("Running natively on {}", os)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_os_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
