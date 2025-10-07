use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum IpcMessage {
    #[serde(rename = "editor.open_file")]
    OpenFile {
        path: PathBuf,
        line: Option<usize>,
        column: Option<usize>,
    },
    #[serde(rename = "watcher.watch_directory")]
    WatchDirectory { path: PathBuf },
    #[serde(rename = "explorer.refresh_file")]
    RefreshFile {
        path: PathBuf,
        change_type: String,
    },
    #[serde(rename = "explorer.navigate")]
    Navigate { directory: PathBuf },
    #[serde(rename = "broadcast.selection_update")]
    SelectionUpdate { files: Vec<PathBuf> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

pub struct IpcClient {
    pipe_path: String,
    _sender: mpsc::UnboundedSender<IpcMessage>,
    receiver: mpsc::UnboundedReceiver<IpcMessage>,
    next_id: u64,
    connected: bool,
}

impl IpcClient {
    pub fn new(pipe_path: String) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            pipe_path,
            _sender: sender,
            receiver,
            next_id: 1,
            connected: false,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        #[cfg(windows)]
        {
            use std::io::ErrorKind;
            use tokio::net::windows::named_pipe::ClientOptions;

            match ClientOptions::new().open(&self.pipe_path) {
                Ok(_pipe) => {
                    self.connected = true;
                    log::info!("Connected to IPC daemon at {}", self.pipe_path);
                    Ok(())
                }
                Err(e) if e.kind() == ErrorKind::NotFound => {
                    log::warn!(
                        "IPC daemon not available at {} - running in standalone mode",
                        self.pipe_path
                    );
                    self.connected = false;
                    Ok(())
                }
                Err(e) => Err(e).context("Failed to connect to IPC daemon"),
            }
        }

        #[cfg(not(windows))]
        {
            use tokio::net::UnixStream;

            match UnixStream::connect(&self.pipe_path).await {
                Ok(_stream) => {
                    self.connected = true;
                    log::info!("Connected to IPC daemon at {}", self.pipe_path);
                    Ok(())
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    log::warn!(
                        "IPC daemon not available at {} - running in standalone mode",
                        self.pipe_path
                    );
                    self.connected = false;
                    Ok(())
                }
                Err(e) => Err(e).context("Failed to connect to IPC daemon"),
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub async fn send_message(&mut self, message: IpcMessage) -> Result<()> {
        if !self.connected {
            log::debug!("Skipping IPC message - not connected: {:?}", message);
            return Ok(());
        }

        #[cfg(windows)]
        {
            use tokio::net::windows::named_pipe::ClientOptions;

            let mut pipe = ClientOptions::new()
                .open(&self.pipe_path)
                .context("Failed to open named pipe")?;

            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                id: self.next_id,
                method: match &message {
                    IpcMessage::OpenFile { .. } => "editor.open_file".to_string(),
                    IpcMessage::WatchDirectory { .. } => "watcher.watch_directory".to_string(),
                    IpcMessage::RefreshFile { .. } => "explorer.refresh_file".to_string(),
                    IpcMessage::Navigate { .. } => "explorer.navigate".to_string(),
                    IpcMessage::SelectionUpdate { .. } => "broadcast.selection_update".to_string(),
                },
                params: serde_json::to_value(&message)?,
            };

            self.next_id += 1;

            let request_str = serde_json::to_string(&request)?;
            pipe.write_all(request_str.as_bytes()).await?;
            pipe.write_all(b"\n").await?;

            log::debug!("Sent IPC message: {:?}", message);
        }

        #[cfg(not(windows))]
        {
            use tokio::net::UnixStream;

            let mut stream = UnixStream::connect(&self.pipe_path)
                .await
                .context("Failed to connect to Unix socket")?;

            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                id: self.next_id,
                method: match &message {
                    IpcMessage::OpenFile { .. } => "editor.open_file".to_string(),
                    IpcMessage::WatchDirectory { .. } => "watcher.watch_directory".to_string(),
                    IpcMessage::RefreshFile { .. } => "explorer.refresh_file".to_string(),
                    IpcMessage::Navigate { .. } => "explorer.navigate".to_string(),
                    IpcMessage::SelectionUpdate { .. } => "broadcast.selection_update".to_string(),
                },
                params: serde_json::to_value(&message)?,
            };

            self.next_id += 1;

            let request_str = serde_json::to_string(&request)?;
            stream.write_all(request_str.as_bytes()).await?;
            stream.write_all(b"\n").await?;

            log::debug!("Sent IPC message: {:?}", message);
        }

        Ok(())
    }

    pub async fn start_event_listener(
        &self,
        sender: mpsc::UnboundedSender<IpcMessage>,
    ) -> Result<()> {
        if !self.connected {
            log::debug!("Not starting IPC event listener - not connected");
            return Ok(());
        }

        let pipe_path = self.pipe_path.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::event_loop(pipe_path, sender).await {
                log::error!("IPC event listener error: {}", e);
            }
        });

        Ok(())
    }

    async fn event_loop(
        pipe_path: String,
        sender: mpsc::UnboundedSender<IpcMessage>,
    ) -> Result<()> {
        #[cfg(windows)]
        {
            use tokio::net::windows::named_pipe::ClientOptions;

            let pipe = ClientOptions::new()
                .open(&pipe_path)
                .context("Failed to open named pipe for events")?;

            let reader = BufReader::new(pipe);
            Self::process_incoming_messages(reader, sender).await
        }

        #[cfg(not(windows))]
        {
            use tokio::net::UnixStream;

            let stream = UnixStream::connect(&pipe_path)
                .await
                .context("Failed to connect to Unix socket for events")?;

            let reader = BufReader::new(stream);
            Self::process_incoming_messages(reader, sender).await
        }
    }

    async fn process_incoming_messages<R>(
        mut reader: BufReader<R>,
        sender: mpsc::UnboundedSender<IpcMessage>,
    ) -> Result<()>
    where
        R: tokio::io::AsyncRead + Unpin,
    {
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;

            if bytes_read == 0 {
                log::info!("IPC connection closed");
                break;
            }

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<JsonRpcRequest>(line) {
                Ok(request) => {
                    if let Ok(message) = serde_json::from_value::<IpcMessage>(request.params) {
                        if sender.send(message).is_err() {
                            log::error!("Failed to send IPC message to app");
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to parse IPC message: {}", e);
                }
            }
        }

        Ok(())
    }

    pub fn try_recv(&mut self) -> Option<IpcMessage> {
        self.receiver.try_recv().ok()
    }
}

pub fn open_file_in_editor(path: &Path, line: Option<usize>, column: Option<usize>) -> Result<()> {
    use std::process::Command;

    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.args(["/C", "code"]);
        c
    } else {
        Command::new("code")
    };

    let mut arg = path.display().to_string();
    if let Some(line) = line {
        arg.push_str(&format!(":{}", line));
        if let Some(column) = column {
            arg.push_str(&format!(":{}", column));
        }
    }

    cmd.arg(arg);

    match cmd.spawn() {
        Ok(_) => {
            log::info!("Opened file in editor: {}", path.display());
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to open file in editor: {}", e);
            Err(e.into())
        }
    }
}