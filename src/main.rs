
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

mod config;
mod error;
mod handlers;
mod torrent;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Starting server!");

    let config = config::ServerConfig::from_env();
    
    tokio::spawn(async move {
        match UdpSocket::bind(&config.discovery_addr).await {
            Ok(mut socket) => {
                info!("UDP discovery service started on {}", config.discovery_addr);
                loop {
                    if let Ok((_, addr)) = socket.recv_from(&mut [1; 1]).await {
                        if let Err(e) = socket.send_to(&[1; 1], &addr).await {
                            error!("Failed to send discovery response to {}: {}", addr, e);
                        } else {
                            info!("Discovery packet sent to {}", addr);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to bind UDP socket to {}: {}", config.discovery_addr, e);
            }
        }
    });

    let server_config = config::ServerConfig::from_env();
    let server_addr = server_config.server_addr.clone();
    
    HttpServer::new(move || {
        App::new()
            .data(client(&server_config))
            .data(searcher())
            .service(handlers::search_torrents)
            .service(handlers::list_torrents)
            .service(handlers::add_torrent)
            .service(handlers::resume_torrent)
            .service(handlers::pause_torrent)
            .service(handlers::del_torrent)
            .default_service(web::route().to(HttpResponse::NotFound))
    })
    .bind(server_addr)?
    .run()
    .await
}

pub fn searcher() -> Searcher {
    Searcher::new().using(L337xTo::new(TorrentDownloader::new()))
}

pub fn client(config: &config::ServerConfig) -> Client {
    let client = TransClient::new(&config.transmission_url);
    Client::new(client)
}
