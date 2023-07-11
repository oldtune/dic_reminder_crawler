use crate::parser::Parser;

use super::{WordCrawler, WordDefinition, WordTypeDefinition};
use anyhow::{Context, Result};
use async_trait::async_trait;
use tl::{ParserOptions, VDom};

pub struct EnToEnCrawler {
    client: reqwest::Client,
}

impl EnToEnCrawler {
    pub fn new(http_client: reqwest::Client, parser: &Parser) -> Self {
        Self {
            client: http_client,
        }
    }
}

pub trait Crawler {
    fn crawl(word: &str) -> anyhow::Result<WordDefinition>;
}

pub trait HTMLExtractor {
    fn extract_word(&self, html: &str) -> anyhow::Result<WordDefinition>;
}

pub struct DictionaryCambridgeExtractor {}

impl HTMLExtractor for DictionaryCambridgeExtractor {
    fn extract_word(&self, html: &str) -> anyhow::Result<WordDefinition> {
        let html_doc = tl::parse(html, ParserOptions::default())
            .context("Failed to create html doc from html string")?;
        let dictionary_block = self.extract_dictionary_block_from_html_doc();
        
        let mut first_word = dictionary_block.into_iter().take(1);
    }
}

impl DictionaryCambridgeExtractor {
    fn extract_dictionary_block_from_html_doc(&self, dom: &tl::VDom) -> Vec<WordDefinition>{
        let mut result = vec![];

       let block_iter = dom.query_selector("div.pr.entry-body__el");

       if block_iter.is_none(){
        return result;
       }

      for node in block_iter.unwrap(){ 
        let el = node.get(dom.parser());
        if el.is_none(){
            continue;
        }
        
        let tag = el.unwrap().as_tag();
        
        if tag.is_none(){
            continue;
        }

        let tag = tag.unwrap();
      }
    }

    fn extract_list_word_type_from_block(dom: &VDom, tag: )

    fn extract_word_title() -> {

    }

}