use serde::{Deserialize, Serialize};
use tokio_kcp::KcpStream;
use tokio::{io::WriteHalf, sync::Mutex, task::JoinHandle};
use std::sync::Arc;

pub(crate) struct Kcp {
  pub task: JoinHandle<()>,
  pub writer: Option<Arc<Mutex<WriteHalf<KcpStream>>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Payload {
  pub id: String,
  pub event: String,
  pub data: Vec<u8>,
}
