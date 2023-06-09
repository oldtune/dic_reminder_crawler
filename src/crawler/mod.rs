use std::vec;

use anyhow::Error;
use async_trait::async_trait;
use reqwest::Request;

pub mod entoen;
pub mod entovi;

#[async_trait]
pub trait WordCrawler {
    async fn crawl(&self, word: &str) -> anyhow::Result<WordDefinition>;
}

// #[derive(Error, Debug)]
// pub enum CrawlError {
//     #[error("Failed to send request")]
//     RequestError(#[from] reqwest::Error),
//     #[error("Failed to parse html content")]
//     ParsingError,
//     OtherError(#[from] String),
// }

#[derive(Debug)]
pub struct WordDefinition {
    pub word: String,
    pub pronounce: String,
    pub type_and_definitions: Vec<WordTypeDefinition>,
}

impl WordDefinition {
    pub fn new(word: &str, pronounce: &str, word_type_definition: Vec<WordTypeDefinition>) -> Self {
        Self {
            word: word.to_string(),
            pronounce: pronounce.to_string(),
            type_and_definitions: word_type_definition,
        }
    }
}

#[derive(Debug)]
pub struct WordTypeDefinition {
    pub word_type: String,
    pub meaning: Vec<Meaning>,
}

#[derive(Debug)]
pub struct Meaning {
    pub meaning: String,
    pub examples: Vec<Example>,
}

#[derive(Debug)]
pub struct Example {
    pub sentence: String,
    pub meaning: String,
}

#[derive(Debug)]
pub enum WordType {
    //ngoai dong tu
    TransitiveVerb,
    //noi dong tu
    InTransitiveVerb,
    Noun,
    Pronounce,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Intersection,
    Article,
}
