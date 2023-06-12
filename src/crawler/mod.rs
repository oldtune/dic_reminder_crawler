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
    pub fn new(word: &str, pronounce: &str) -> Self {
        Self {
            word: word.to_string(),
            pronounce: pronounce.to_string(),
            type_and_definitions: vec![],
        }
    }
}

#[derive(Debug)]
pub struct WordTypeDefinition {
    pub word_type: WordType,
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
    Verb,
    Noun,
    Pronounce,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Intersection,
    Article,
}
