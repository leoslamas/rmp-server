use std::sync::Arc;
use scraper::Html;
use remote_media_pi::torrent::{
    downloader::SourceAdapter,
    search::SearchResult,
};

pub struct MockTorrentSource {
    pub search_results: Vec<SearchResult>,
    pub base_url: String,
}

impl MockTorrentSource {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            search_results: Vec::new(),
            base_url: "https://mock-source.com".to_string(),
        })
    }

    pub fn with_results(mut self, results: Vec<SearchResult>) -> Arc<Self> {
        self.search_results = results;
        Arc::new(self)
    }
}

impl Default for MockTorrentSource {
    fn default() -> Self {
        Self {
            search_results: Vec::new(),
            base_url: "https://mock-source.com".to_string(),
        }
    }
}

impl SourceAdapter for MockTorrentSource {
    fn build_url(&self, terms: &str) -> String {
        format!("{}/search/{}", self.base_url, terms)
    }

    fn get_document(&self, _url: String) -> Option<String> {
        Some("<html><body>Mock HTML</body></html>".to_string())
    }

    fn select_results(&self, _fragment: Html) -> Vec<SearchResult> {
        self.search_results.clone()
    }
}