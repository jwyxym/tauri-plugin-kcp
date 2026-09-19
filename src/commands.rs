use tauri::Runtime;

use crate::error::Result;

use crate::platform;


#[tauri::command]
pub async fn connect<R: Runtime>(
    window: tauri::Window<R>,
    id: String,
    remote: String,
) -> Result<()> {
    platform::connect(window, id, remote).await.map_err(|e| e.into())
}

#[tauri::command]
pub async fn listen<R: Runtime>(window: tauri::Window<R>, id: String, bind_at: String) -> Result<()> {
    platform::listen(window, id, bind_at).await.map_err(|e| e.into())
}

#[tauri::command]
pub async fn close(id: String) -> Result<()> {
    platform::close(id).await.map_err(|e| e.into())
}

#[tauri::command]
pub async fn send(id: String, message: Vec<u8>) -> Result<()> {
    platform::send(id, message).await.map_err(|e| e.into())
}
