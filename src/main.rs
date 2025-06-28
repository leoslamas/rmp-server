use std::env;

use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use tokio::net::UdpSocket;

use torrent::{
    client::Client, 
    downloader::TorrentDownloader, 
    search::Searcher, 
    source::l337xto::L337xTo
};
use transmission_rpc::TransClient;

#[macro_use]
extern crate log;

mod handlers;
mod torrent;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Starting server!");

    tokio::spawn(async {
        let mut socket = UdpSocket::bind(env::var("D_ADDR").unwrap()).await.unwrap();
        loop {
            let (_, addr) = socket.recv_from(&mut [1; 1]).await.unwrap();
            socket.send_to(&[1; 1], &addr).await.unwrap();
            info!("Discovering! Discovery packet sent to {}", addr);
        }
    });

    HttpServer::new(move || {
        App::new()
            .data(client())
            .data(searcher())
            .service(handlers::search_torrents)
            .service(handlers::list_torrents)
            .service(handlers::add_torrent)
            .service(handlers::resume_torrent)
            .service(handlers::pause_torrent)
            .service(handlers::del_torrent)
            .default_service(web::route().to(HttpResponse::NotFound))
    })
    .bind(env::var("S_ADDR").unwrap())?
    .run()
    .await
}

pub fn searcher() -> Searcher {
    Searcher::new().using(L337xTo::new(TorrentDownloader::new()))
}

pub fn client() -> Client {
    let url = env::var("TURL").unwrap();
    let client = TransClient::new(&url);

    Client::new(client)
}
