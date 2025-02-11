use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Dialer<R>> {
    Ok(Dialer(app.clone()))
}

/// Access to the dialer APIs.
pub struct Dialer<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Dialer<R> {
    pub fn dial(&self, payload: DialRequest) {
        println!("{:?}", payload);
    }
}
