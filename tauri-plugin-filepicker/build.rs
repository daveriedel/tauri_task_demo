const COMMANDS: &[&str] = &["ping", "get_file_picker"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
