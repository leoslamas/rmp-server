use std::{
    sync::{Arc, Barrier},
    thread,
};

use log::{error, info};
use serde::{Deserialize, Serialize};

use super::downloader::SourceAdapter;

pub struct Searcher {
    adapters: Vec<Arc<dyn SourceAdapter>>,
}

impl Default for Searcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
        }
    }

    pub fn using(mut self, adapter: Arc<dyn SourceAdapter>) -> Self {
        self.adapters.push(adapter);
        self
    }

    pub fn search(&self, terms: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let size = self.adapters.len();

        let mut handles = Vec::with_capacity(size);
        let barrier = Arc::new(Barrier::new(size));

        for adapter in &self.adapters {
            let trms = String::from(terms);
            let brr = Arc::clone(&barrier);
            let adptr: Arc<dyn SourceAdapter> = Arc::clone(adapter);

            handles.push(thread::spawn(move || {
                brr.wait();
                let url = adptr.build_url(&trms);
                info!("Scraping: {url}");
                let document = adptr.get_document(url);
                match document {
                    Some(s) => {
                        let html = adptr.scrap_from_document(s);
                        adptr.select_results(html)
                    }
                    None => {
                        error!("Error scraping page!");
                        vec![]
                    }
                }
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

#[derive(Eq, Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub seeders: u32,
    pub leechers: u32,
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
        self.name == other.name && self.url == other.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use scraper::Html;

    // Mock source adapter for testing
    struct MockSourceAdapter {
        results: Vec<SearchResult>,
    }

    impl MockSourceAdapter {
        fn new(results: Vec<SearchResult>) -> Arc<Self> {
            Arc::new(Self { results })
        }
    }

    impl SourceAdapter for MockSourceAdapter {
        fn build_url(&self, _terms: &str) -> String {
            "http://test.com".to_string()
        }

        fn get_document(&self, _url: String) -> Option<String> {
            Some("<html></html>".to_string())
        }

        fn select_results(&self, _fragment: Html) -> Vec<SearchResult> {
            self.results.clone()
        }
    }

    fn sample_results() -> Vec<SearchResult> {
        vec![
            SearchResult {
                name: "High Seeds Movie".to_string(),
                seeders: 1000,
                leechers: 100,
                size: "1.5 GB".to_string(),
                url: "magnet:high".to_string(),
            },
            SearchResult {
                name: "Medium Seeds Movie".to_string(), 
                seeders: 500,
                leechers: 50,
                size: "800 MB".to_string(),
                url: "magnet:medium".to_string(),
            },
            SearchResult {
                name: "Low Seeds Movie".to_string(),
                seeders: 100,
                leechers: 10,
                size: "2.0 GB".to_string(),
                url: "magnet:low".to_string(),
            },
        ]
    }

    #[test]
    fn test_searcher_new() {
        let searcher = Searcher::new();
        assert_eq!(searcher.adapters.len(), 0);
    }

    #[test]
    fn test_searcher_using() {
        let mock_adapter = MockSourceAdapter::new(vec![]);
        let searcher = Searcher::new().using(mock_adapter);
        assert_eq!(searcher.adapters.len(), 1);
    }

    #[test]
    fn test_search_results_sorted_by_seeders() {
        let results = sample_results();
        let mock_adapter = MockSourceAdapter::new(results);
        let searcher = Searcher::new().using(mock_adapter);
        
        let search_results = searcher.search("test");
        
        // Results should be sorted by seeders in descending order (reverse)
        assert_eq!(search_results.len(), 3);
        assert_eq!(search_results[0].name, "High Seeds Movie");
        assert_eq!(search_results[0].seeders, 1000);
        assert_eq!(search_results[1].name, "Medium Seeds Movie");
        assert_eq!(search_results[1].seeders, 500);
        assert_eq!(search_results[2].name, "Low Seeds Movie");
        assert_eq!(search_results[2].seeders, 100);
    }

    #[test]
    fn test_search_deduplication() {
        let mut results = sample_results();
        results.push(results[0].clone()); // Add duplicate
        
        let mock_adapter = MockSourceAdapter::new(results);
        let searcher = Searcher::new().using(mock_adapter);
        
        let search_results = searcher.search("test");
        
        // Should be deduplicated
        assert_eq!(search_results.len(), 3);
    }

    #[test]
    fn test_search_empty_results() {
        let mock_adapter = MockSourceAdapter::new(vec![]);
        let searcher = Searcher::new().using(mock_adapter);
        
        let search_results = searcher.search("nonexistent");
        
        assert_eq!(search_results.len(), 0);
    }

    #[test]
    fn test_search_result_equality() {
        let result1 = SearchResult {
            name: "Test Movie".to_string(),
            seeders: 100,
            leechers: 10,
            size: "1.0 GB".to_string(),
            url: "magnet:test".to_string(),
        };

        let result2 = SearchResult {
            name: "Test Movie".to_string(),
            seeders: 200, // Different seeders
            leechers: 20, // Different leechers
            size: "2.0 GB".to_string(), // Different size
            url: "magnet:test".to_string(),
        };

        let result3 = SearchResult {
            name: "Test Movie".to_string(),
            seeders: 100,
            leechers: 10,
            size: "1.0 GB".to_string(),
            url: "magnet:different".to_string(), // Different URL
        };

        // Should be equal (same name and URL)
        assert_eq!(result1, result2);
        
        // Should not be equal (different URL)
        assert_ne!(result1, result3);
    }

    #[test]
    fn test_search_result_ordering() {
        let low = SearchResult {
            name: "Low".to_string(),
            seeders: 100,
            leechers: 10,
            size: "1.0 GB".to_string(),
            url: "magnet:low".to_string(),
        };

        let high = SearchResult {
            name: "High".to_string(),
            seeders: 1000,
            leechers: 100,
            size: "1.0 GB".to_string(),
            url: "magnet:high".to_string(),
        };

        assert!(high > low);
        assert!(low < high);
    }
}
