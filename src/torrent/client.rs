use serde::{Deserialize, Serialize};
use transmission_rpc::{
    types::{
        Id, Nothing, Result, RpcResponse, TorrentAction, TorrentAddArgs, TorrentAdded,
        TorrentGetField,
    },
    TransClient,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Torrent {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub size: i64,
    pub progress: i32,
}

pub struct Client {
    client: TransClient,
}

impl Client {
    pub fn new(client: TransClient) -> Self {
        Self { client }
    }

    pub async fn list_torrents(&self) -> Result<Vec<Torrent>> {
        let res = self
            .client
            .torrent_get(
                Some(vec![
                    TorrentGetField::Id,
                    TorrentGetField::Name,
                    TorrentGetField::Status,
                    TorrentGetField::Isfinished,
                    TorrentGetField::Isstalled,
                    TorrentGetField::Totalsize,
                    TorrentGetField::Percentdone,
                    TorrentGetField::Error,
                ]),
                None,
            )
            .await?;
        let torrents = res.arguments.torrents;
        Ok(torrents
            .iter()
            .map(|t| {
                let status = match t.status {
                    Some(0) | Some(1) | Some(2) => "paused",
                    Some(3) | Some(4) => "downloading",
                    Some(5) | Some(6) => "done",
                    _ => {
                        if t.is_finished.is_some() && t.is_finished.unwrap() {
                            "done"
                        } else if t.error.is_some() {
                            "error"
                        } else if t.is_stalled.is_some() && t.is_stalled.unwrap() {
                            "paused"
                        } else {
                            "downloading"
                        }
                    }
                };

                Torrent {
                    id: t.id.unwrap(),
                    name: t.name.clone().unwrap(),
                    size: t.total_size.unwrap(),
                    status: status.into(),
                    progress: (t.percent_done.unwrap() * 100.0) as i32,
                }
            })
            .collect())
    }

    pub async fn add_torrent(&self, url: &str) -> Result<RpcResponse<TorrentAdded>> {
        let add = TorrentAddArgs {
            filename: Some(url.into()),
            ..TorrentAddArgs::default()
        };

        self.client.torrent_add(add).await
    }

    pub async fn remove_torrent(&self, id: i64) -> Result<RpcResponse<Nothing>> {
        self.client.torrent_remove(vec![Id::Id(id)], true).await
    }

    pub async fn pause_torrent(&self, id: i64) -> Result<RpcResponse<Nothing>> {
        self.client
            .torrent_action(TorrentAction::Stop, vec![Id::Id(id)])
            .await
    }

    pub async fn resume_torrent(&self, id: i64) -> Result<RpcResponse<Nothing>> {
        self.client
            .torrent_action(TorrentAction::Start, vec![Id::Id(id)])
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torrent_creation() {
        let torrent = Torrent {
            id: 1,
            name: "Test Movie".to_string(),
            status: "downloading".to_string(),
            size: 1000000000, // 1GB
            progress: 50,
        };

        assert_eq!(torrent.id, 1);
        assert_eq!(torrent.name, "Test Movie");
        assert_eq!(torrent.status, "downloading");
        assert_eq!(torrent.size, 1000000000);
        assert_eq!(torrent.progress, 50);
    }

    #[test]
    fn test_torrent_clone() {
        let torrent = Torrent {
            id: 1,
            name: "Test Movie".to_string(),
            status: "downloading".to_string(),
            size: 1000000000,
            progress: 50,
        };

        let cloned = torrent.clone();
        assert_eq!(torrent.id, cloned.id);
        assert_eq!(torrent.name, cloned.name);
        assert_eq!(torrent.status, cloned.status);
        assert_eq!(torrent.size, cloned.size);
        assert_eq!(torrent.progress, cloned.progress);
    }

    #[test]
    fn test_torrent_serialization() {
        let torrent = Torrent {
            id: 1,
            name: "Test Movie".to_string(),
            status: "downloading".to_string(),
            size: 1000000000,
            progress: 50,
        };

        // Test that the torrent can be serialized to JSON
        let json = serde_json::to_string(&torrent);
        assert!(json.is_ok());
        
        // Test that it can be deserialized back
        let json_str = json.unwrap();
        let deserialized = serde_json::from_str::<Torrent>(&json_str);
        assert!(deserialized.is_ok());
        
        let deserialized_torrent = deserialized.unwrap();
        assert_eq!(torrent.id, deserialized_torrent.id);
        assert_eq!(torrent.name, deserialized_torrent.name);
        assert_eq!(torrent.status, deserialized_torrent.status);
        assert_eq!(torrent.size, deserialized_torrent.size);
        assert_eq!(torrent.progress, deserialized_torrent.progress);
    }

    // Note: The Client struct methods require an actual TransClient which needs 
    // a running Transmission daemon. For true unit testing, we would need to 
    // create a trait for the transmission client and then create mock implementations.
    // For integration tests, these would test against a real or test Transmission instance.

    #[test]
    fn test_client_creation() {
        // This test just verifies that we can create a client struct
        // In a real scenario, this would require a running transmission daemon
        // or a mocked transmission client
        
        use transmission_rpc::TransClient;
        let trans_client = TransClient::new("http://localhost:9091/transmission/rpc");
        let _client = Client::new(trans_client);
        
        // Just verify the client was created - we can't test the methods without
        // a running transmission daemon or proper mocking
        assert!(true); // Placeholder assertion
    }
}
