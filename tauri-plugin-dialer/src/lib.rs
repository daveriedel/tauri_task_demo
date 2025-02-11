use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Dialer;
#[cfg(mobile)]
use mobile::Dialer;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the dialer APIs.
pub trait DialerExt<R: Runtime> {
    fn dialer(&self) -> &Dialer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::DialerExt<R> for T {
    fn dialer(&self) -> &Dialer<R> {
        self.state::<Dialer<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("dialer")
        .invoke_handler(tauri::generate_handler![commands::dial])
        .setup(|app, api| {
            #[cfg(mobile)]
            let dialer = mobile::init(app, api)?;
            #[cfg(desktop)]
            let dialer = desktop::init(app, api)?;
            app.manage(dialer);
            Ok(())
        })
        .build()
}
