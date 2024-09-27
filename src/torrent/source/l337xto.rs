use std::sync::Arc;

use scraper::{Html, Selector};

use crate::torrent::{downloader::{Downloader, SourceAdapter}, search::SearchResult};

const BASE_URL: &str = "https://1337xx.to";

pub struct L337xTo {
    downloader: Box<dyn Downloader>,
}

impl L337xTo {
    pub fn new(downloader: Box<dyn Downloader>) -> Arc<Self> {
        Arc::new(Self { downloader })
    }
}

impl SourceAdapter for L337xTo {
    fn build_url(&self, terms: &str) -> String {
        format!("{}/search/{}/1/", BASE_URL, terms)
    }

    fn get_document(&self, url: String) -> Option<String> {
        self.downloader.download(url)
    }

    fn select_results(&self, fragment: Html) -> Vec<SearchResult> {
        let mut search_results = Vec::new();

        let selector = Selector::parse("tr").unwrap();
        let name_selector = Selector::parse(".name").unwrap();
        let seeders_selector = Selector::parse(".seeds").unwrap();
        let leechers_selector = Selector::parse(".leeches").unwrap();
        let size_selector = Selector::parse(".size").unwrap();
        let url_selector = Selector::parse(".name a:nth-child(2)").unwrap();
        let magnet_selector = Selector::parse("a[href^=magnet]").unwrap();

        for row in fragment.select(&selector).take(6) {
            let name_sel = row.select(&name_selector).next();
            let seeders_sel = row.select(&seeders_selector).next();
            let leechers_sel = row.select(&leechers_selector).next();
            let size_sel = row.select(&size_selector).next();
            let url_sel = row.select(&url_selector).next();
            let (name, seeders, leechers, size, url);

            let (Some(name_s), Some(seed_s), Some(leech_s), Some(size_s), Some(url_s)) =
            (name_sel, seeders_sel, leechers_sel, size_sel, url_sel) else {
                continue;
            };

            let found = name_s.text().map(String::from).collect::<String>();
            warn!("name: {}", found);
            name = found;

            let found = seed_s.text().map(String::from).collect::<String>();
            warn!("seeders: {}", found);
            seeders = found;

            let found = leech_s.text().map(String::from).collect::<String>();
            warn!("leechers: {}", found);
            leechers = found;

            let found = size_s.text().map(String::from).collect::<String>();
            let found_num = found.split_whitespace().next();
            warn!("size: {}", found_num.unwrap_or_default());
            size = String::from(found_num.unwrap_or_default());

            let found = url_s.value().attr("href");
            let Some(href) = found  else {
                continue;
            };

            warn!("url: {}", href);
            let magnet_url = format!("{}{}", BASE_URL, String::from(href));
            let document = self.get_document(magnet_url).unwrap();
            url = self
                .find_magnet(&document, &magnet_selector)
                .unwrap_or_default();
            warn!("magnet: {}", url);

            let result = SearchResult {
                name,
                seeders,
                leechers,
                size,
                url,
            };

            search_results.push(result);
        }

        search_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeDownloader;

    impl FakeDownloader {
        fn new() -> Self {
            Self
        }
    }

    impl Downloader for FakeDownloader {
        fn download(&self, _url: String) -> Option<String> {
            fake()
        }
    }

    #[test]
    fn build_url() {
        let url = get_adapter().build_url("test");
        assert_eq!(url, "https://1337xx.to/search/test/1/");
    }

    #[test]
    fn html_parse() {
        let adapter = get_adapter();
        for result in adapter
            .select_results(adapter.scrap_from_document(fake().unwrap()))
            .iter()
            .take(1)
        {
            assert_eq!("My.Torrent.Name", result.name);
            assert_eq!("2992", result.seeders);
            assert_eq!("173", result.leechers);
            assert_eq!("222.4", result.size);
            assert_eq!("magnet:My.Magnet.Link", result.url);
        }

        let magnet_selector = Selector::parse("a[href^=magnet]").unwrap();
        let magnet = adapter
            .find_magnet(&fake().unwrap(), &magnet_selector)
            .unwrap();
        assert_eq!("magnet:My.Magnet.Link", magnet);
    }

    fn get_adapter() -> Arc<impl SourceAdapter> {
        L337xTo::new(Box::new(FakeDownloader::new()))
    }

    fn fake() -> Option<String> {
        Some(r#"
            <!-- Torrents -->
            <table class="table-list table table-responsive table-striped">
            <thead>
            <tr>
            <th class="coll-1 name">name</th>
            <th class="coll-2">se</th>
            <th class="coll-3">le</th>
            <th class="coll-date">time</th>
            <th class="coll-4"><span class="size">size</span> <span class="info">info</span></th>
            <th class="coll-5">uploader</th>
            </tr>
            </thead>
            <tbody>
            <tr>
            <td class="coll-1 name"><a href="/sub/6/0/" class="icon"><i class="flaticon-divx"></i></a><a href="/torrent/My.Torrent.Name/">My.Torrent.Name</a></td>
            <td class="coll-2 seeds">2992</td>
            <td class="coll-3 leeches">173</td>
            <td class="coll-date">Jul. 11th '18</td>
            <td class="coll-4 size mob-user">222.4 MB<span class="seeds">2992</span></td>
            <td class="coll-5 user"><a href="/user/EZTVag/">EZTVag</a></td>
            </tr>
            </tbody>
            </table>
            <!-- Magnet -->
            <div class="col-9 page-content">
            <div class="box-info torrent-detail-page  vpn-info-wrap">
            <div class="box-info-heading clearfix"><h1> My.Torrent.Name </h1>
            </div>
            <div class="lf3ba6c418b5f4ee4e1f50b3fbfdced465c96e2d0 no-top-radius">
            <div class="lce207072b5a6a519bafb45db419be47c2d331555 clearfix">
            <ul class="le85aec8b46f88def3aed2f8996f85ac2edd53594 l1b1397bcdc13f88822df0b36abc27cdf17bbd6c5">
            <li><a class="lb6031b0d322cf2f9769768fa16a450c23c954366 l986ff5effa3ea4d6ac47830547ffebda8130f266 ld3a91ea78222b30937f8a1c6b14fcd7e17c8a2a6" href="magnet:My.Magnet.Link" onclick="javascript: count(this);"><span class="icon"><i class="flaticon-l1b18a30c95d2bb796ec169ab55ac2b8c03e90298"></i></span>Magnet Download</a> </li>
            <li style="margin-top:0px;"></li>
            <li class="dropdown">
            </ul>
            </li>
        "#.to_string())
    }
}
