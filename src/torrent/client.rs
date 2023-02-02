use std::env;

use serde::{Deserialize, Serialize};
use transmission_rpc::{
    types::{Id, Nothing, Result, RpcResponse, TorrentAction, TorrentAddArgs, TorrentAdded, TorrentGetField},
    TransClient,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Torrent {
    id: i64,
    name: String,
    status: String,
    size: i64,
    progress: i32,
}

pub struct Client {
    client: TransClient,
}

impl Client {
    pub fn new(client: TransClient) -> Self {
        Self { client }
    }

    pub async fn list_torrents(&self) -> Result<Vec<Torrent>> {
        let res = self.client.torrent_get(Some(vec![
            TorrentGetField::Id, 
            TorrentGetField::Name, 
            TorrentGetField::Status, 
            TorrentGetField::Isfinished, 
            TorrentGetField::Isstalled, 
            TorrentGetField::Totalsize,
            TorrentGetField::Percentdone, 
            TorrentGetField::Error]), None).await?;
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

pub fn client() -> Client {
    let url = env::var("TURL").unwrap();
    let client = TransClient::new(&url);

    Client::new(client)
}
