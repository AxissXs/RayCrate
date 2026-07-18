use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::{error, info};

use raycrate_core::config::ProxyProfile;
use raycrate_core::tun::TunManager;
use raycrate_core::xray::XrayManager;
use raycrate_core::{RayCrateError, Result};

#[derive(Parser, Debug)]
#[command(
    name = "raycrate",
    version = "0.1.0",
    author = "RayCrate Contributors",
    about = "Cross-platform proxy client with Xray-core, TUN mode, and multi-protocol support"
)]
struct Cli {
    /// Enable debug logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Import a proxy from a link or subscription
    Import {
        #[command(subcommand)]
        source: ImportSource,
    },

    /// List imported proxy profiles
    List,

    /// Connect to a proxy profile
    Connect {
        /// Profile ID, index, or name to connect to
        profile: String,

        /// Local SOCKS5 proxy port
        #[arg(short, long, default_value = "1080")]
        port: u16,

        /// Xray-core binary path
        #[arg(short, long, default_value = "xray")]
        xray: String,

        /// Enable TUN mode (requires root)
        #[arg(short, long)]
        tun: bool,

        /// TUN interface name
        #[arg(long, default_value = "utun99")]
        tun_name: String,
    },

    /// Disconnect from current proxy
    Disconnect,

    /// Check latency for a proxy profile
    Ping {
        /// Profile ID, index, or name to ping
        profile: String,
    },

    /// Run a speed test for a proxy profile
    Speedtest {
        /// Profile ID, index, or name to test
        profile: String,
    },

    /// Start Xray-core with a config file
    RunXray {
        /// Path to Xray JSON config file
        config: PathBuf,

        /// Xray-core binary path
        #[arg(short, long, default_value = "xray")]
        binary: String,
    },

    /// Generate Xray config from a proxy link
    GenConfig {
        /// Proxy link (e.g. vless://...)
        link: String,

        /// Local SOCKS5 port
        #[arg(short, long, default_value = "1080")]
        port: u16,

        /// Output file path (prints to stdout if not provided)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug)]
enum ImportSource {
    /// Import from a single proxy link
    Link { url: String },

    /// Import from a subscription URL
    Sub {
        url: String,

        /// Output file for profiles
        #[arg(short, long, default_value = "profiles.json")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize tracing
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::new(format!("raycrate={},raycrate_core={}", log_level, log_level))
        )
        .init();

    info!("RayCrate v0.1.0 - Cross-platform Proxy Client");

    if let Err(e) = run_command(cli).await {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Import { source } => match source {
            ImportSource::Link { url } => {
                let profile = ProxyProfile::parse_link(&url)?;
                info!("Successfully imported proxy: {} ({:?})", profile.name, profile.protocol);
                println!("Profile: {}", serde_json::to_string_pretty(&profile)?);

                // Save to default profiles file
                save_profile(&profile, "profiles.json").await?;
            }
            ImportSource::Sub { url, output } => {
                info!("Fetching subscription from: {}", url);
                let profiles = ProxyProfile::fetch_subscription(&url).await?;
                info!("Fetched {} profiles from subscription", profiles.len());

                let json = serde_json::to_string_pretty(&profiles)?;
                tokio::fs::write(&output, json).await?;
                info!("Saved profiles to: {}", output.display());

                for (i, p) in profiles.iter().enumerate() {
                    println!("[{}] {} - {}:{}", i, p.name, p.server, p.port);
                }
            }
        },

        Commands::List => {
            let profiles = load_profiles("profiles.json").await?;
            if profiles.is_empty() {
                println!("No profiles found. Use 'raycrate import' to add some.");
            } else {
                println!("{:<5} {:<30} {:<20} {:<10}", "#", "Name", "Server", "Protocol");
                println!("{}", "-".repeat(70));
                for (i, p) in profiles.iter().enumerate() {
                    println!(
                        "{:<5} {:<30} {:<20} {:?}",
                        i,
                        p.name.chars().take(28).collect::<String>(),
                        format!("{}:{}", p.server, p.port),
                        p.protocol
                    );
                }
            }
        }

        Commands::Connect {
            profile,
            port,
            xray,
            tun,
            tun_name,
        } => {
            let profiles = load_profiles("profiles.json").await?;
            let selected = find_profile(&profiles, &profile)
                .ok_or_else(|| RayCrateError::ConfigError(format!("Profile '{}' not found", profile)))?;

            info!("Connecting to: {} ({}:{})", selected.name, selected.server, selected.port);

            // Generate Xray config
            let config = XrayManager::generate_config(selected, port)?;
            let config_path = std::env::temp_dir().join("raycrate_config.json");
            tokio::fs::write(&config_path, &config).await?;
            info!("Generated Xray config at: {}", config_path.display());

            // Start Xray
            let xray_mgr = XrayManager::new(xray);
            xray_mgr.start(&config).await?;
            info!("Xray-core started. SOCKS5 proxy available at: 127.0.0.1:{}", port);

            if tun {
                info!("Initializing TUN mode...");
                let mut tun_mgr = TunManager::new(&tun_name);
                tun_mgr.start().await?;
                info!("TUN interface '{}' is up", tun_name);
            }

            println!("Connected to '{}' successfully!", selected.name);
            println!("SOCKS5 Proxy: 127.0.0.1:{}", port);
            if tun {
                println!("TUN Mode: {} (active)", tun_name);
            }

            // Keep running until interrupted
            info!("Press Ctrl+C to disconnect...");
            tokio::signal::ctrl_c().await?;
            info!("Shutting down...");

            if tun {
                let mut tun_mgr = TunManager::new(&tun_name);
                tun_mgr.stop().await?;
            }

            xray_mgr.stop().await?;
            info!("Disconnected.");
        }

        Commands::Disconnect => {
            info!("Disconnecting...");
            // In a real app, you'd track the running process handles
            // Here we just inform the user
            println!("Use Ctrl+C in the terminal where raycrate connect is running to disconnect.");
        }

        Commands::Ping { profile } => {
            let profiles = load_profiles("profiles.json").await?;
            let selected = find_profile(&profiles, &profile)
                .ok_or_else(|| RayCrateError::ConfigError(format!("Profile '{}' not found", profile)))?;

            info!("Pinging: {} ({}:{})...", selected.name, selected.server, selected.port);
            
            // Simple TCP connect latency test
            let start = std::time::Instant::now();
            match tokio::time::timeout(
                std::time::Duration::from_secs(5),
                tokio::net::TcpStream::connect(format!("{}:{}", selected.server, selected.port))
            ).await {
                Ok(Ok(_)) => {
                    let latency = start.elapsed().as_millis();
                    println!("✓ Latency: {} ms to {}:{}", latency, selected.server, selected.port);
                }
                Ok(Err(e)) => {
                    println!("✗ Connection failed: {}", e);
                }
                Err(_) => {
                    println!("✗ Timeout (>{:?})", std::time::Duration::from_secs(5));
                }
            }
        }

        Commands::Speedtest { profile } => {
            let profiles = load_profiles("profiles.json").await?;
            let selected = find_profile(&profiles, &profile)
                .ok_or_else(|| RayCrateError::ConfigError(format!("Profile '{}' not found", profile)))?;

            info!("Running speed test for: {} ...", selected.name);
            println!("Speed test feature will be fully implemented in a future release.");
            println!("Profile: {} ({}:{})", selected.name, selected.server, selected.port);
        }

        Commands::RunXray { config, binary } => {
            info!("Starting Xray-core with config: {}", config.display());
            let config_str = tokio::fs::read_to_string(&config).await?;
            let xray_mgr = XrayManager::new(binary);
            xray_mgr.start(&config_str).await?;

            info!("Xray-core is running. Press Ctrl+C to stop.");
            tokio::signal::ctrl_c().await?;

            xray_mgr.stop().await?;
            info!("Xray-core stopped.");
        }

        Commands::GenConfig { link, port, output } => {
            let profile = ProxyProfile::parse_link(&link)?;
            let config = XrayManager::generate_config(&profile, port)?;

            if let Some(path) = output {
                tokio::fs::write(&path, &config).await?;
                info!("Xray config written to: {}", path.display());
            } else {
                println!("{}", config);
            }
        }
    }

    Ok(())
}

// Helper functions

async fn load_profiles(path: &str) -> Result<Vec<ProxyProfile>> {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => {
            serde_json::from_str(&content)
                .map_err(|e| RayCrateError::ConfigError(format!("Failed to parse profiles: {}", e)))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(RayCrateError::IoError(e)),
    }
}

async fn save_profile(profile: &ProxyProfile, path: &str) -> Result<()> {
    let mut profiles = load_profiles(path).await?;
    profiles.push(profile.clone());
    let json = serde_json::to_string_pretty(&profiles)?;
    tokio::fs::write(path, json).await?;
    Ok(())
}

fn find_profile<'a>(profiles: &'a [ProxyProfile], query: &str) -> Option<&'a ProxyProfile> {
    // Try by index first
    if let Ok(index) = query.parse::<usize>() {
        return profiles.get(index);
    }
    // Try by name or id
    profiles.iter().find(|p| p.name == query || p.id == query)
}
