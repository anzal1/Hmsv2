#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn get_environment_variable (name: &str) -> String {
  std::env::var(name).unwrap_or_else(|_| "".to_string())
}

fn main() {
  tauri::Builder::default().invoke_handler(tauri::generate_handler![get_environment_variable])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


