mod l337xto;
use std::time::Duration;

pub(crate) use l337xto::L337xTo;

use scraper::{Html, Selector};

use super::search::SearchResult;

//https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/

pub trait Downloader {
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
            .build().unwrap();
            let result = client.get(&url).send();
            match result {
                Ok(res) => {
                    info!("Response...");
                    let text = res.text();
                    match text {
                        Ok(t) => {
                            info!("Body...");
                            return Some(t);
                        },
                        Err(e) => {
                            error!("Err: {e}");
                            return None;
                        }
                    }
                },
                Err(e) => {
                    error!("Nothing found! {e}");
                    return None;
                }
            }
        })
        .join()
        .unwrap()
    }
}

pub trait SourceAdapter {
    fn build_url(&self, terms: &str) -> String;

    fn select_results(&self, fragment: Html) -> Vec<SearchResult>;

    fn get_document(&self, url: String) -> Option<String>;

    fn scrap_from_document(&self, document: String) -> Html {
        Html::parse_fragment(document.as_str())
    }

    fn find_magnet(&self, document: &String, selector: &Selector) -> Option<String> {
        let html = self.scrap_from_document(document.clone());
        if let Some(selection) = html.select(selector).next() {
            return selection.value().attr("href").map(|x| x.to_string());
        }
        None
    }
}
