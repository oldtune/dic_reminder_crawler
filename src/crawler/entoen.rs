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
