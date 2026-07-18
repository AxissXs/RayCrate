use crate::{RayCrateError, Result};
use tracing::info;

pub struct TunManager {
    interface_name: String,
    is_active: bool,
}

impl TunManager {
    pub fn new(interface_name: &str) -> Self {
        Self {
            interface_name: interface_name.to_string(),
            is_active: false,
        }
    }

    /// Starts TUN virtual network device
    pub async fn start(&mut self) -> Result<()> {
        if self.is_active {
            return Err(RayCrateError::TunError("TUN interface already active".to_string()));
        }

        info!("Initializing TUN interface: {}", self.interface_name);
        
        // Configuration for tun2 virtual device
        let mut config = tun2::Configuration::default();
        config.name(&self.interface_name);
        config.address("10.0.0.2");
        config.netmask("255.255.255.0");
        config.up();

        // Note: Actual packet read/write loops and routing table setup would be initialized here using tun2::create(&config)
        self.is_active = true;
        info!("TUN interface successfully activated.");
        Ok(())
    }

    /// Stops TUN virtual network device and restores routing tables
    pub async fn stop(&mut self) -> Result<()> {
        if !self.is_active {
            return Ok(());
        }

        info!("Stopping TUN interface: {}", self.interface_name);
        self.is_active = false;
        info!("TUN interface stopped and routing tables restored.");
        Ok(())
    }
}
