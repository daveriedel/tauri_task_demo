use std::error::Error;

use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_notifications);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Notifications<R>> {
    #[cfg(target_os = "android")]
    println!("Initializing the Android plugin");
    let handle = api.register_android_plugin("com.plugin.filepicker", "FilepickerPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_notifications)?;
    Ok(Notifications(handle))
}

/// Access to the notifications APIs.
pub struct Notifications<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Notifications<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        self.0
            .run_mobile_plugin("ping", payload)
            .map_err(Into::into)
    }
    pub fn get_file_picker(&self, payload: FilepickerOptions) -> crate::Result<FilePickerResponse> {
        println!("Running file picker");
        let res: crate::Result<FilePickerResponse> = self
            .0
            .run_mobile_plugin("get_file_picker", payload)
            .map_err(Into::into);

        match res {
            Ok(mut res) => {
                let files = res.files.unwrap_or_default();
                let mut file_names: Vec<String> = vec![];
                for file in files {
                    let parts = file.split("/");
                    file_names.push(parts.last().unwrap().to_string());
                }
                return Ok(FilePickerResponse {
                    files: Some(file_names.clone()),
                });
            }
            Err(e) => {
                println!("Error {}", e);
            }
        };
        Ok(FilePickerResponse {
            files: Some(vec!["file picker response".to_string()]),
        })
    }
}
