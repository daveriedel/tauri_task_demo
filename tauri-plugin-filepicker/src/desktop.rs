use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Notifications<R>> {
    Ok(Notifications(app.clone()))
}

/// Access to the notifications APIs.
pub struct Notifications<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Notifications<R> {
    pub fn get_file_picker(&self, payload: FilepickerOptions) -> crate::Result<FilePickerResponse> {
        Ok(FilePickerResponse {
            files: Some(vec!["".to_string()]),
        })
    }
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
}
