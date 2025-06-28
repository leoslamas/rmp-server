use remote_media_pi::torrent::downloader::Downloader;

pub struct MockDownloader {
    pub responses: std::collections::HashMap<String, Option<String>>,
}

impl MockDownloader {
    pub fn new() -> Self {
        Self {
            responses: std::collections::HashMap::new(),
        }
    }

    pub fn with_response(mut self, url: &str, response: Option<String>) -> Self {
        self.responses.insert(url.to_string(), response);
        self
    }
}

impl Default for MockDownloader {
    fn default() -> Self {
        Self::new()
    }
}

impl Downloader for MockDownloader {
    fn download(&self, url: String) -> Option<String> {
        self.responses.get(&url).cloned().flatten()
    }
}