pub use models::*;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;

#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Notifications;

#[cfg(mobile)]
use mobile::Notifications;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the notifications APIs.
pub trait NotificationsExt<R: Runtime> {
    fn notifications(&self) -> &Notifications<R>;
}

impl<R: Runtime, T: Manager<R>> crate::NotificationsExt<R> for T {
    fn notifications(&self) -> &Notifications<R> {
        self.state::<Notifications<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("filepicker")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::get_file_picker
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let notifications = mobile::init(app, api)?;
            #[cfg(desktop)]
            let notifications = desktop::init(app, api)?;
            app.manage(notifications);
            Ok(())
        })
        .build()
}
