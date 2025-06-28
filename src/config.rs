use std::env;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub discovery_addr: String,
    pub server_addr: String,
    pub transmission_url: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            discovery_addr: "0.0.0.0:9191".to_string(),
            server_addr: "0.0.0.0:9090".to_string(),
            transmission_url: "http://127.0.0.1:9091/transmission/rpc".to_string(),
        }
    }
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let default = Self::default();
        
        Self {
            discovery_addr: env::var("D_ADDR").unwrap_or(default.discovery_addr),
            server_addr: env::var("S_ADDR").unwrap_or(default.server_addr),
            transmission_url: env::var("TURL").unwrap_or(default.transmission_url),
        }
    }
}

#[derive(Debug, Clone)]
pub struct L337xToConfig {
    pub base_url: String,
    pub selectors: L337xToSelectors,
}

#[derive(Debug, Clone)]
pub struct L337xToSelectors {
    pub row: String,
    pub name: String,
    pub seeders: String,
    pub leechers: String,
    pub size: String,
    pub url: String,
    pub magnet: String,
}

impl Default for L337xToConfig {
    fn default() -> Self {
        Self {
            base_url: "https://1337xx.to".to_string(),
            selectors: L337xToSelectors::default(),
        }
    }
}

impl Default for L337xToSelectors {
    fn default() -> Self {
        Self {
            row: "tr".to_string(),
            name: ".name".to_string(),
            seeders: ".seeds".to_string(),
            leechers: ".leeches".to_string(),
            size: ".size".to_string(),
            url: ".name a:nth-child(2)".to_string(),
            magnet: "a[href^=magnet]".to_string(),
        }
    }
}