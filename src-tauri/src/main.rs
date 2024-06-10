// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use thiserror::Error;
use tauri::InvokeError;

#[derive(Error, Debug)]
enum ReqwestError {
    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unexpected error: {0}")]
    Other(String),
}

impl From<ReqwestError> for InvokeError {
  fn from(error: ReqwestError) -> Self {
      InvokeError::from(format!("{}", error))
  }
}

#[tauri::command]
fn get_book(code: &str) -> Result<String, ReqwestError> {
  let url = format!("https://codes.cornelsen.de/codes/webcode-ws/api/v1/searchAll?searchterm={}", code);
  let resp = reqwest::blocking::get(&url)?.text()?;
  Ok(resp)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_book])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
