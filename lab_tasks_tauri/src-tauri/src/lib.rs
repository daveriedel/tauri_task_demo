use tauri_plugin_dialer::{DialRequest, DialerExt};
use tauri_plugin_filepicker::{FilePickerResponse, FilepickerOptions, NotificationsExt};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_file_picker(app: tauri::AppHandle) -> FilePickerResponse {
    let plugin = app.notifications();
    let options = FilepickerOptions {
        multiple: Some(false),
    };
    match plugin.get_file_picker(options) {
        Ok(file) => {
            println!("File: {:?}", file);
            return file;
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    };
    FilePickerResponse {
        files: Some(vec!["".to_string()]),
    }
}

#[tauri::command]
fn open_dialer(app: tauri::AppHandle, tel: String) {
    let plugin = app.dialer();
    let dial_request = DialRequest { tel };
    plugin.dial(dial_request);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    match tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_filepicker::init())
        .plugin(tauri_plugin_dialer::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            open_file_picker,
            open_dialer
        ])
        .run(tauri::generate_context!())
    {
        Ok(_app) => {}
        Err(e) => eprintln!("Error boot: {}", e),
    }
}
