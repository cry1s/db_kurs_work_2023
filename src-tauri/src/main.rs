// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod model;
pub mod services;

use tauri::Manager;

use crate::db::connection::establish_connection_pool;

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}
#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let connections = establish_connection_pool().await.unwrap();
  tauri::Builder::default()
    .manage(connections)  
    .invoke_handler(tauri::generate_handler![greet])
    .setup(|app| {
      #[cfg(debug_assertions)]
      {
          let window = app.get_window("main").unwrap();
          window.open_devtools();
      }
      Ok(())
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
