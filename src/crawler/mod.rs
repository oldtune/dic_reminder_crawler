pub mod entoen;
pub mod entovi;

pub trait WordCrawler {
    fn crawl(&self, word: &str) -> Result<WordDefinition, CrawlError>;
}

pub enum CrawlError {}

pub struct WordDefinition {
    pub word: String,
    pub type_and_definitions: Vec<WordTypeDefinition>,
}

pub struct WordTypeDefinition {
    pub word_type: WordType,
    pub meaning: Vec<String>,
    pub example: Vec<String>,
}

pub enum WordType {
    Verb,
    Noun,
    Pronounce,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Intersection,
}
