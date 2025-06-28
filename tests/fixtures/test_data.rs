use remote_media_pi::torrent::{
    client::Torrent,
    search::SearchResult,
};

pub fn sample_torrents() -> Vec<Torrent> {
    vec![
        Torrent {
            id: 1,
            name: "Test Movie 2023".to_string(),
            status: "downloading".to_string(),
            size: 1610612736, // 1.5 GB in bytes
            progress: 75,
        },
        Torrent {
            id: 2,
            name: "Another Show S01E01".to_string(),
            status: "done".to_string(),
            size: 786432000, // 750 MB in bytes
            progress: 100,
        },
        Torrent {
            id: 3,
            name: "Old Movie 1999".to_string(),
            status: "paused".to_string(),
            size: 2147483648, // 2 GB in bytes
            progress: 25,
        },
    ]
}

pub fn sample_search_results() -> Vec<SearchResult> {
    vec![
        SearchResult {
            name: "Test Movie 2023".to_string(),
            seeders: 1500,
            leechers: 200,
            size: "1.5 GB".to_string(),
            url: "magnet:?xt=urn:btih:123456789abcdef&dn=Test+Movie+2023".to_string(),
        },
        SearchResult {
            name: "Another Show S01E01".to_string(),
            seeders: 800,
            leechers: 50,
            size: "750 MB".to_string(),
            url: "magnet:?xt=urn:btih:987654321fedcba&dn=Another+Show+S01E01".to_string(),
        },
        SearchResult {
            name: "Old Movie 1999".to_string(),
            seeders: 250,
            leechers: 10,
            size: "2.0 GB".to_string(),
            url: "magnet:?xt=urn:btih:abcdef123456789&dn=Old+Movie+1999".to_string(),
        },
    ]
}