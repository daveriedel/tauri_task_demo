// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    println!("Hello, world!");
    lab_tasks_tauri_lib::run()
}
