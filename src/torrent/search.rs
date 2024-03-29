use std::{
    sync::{Arc, Barrier},
    thread,
};

use serde::{Deserialize, Serialize};

use super::source::{L337xTo, SourceAdapter, TorrentDownloader};

pub struct Searcher {
    adapters: Vec<Arc<dyn SourceAdapter + Send + Sync>>,
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
        }
    }

    pub fn using(mut self, adapter: Arc<dyn SourceAdapter + Send + Sync>) -> Self {
        self.adapters.push(adapter);
        self
    }

    pub fn search(&self, terms: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let size = self.adapters.len();

        let mut handles = Vec::with_capacity(size);
        let barrier = Arc::new(Barrier::new(size));

        for adapter in &self.adapters {
            let c = Arc::clone(&barrier);
            let terms = String::from(terms);
            let a = Arc::clone(&adapter);

            handles.push(thread::spawn(move || {
                c.wait();
                //this was really painful
                (|terms: &str, adapter: Arc<dyn SourceAdapter>| {
                    let url = adapter.build_url(&terms);
                    info!("Scraping: {url}");
                    let document = adapter.get_document(url);
                    match document {
                        Some(s) => {
                            let html = adapter.scrap_from_document(s);
                            adapter.select_results(html)
                        }
                        None => {
                            error!("Error scraping page!");
                            vec![]
                        }
                    }
                })(&terms, a)
            }));
        }

        for handle in handles {
            match handle.join() {
                Ok(ref mut r) => results.append(r),
                Err(e) => error!("{e:#?}"),
            }
        }

        results.sort();
        results.dedup();
        results.reverse();
        results
    }
}

#[derive(Eq, Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub name: String,
    pub seeders: String,
    pub leechers: String,
    pub size: String,
    pub url: String,
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.seeders.cmp(&other.seeders)
    }
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.seeders.partial_cmp(&other.seeders)
    }
}

impl PartialEq for SearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.seeders == other.seeders
    }
}

pub fn searcher() -> Searcher {
    Searcher::new().using(L337xTo::new(TorrentDownloader::new()))
}
