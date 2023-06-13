use std::{
    borrow::{Borrow, BorrowMut},
    thread::current,
};

use anyhow::bail;
use async_trait::async_trait;
use tl::{HTMLTag, ParserOptions, VDom};

use crate::{helper::string_helper::join_split, parser::Parser};

use super::{Example, Meaning, WordCrawler, WordDefinition, WordType, WordTypeDefinition};

pub struct EnToViCrawler {
    client: reqwest::Client,
}

impl EnToViCrawler {
    pub fn new(http_client: reqwest::Client) -> Self {
        Self {
            client: http_client,
        }
    }

    #[inline]
    fn construct_query_endpoint(&self, word: &str) -> String {
        if word.trim().contains(" ") {
            let split = word.split(" ");
            let split_joined = join_split(split, '+');
            return format!(
                "http://tratu.coviet.vn/hoc-tieng-anh/tu-dien/lac-viet/A-V/{}.html",
                split_joined
            );
        }

        format!(
            "http://tratu.coviet.vn/hoc-tieng-anh/tu-dien/lac-viet/A-V/{}.html",
            word
        )
    }

    async fn fetch_html(&self, endpoint: &'_ str) -> reqwest::Result<String> {
        let response = self
            .client
            .get(endpoint)
            .send()
            .await?
            .text_with_charset("utf-8")
            .await?;
        Ok(response)
    }

    fn get_definition(&self, parser: &Parser) -> Option<WordDefinition> {
        let word = parser.query_selector_first_element_inner_text("div.w.fl");
        if word.is_none() {
            return None;
        }

        let pronounce = parser.query_selector_first_element_inner_text("div.p5l.fl.cB");

        let mut word_types = parser.query_selector_elements("[id^='partofspeech']");

        word_types = word_types
            .into_iter()
            .filter(|x| !parser.has_id(x, "partofspeech_100"))
            .collect();

        for type_definition_tag in word_types.iter() {
            dbg!(self.extract_type_definition(type_definition_tag, parser));
            //get definition here
        }

        return Some(WordDefinition::new(
            &word.unwrap(),
            &pronounce.unwrap_or("".to_string()),
        ));
    }

    pub fn extract_type_definition(
        &self,
        tag: &HTMLTag,
        parser: &Parser,
    ) -> Option<WordTypeDefinition> {
        let inner_tags = parser.query_selector(tag, "div");
        //must present at least 1 ub and a m
        let mut word_type = None;
        for div_tag in inner_tags {
            if parser.has_class(div_tag, "ub") {
                word_type = Some(WordTypeDefinition {
                    word_type: parser.inner_text(div_tag),
                    meaning: vec![],
                })
            }
            if parser.has_class(div_tag, "m") {
                let word = word_type.as_mut();
                match word {
                    Some(mut word_type) => word_type.meaning.push(Meaning {
                        meaning: parser.inner_text(div_tag),
                        examples: vec![],
                    }),
                    None => (),
                }
            }
            if parser.has_class(div_tag, "e") {
                let word = word_type.as_mut();
                match word {
                    Some(mut word_type) => {
                        let mut last_meaning = word_type.meaning.last_mut().unwrap();
                        last_meaning.examples.push(Example {
                            sentence: parser.inner_text(div_tag),
                            meaning: "".to_string(),
                        });
                    }
                    None => (),
                }
            }
            if parser.has_class(div_tag, "em") {
                let word = word_type.as_mut();
                match word {
                    Some(mut word_type) => {
                        let mut last_meaning = word_type.meaning.last_mut().unwrap();
                        let mut last_example = last_meaning.examples.last_mut().unwrap();
                        last_example.meaning = parser.inner_text(div_tag);
                    }
                    None => (),
                }
            }
        }

        word_type
    }

    fn parse_word_type(&self, vi_type: &str) -> Option<WordType> {
        match vi_type {
            "phó từ" => Some(WordType::Article),
            _ => None,
        }
    }
}

#[async_trait]
impl WordCrawler for EnToViCrawler {
    async fn crawl(&self, word: &str) -> anyhow::Result<super::WordDefinition> {
        let endpoint = self.construct_query_endpoint(word);
        let html = self.fetch_html(&endpoint).await?;
        let parser = Parser::new(&html)?;
        let definition = self.get_definition(&parser);
        if definition.is_none() {
            bail!("word not found");
        }
        Ok(definition.unwrap())
    }
}

pub enum ParseError {
    NotUbClass(String),
    NotEClass(String),
    NotEmClass(String),
    NotMClass(String),
}
