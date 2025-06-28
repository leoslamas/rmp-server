use remote_media_pi::torrent::client::Torrent;
use transmission_rpc::{
    types::{Nothing, Result, RpcResponse, TorrentAdded},
};

pub struct MockTransmissionClient {
    pub torrents: Vec<Torrent>,
    pub add_should_fail: bool,
    pub action_should_fail: bool,
}

impl MockTransmissionClient {
    pub fn new() -> Self {
        Self {
            torrents: Vec::new(),
            add_should_fail: false,
            action_should_fail: false,
        }
    }

    pub fn with_torrents(mut self, torrents: Vec<Torrent>) -> Self {
        self.torrents = torrents;
        self
    }

    pub fn with_add_failure(mut self) -> Self {
        self.add_should_fail = true;
        self
    }

    pub fn with_action_failure(mut self) -> Self {
        self.action_should_fail = true;
        self
    }
}

impl Default for MockTransmissionClient {
    fn default() -> Self {
        Self::new()
    }
}

// Note: This would need actual implementation when we have a trait for the transmission client
// For now, it serves as a placeholder for future mock implementation