#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod stream;
mod user;

use rusqlite::{Connection, Result};
use serde::{self, Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use user::users::SaveProfile;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        // !
        .invoke_handler(tauri::generate_handler![save_profile])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("Ошибка при запуске приложения");
}

#[tauri::command]
fn save_profile(data: &str) {
    let db: Connection = Connection::open("./db.db").unwrap();
    db.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            token  TEXT NOT NULL,
            login  TEXT NOT NULL,
            password  TEXT NOT NULL,
            url  TEXT NOT NULL
        )",
        (),
    )
    .expect("Error to created table");
    let profile: SaveProfile = serde_json::from_str(data).unwrap();
    db.execute(
        "INSERT INTO person (token, url, login, password) VALUES (?1, ?2, ?3, ?4)",
        (profile.token, profile.url, profile.login, profile.password)
    ).expect("Error to insert profile");
}
