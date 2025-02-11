use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_dialer);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Dialer<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.dialer", "DialerPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_dialer)?;
    Ok(Dialer(handle))
}

/// Access to the dialer APIs.
pub struct Dialer<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Dialer<R> {
    pub fn dial(&self, payload: DialRequest) {
        self.0.run_mobile_plugin::<()>("dial", payload).unwrap();
    }
}
