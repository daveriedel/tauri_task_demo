use std::path::Path;

use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::NotificationsExt;
use crate::Result;

#[command]
pub(crate) async fn get_file_picker<R: Runtime>(
    app: AppHandle<R>,
    payload: FilepickerOptions,
) -> Result<FilePickerResponse> {
    app.notifications().get_file_picker(payload)
}

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.notifications().ping(payload)
}
