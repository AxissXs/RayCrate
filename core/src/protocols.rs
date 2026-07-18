use crate::{RayCrateError, Result};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct OpenVpnManager {
    process: Arc<Mutex<Option<Child>>>,
    binary_path: String,
}

impl OpenVpnManager {
    pub fn new(binary_path: String) -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            binary_path,
        }
    }

    pub async fn start(&self, config_path: &str, auth_user_pass: Option<&str>) -> Result<()> {
        let mut proc_guard = self.process.lock().await;
        if proc_guard.is_some() {
            return Err(RayCrateError::ConfigError("OpenVPN is already running".to_string()));
        }

        info!("Starting OpenVPN from config: {}", config_path);
        let mut cmd = Command::new(&self.binary_path);
        cmd.arg("--config").arg(config_path);

        if let Some(auth) = auth_user_pass {
            cmd.arg("--auth-user-pass").arg(auth);
        }

        let child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| RayCrateError::ConfigError(format!("Failed to spawn OpenVPN: {}", e)))?;

        *proc_guard = Some(child);
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut proc_guard = self.process.lock().await;
        if let Some(mut child) = proc_guard.take() {
            info!("Stopping OpenVPN");
            child.kill()?;
            child.wait()?;
        }
        Ok(())
    }
}

pub struct OpenConnectManager {
    process: Arc<Mutex<Option<Child>>>,
    binary_path: String,
}

impl OpenConnectManager {
    pub fn new(binary_path: String) -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            binary_path,
        }
    }

    pub async fn start(&self, server: &str, username: &str) -> Result<()> {
        let mut proc_guard = self.process.lock().await;
        if proc_guard.is_some() {
            return Err(RayCrateError::ConfigError("OpenConnect is already running".to_string()));
        }

        info!("Starting OpenConnect to server: {}", server);
        let child = Command::new(&self.binary_path)
            .arg("--user")
            .arg(username)
            .arg(server)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| RayCrateError::ConfigError(format!("Failed to spawn OpenConnect: {}", e)))?;

        *proc_guard = Some(child);
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut proc_guard = self.process.lock().await;
        if let Some(mut child) = proc_guard.take() {
            info!("Stopping OpenConnect");
            child.kill()?;
            child.wait()?;
        }
        Ok(())
    }
}
