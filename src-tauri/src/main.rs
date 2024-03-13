// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;
mod keyboard_layout;

use data::ProfilGame;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_default_profil])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_default_profil() -> ProfilGame {
    println!("create_default_profil");
    let profil = ProfilGame::new("test Game");
    profil
}
