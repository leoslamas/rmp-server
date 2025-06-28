use std::time::Duration;

use log::{error, info};
use scraper::{Html, Selector};

use super::search::SearchResult;

pub trait Downloader: Send + Sync {
    fn download(&self, url: String) -> Option<String>;
}

pub struct TorrentDownloader;

impl TorrentDownloader {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl Downloader for TorrentDownloader {
    fn download(&self, url: String) -> Option<String> {
        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::builder()
                .connect_timeout(Duration::from_secs(30))
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap();
            let result = client.get(&url).send();
            match result {
                Ok(res) => {
                    info!("Response...");
                    let text = res.text();
                    match text {
                        Ok(t) => {
                            info!("Body...");
                            Some(t)
                        }
                        Err(e) => {
                            error!("Err: {e}");
                            None
                        }
                    }
                }
                Err(e) => {
                    error!("Nothing found! {e}");
                    None
                }
            }
        })
        .join()
        .unwrap()
    }
}

pub trait SourceAdapter: Send + Sync {
    fn build_url(&self, terms: &str) -> String;

    fn select_results(&self, fragment: Html) -> Vec<SearchResult>;

    fn get_document(&self, url: String) -> Option<String>;

    fn scrap_from_document(&self, document: String) -> Html {
        Html::parse_fragment(document.as_str())
    }

    fn find_magnet(&self, document: &str, selector: &Selector) -> Option<String> {
        let html = self.scrap_from_document(document.to_owned());
        if let Some(selection) = html.select(selector).next() {
            return selection.value().attr("href").map(|x| x.to_string());
        }
        None
    }
}

//https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
