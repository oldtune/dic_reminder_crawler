use super::WordCrawler;

pub struct EnToViCrawler {
    client: reqwest::Client,
}

impl EnToViCrawler {
    pub fn new(http_client: reqwest::Client) -> Self {
        Self {
            client: http_client,
        }
    }
}

impl WordCrawler for EnToViCrawler {
    fn crawl(&self, word: &str) -> Result<super::WordDefinition, super::CrawlError> {
        todo!()
    }
}
