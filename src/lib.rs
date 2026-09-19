use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;
pub mod error;
pub mod models;

pub mod platform;

const PLUGIN_NAME: &str = "kcp";

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            commands::connect,
            commands::listen,
            commands::close,
            commands::send
        ])
        .build()
}
