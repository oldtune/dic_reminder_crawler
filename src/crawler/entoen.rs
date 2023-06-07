use super::WordCrawler;

pub struct EnToEnCrawler {
    client: reqwest::Client,
}

impl EnToEnCrawler {
    pub fn new(http_client: reqwest::Client) -> Self {
        Self {
            client: http_client,
        }
    }
}

impl WordCrawler for EnToEnCrawler {
    fn crawl(&self, word: &str) -> Result<super::WordDefinition, super::CrawlError> {
        todo!()
    }
}
