use remote_media_pi::torrent::search::Searcher;
use crate::mocks::torrent_source::MockTorrentSource;
use crate::fixtures::test_data::sample_search_results;

#[test]
fn test_searcher_with_mock_adapter() {
    let results = sample_search_results();
    let mock_source = MockTorrentSource::default().with_results(results.clone());
    
    let searcher = Searcher::new().using(mock_source);
    let search_results = searcher.search("test query");
    
    // Results should be sorted by seeders in descending order
    assert_eq!(search_results.len(), 3);
    assert_eq!(search_results[0].name, "Test Movie 2023");
    assert_eq!(search_results[0].seeders, 1500);
    assert_eq!(search_results[1].name, "Another Show S01E01"); 
    assert_eq!(search_results[1].seeders, 800);
    assert_eq!(search_results[2].name, "Old Movie 1999");
    assert_eq!(search_results[2].seeders, 250);
}

#[test]
fn test_searcher_deduplication() {
    let mut results = sample_search_results();
    // Add duplicate
    results.push(results[0].clone());
    
    let mock_source = MockTorrentSource::default().with_results(results);
    let searcher = Searcher::new().using(mock_source);
    let search_results = searcher.search("test query");
    
    // Should be deduplicated
    assert_eq!(search_results.len(), 3);
}

#[test]
fn test_searcher_empty_results() {
    let mock_source = MockTorrentSource::default().with_results(vec![]);
    let searcher = Searcher::new().using(mock_source);
    let search_results = searcher.search("nonexistent");
    
    assert_eq!(search_results.len(), 0);
}