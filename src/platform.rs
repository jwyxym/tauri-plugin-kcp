use std::{collections::HashMap, io, net::SocketAddr, sync::Arc};

use debug_print::debug_println;
use tokio_kcp::{KcpConfig, KcpListener, KcpStream};
use lazy_static::lazy_static;
use tauri::{Emitter, Manager, Runtime};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, sync::{Mutex, RwLock}, task::JoinHandle};

use crate::models::{Kcp, Payload};

lazy_static! {
    static ref SESSIONS: RwLock<HashMap<String, Kcp>> = RwLock::new(HashMap::new());
}

fn parse_addr(value: &str) -> io::Result<SocketAddr> {
    value.parse().map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))
}

fn kcp_error(error: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, error.to_string())
}

async fn receive<R: Runtime>(window: tauri::Window<R>, id: String, mut reader: tokio::io::ReadHalf<KcpStream>) {
    let mut buffer = vec![0u8; 64 * 1024];
    loop {
        match reader.read(&mut buffer).await {
            Ok(0) | Err(_) => {
                let _ = window.app_handle().emit_to(window.label(), "plugin://kcp", Payload {
                    id: id.clone(),
                    event: "closed".into(),
                    data: vec![],
                });
                break;
            }
            Ok(len) => {
                let _ = window.app_handle().emit_to(window.label(), "plugin://kcp", Payload {
                    id: id.clone(),
                    event: "data".into(),
                    data: buffer[..len].to_vec(),
                });
            }
        }
    }
}

fn spawn_receive<R: Runtime>(window: tauri::Window<R>, id: String, reader: tokio::io::ReadHalf<KcpStream>) -> JoinHandle<()> {
    tokio::spawn(receive(window, id, reader))
}

pub async fn connect<R: Runtime>(window: tauri::Window<R>, id: String, remote: String) -> io::Result<()> {
    close_session(id.clone()).await.ok();
    let config = KcpConfig::default();
    let stream = KcpStream::connect(&config, parse_addr(&remote)?).await.map_err(kcp_error)?;
    let (reader, writer) = tokio::io::split(stream);
    let writer = Arc::new(Mutex::new(writer));
    let task = spawn_receive(window, id.clone(), reader);
    SESSIONS.write().await.insert(id, Kcp { task, writer: Some(writer) });
    Ok(())
}

pub async fn listen<R: Runtime>(window: tauri::Window<R>, id: String, bind_at: String) -> io::Result<()> {
    close_session(id.clone()).await.ok();
    let config = KcpConfig::default();
    let listener = KcpListener::bind(config, bind_at).await.map_err(kcp_error)?;
    let listener_id = id.clone();
    let task = tokio::spawn(async move {
        let mut listener = listener;
        if let Ok((stream, remote)) = listener.accept().await {
            debug_println!("{} accepted KCP connection from {}", listener_id, remote);
            let (reader, writer) = tokio::io::split(stream);
            let writer = Arc::new(Mutex::new(writer));
            let receive_task = spawn_receive(window, listener_id.clone(), reader);
            SESSIONS.write().await.insert(listener_id, Kcp { task: receive_task, writer: Some(writer) });
        }
    });
    SESSIONS.write().await.insert(id, Kcp { task, writer: None });
    Ok(())
}

async fn close_session(id: String) -> io::Result<()> {
    if let Some(session) = SESSIONS.write().await.remove(&id) {
        session.task.abort();
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, format!("ID {} not found.", id)))
    }
}

pub async fn close<R: Runtime>(window: tauri::Window<R>, id: String) -> io::Result<()> {
    let result = close_session(id.clone()).await;
    if result.is_ok() {
        let _ = window.app_handle().emit_to(
            window.label(),
            "plugin://kcp",
            Payload {
                id,
                event: "closed".into(),
                data: vec![],
            },
        );
    }
    result
}

pub async fn send(id: String, message: Vec<u8>) -> io::Result<()> {
    let sessions = SESSIONS.read().await;
    let session = sessions.get(&id).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("ID {} not found.", id)))?;
    let writer = session.writer.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "KCP listener has not accepted a connection"))?.clone();
    drop(sessions);
    let result = writer.lock().await.write_all(&message).await;
    result
}
