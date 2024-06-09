// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn get_book(code: &str) -> Result<String, Box<dyn std::error::Error>> {
  let url = format!("https://codes.cornelsen.de/codes/webcode-ws/api/v1/searchAll?searchterm={}", code);
  let resp: String = reqwest::blocking::get(&url)?.text()?;
  Ok(resp)
}

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
