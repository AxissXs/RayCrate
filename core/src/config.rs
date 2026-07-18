use crate::{RayCrateError, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ProtocolType {
    Vless,
    Vmess,
    Trojan,
    Shadowsocks,
    Hysteria2,
    Tuic,
    OpenVPN,
    OpenConnect,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProxyProfile {
    pub id: String,
    pub name: String,
    pub protocol: ProtocolType,
    pub server: String,
    pub port: u16,
    pub uuid: Option<String>,
    pub password: Option<String>,
    pub sni: Option<String>,
    pub pbk: Option<String>, // Public key for reality
    pub sid: Option<String>, // Short ID for reality
    pub flow: Option<String>,
    pub security: Option<String>,
    pub network: Option<String>, // tcp, ws, gRPC
    pub path: Option<String>,    // ws path
}

impl ProxyProfile {
    /// Parses a proxy link (e.g. vless://, trojan://, ss://)
    pub fn parse_link(link: &str) -> Result<Self> {
        let url = url::Url::parse(link)?;
        let protocol = match url.scheme() {
            "vless" => ProtocolType::Vless,
            "vmess" => ProtocolType::Vmess,
            "trojan" => ProtocolType::Trojan,
            "ss" => ProtocolType::Shadowsocks,
            "hysteria2" | "hy2" => ProtocolType::Hysteria2,
            "tuic" => ProtocolType::Tuic,
            sch => return Err(RayCrateError::ConfigError(format!("Unsupported scheme: {}", sch))),
        };

        let server = url.host_str().ok_or_else(|| {
            RayCrateError::ConfigError("Missing server address in proxy link".to_string())
        })?.to_string();

        let port = url.port().unwrap_or(443);
        let name = url.fragment().unwrap_or("Imported Proxy").to_string();
        let uuid = if !url.username().is_empty() {
            Some(url.username().to_string())
        } else {
            url.password().map(|s| s.to_string())
        };

        Ok(ProxyProfile {
            id: uuid.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            name,
            protocol,
            server,
            port,
            uuid,
            password: None,
            sni: None,
            pbk: None,
            sid: None,
            flow: None,
            security: Some("tls".to_string()),
            network: Some("tcp".to_string()),
            path: None,
        })
    }

    /// Fetches and parses a subscription link (Base64 encoded list of links)
    pub async fn fetch_subscription(url: &str) -> Result<Vec<ProxyProfile>> {
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?.text().await?;
        
        // Decode base64 if needed
        let decoded = if let Ok(bytes) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, resp.trim()) {
            String::from_utf8(bytes).unwrap_or(resp)
        } else {
            resp
        };

        let mut profiles = Vec::new();
        for line in decoded.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(profile) = Self::parse_link(line) {
                profiles.push(profile);
            }
        }

        Ok(profiles)
    }
}
