use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::DialerExt;

#[command]
pub(crate) async fn dial<R: Runtime>(app: AppHandle<R>, payload: DialRequest) {
    app.dialer().dial(payload)
}
