use serde::{Deserialize, Serialize};

pub const WEST_API_BASE_URL: &str = "https://west.albion-online-data.com";
pub const EAST_API_BASE_URL: &str = "https://east.albion-online-data.com";
pub const EUROPE_API_BASE_URL: &str = "https://europe.albion-online-data.com";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlbionServer {
    West,
    East,
    Europe,
}

impl AlbionServer {
    pub const ALL: [Self; 3] = [Self::West, Self::East, Self::Europe];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::West => "west",
            Self::East => "east",
            Self::Europe => "europe",
        }
    }
}

pub fn server_base_url(server: &str) -> &'static str {
    match server {
        "east" => EAST_API_BASE_URL,
        "europe" => EUROPE_API_BASE_URL,
        _ => WEST_API_BASE_URL,
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: AlbionServer,
    pub locations: Vec<String>,
    pub qualities: Vec<u8>,
    pub refresh_interval_seconds: u64,
    pub top_limit: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: AlbionServer::West,
            locations: vec![
                "Bridgewatch".to_owned(),
                "Caerleon".to_owned(),
                "Fort Sterling".to_owned(),
                "Lymhurst".to_owned(),
                "Martlock".to_owned(),
                "Thetford".to_owned(),
                "Black Market".to_owned(),
            ],
            qualities: vec![1],
            refresh_interval_seconds: 30,
            top_limit: 10,
        }
    }
}
