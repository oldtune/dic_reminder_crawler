use crawler::{entovi::EnToViCrawler, WordCrawler};
use reqwest::Client;
use tl::ParserOptions;
mod crawler;
mod helper;
mod parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_client = build_http_client()?;
    let crawler = EnToViCrawler::new(http_client);
    let word_definition = crawler.crawl("access").await?;
    dbg!(word_definition);
    Ok(())
}

fn build_http_client() -> reqwest::Result<reqwest::Client> {
    let client_builder = reqwest::ClientBuilder::new().gzip(true);
    let client = client_builder.build();
    client
}
