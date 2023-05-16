use crate::torrent::{client::Client, search::Searcher};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use transmission_rpc::types::{Nothing, Result, RpcResponse};

#[derive(Serialize, Deserialize, Debug)]
struct QueryParams {
    terms: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TorrentObj {
    pub url: String,
}

#[get("/torrent/search")]
async fn search_torrents(Query(params): Query<QueryParams>, searcher: Data<Searcher>) -> impl Responder {
    info!("Searching for {}!", params.terms);

    if params.terms.len() > 1 {
        let search_result = searcher.search(&params.terms.trim());
        HttpResponse::Ok().json(search_result)
    } else {
        HttpResponse::InternalServerError().body("Length > 2")
    }
}

#[get("/torrent/list")]
async fn list_torrents(client: Data<Client>) -> impl Responder {
    info!("Listing torrents!");
    let torrent_list = client.list_torrents().await;

    match torrent_list {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/torrent/add")]
async fn add_torrent(torrent: Json<TorrentObj>, client: Data<Client>) -> impl Responder {
    info!("Adding torrent! {}", torrent.url);
    let result = client.add_torrent(torrent.url.as_str()).await;

    match result {
        Ok(r) => {
            info!("Torrent added! {:?}", r);
            HttpResponse::Ok().body(r.result)
        }
        Err(e) => {
            error!("Torrent add error! {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[post("/torrent/resume/{id}")]
async fn resume_torrent(Path(id): Path<i64>, client: Data<Client>) -> impl Responder {
    info!("Resuming torrent! {}", id);
    let result = client.resume_torrent(id).await;

    process(result)
}

#[post("/torrent/pause/{id}")]
async fn pause_torrent(Path(id): Path<i64>, client: Data<Client>) -> impl Responder {
    info!("Pausing torrent! {}", id);
    let result = client.pause_torrent(id).await;

    process(result)
}

#[delete("/torrent/remove/{id}")]
async fn del_torrent(Path(id): Path<i64>, client: Data<Client>) -> impl Responder {
    info!("Deleting torrent {}!", id);
    let result = client.remove_torrent(id).await;

    process(result)
}

fn process(result: Result<RpcResponse<Nothing>>) -> impl Responder {
    match result {
        Ok(_) => HttpResponse::Ok().json("ok"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
