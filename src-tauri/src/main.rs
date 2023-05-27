// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod model;
pub mod services;

use crate::db::connection::establish_connection_pool;
use crate::services::login_service::*;
use crate::services::patient_service::*;
use crate::services::admin_services::*;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let connections = establish_connection_pool().await.unwrap();
    let authorization = Authorization {
        credentials: tokio::sync::Mutex::new(Authorized::None),
    };
    tauri::Builder::default()
        .manage(connections)
        .manage(authorization)
        .invoke_handler(tauri::generate_handler![
            // login services
            get_all_doctors,
            try_login_doctor,
            try_login_patient,
            create_patient,
            get_current_authorization,
            try_login_admin,

            // admin services
            get_all_from_table,
            update_table_row,
            delete_table_row,
            add_table_row,
        ])
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
