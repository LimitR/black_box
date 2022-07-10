#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod stream;

use home::home_dir;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        // !
        .invoke_handler(tauri::generate_handler![add_task, hello_line])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("Ошибка при запуске приложения");
}

#[tauri::command]
fn add_task(text: String) {
    // !
    let mut path = home_dir().expect("Ошибка доступа к домашней директории");
    // добавляем в путь название файла для заметок
    path.push("tasks.txt");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        // !
        .open(path)
        .expect("Ошибка при открытии файла");

    writeln!(file, "{text}").expect("Ошибка при записи файла");
}

#[tauri::command]
fn hello_line() {
    println!("Hello!");
}
