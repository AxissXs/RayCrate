use crate::{config::ProxyProfile, RayCrateError, Result};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};

pub struct XrayManager {
    process: Arc<Mutex<Option<Child>>>,
    binary_path: String,
}

impl XrayManager {
    pub fn new(binary_path: String) -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            binary_path,
        }
    }

    /// Generates Xray JSON configuration string from a ProxyProfile
    pub fn generate_config(profile: &ProxyProfile, local_port: u16) -> Result<String> {
        let config = serde_json::json!({
            "log": {
                "loglevel": "warning"
            },
            "inbounds": [
                {
                    "tag": "socks-in",
                    "port": local_port,
                    "protocol": "socks",
                    "settings": {
                        "auth": "noauth",
                        "udp": true
                    }
                },
                {
                    "tag": "tun-in",
                    "port": local_port + 1,
                    "protocol": "tun",
                    "settings": {
                        "domainStrategy": "AsIs"
                    }
                }
            ],
            "outbounds": [
                {
                    "tag": "proxy-out",
                    "protocol": match profile.protocol {
                        crate::config::ProtocolType::Vless => "vless",
                        crate::config::ProtocolType::Vmess => "vmess",
                        crate::config::ProtocolType::Trojan => "trojan",
                        crate::config::ProtocolType::Shadowsocks => "shadowsocks",
                        _ => "vless",
                    },
                    "settings": {
                        "vnext": [{
                            "address": profile.server,
                            "port": profile.port,
                            "users": [{
                                "id": profile.uuid.as_deref().unwrap_or(""),
                                "encryption": "none",
                                "flow": profile.flow.as_deref().unwrap_or("")
                            }]
                        }]
                    },
                    "streamSettings": {
                        "network": profile.network.as_deref().unwrap_or("tcp"),
                        "security": profile.security.as_deref().unwrap_or("tls"),
                        "tlsSettings": {
                            "serverName": profile.sni.as_deref().unwrap_or(&profile.server)
                        }
                    }
                },
                {
                    "tag": "direct",
                    "protocol": "freedom"
                },
                {
                    "tag": "block",
                    "protocol": "blackhole"
                }
            ]
        });

        serde_json::to_string_pretty(&config)
            .map_err(|e| RayCrateError::XrayError(e.to_string()))
    }

    /// Starts Xray process with the given configuration
    pub async fn start(&self, config_json: &str) -> Result<()> {
        let mut proc_guard = self.process.lock().await;
        if proc_guard.is_some() {
            return Err(RayCrateError::XrayError("Xray is already running".to_string()));
        }

        // Write config to temp file
        let config_path = std::env::temp_dir().join("raycrate_xray_config.json");
        std::fs::write(&config_path, config_json)?;

        info!("Starting Xray-core from {}", self.binary_path);
        let child = Command::new(&self.binary_path)
            .arg("-config")
            .arg(config_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| RayCrateError::XrayError(format!("Failed to spawn xray binary: {}", e)))?;

        *proc_guard = Some(child);
        Ok(())
    }

    /// Stops Xray process
    pub async fn stop(&self) -> Result<()> {
        let mut proc_guard = self.process.lock().await;
        if let Some(mut child) = proc_guard.take() {
            info!("Stopping Xray-core");
            child.kill()?;
            child.wait()?;
        }
        Ok(())
    }
}
