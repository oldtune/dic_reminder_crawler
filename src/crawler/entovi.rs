use std::{borrow::Borrow, thread::current};

use anyhow::bail;
use async_trait::async_trait;
use tl::{HTMLTag, ParserOptions, VDom};

use crate::{helper::string_helper::join_split, parser::Parser};

use super::{Meaning, WordCrawler, WordDefinition, WordType, WordTypeDefinition};

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

    // fn extract_word(&self, dom: &VDom) -> Option<String> {
    //     let parser = dom.parser();
    //     let query_selector_iter = dom.query_selector("div.w.fl");

    //     if query_selector_iter.is_none() {
    //         return None;
    //     }

    //     let elements = query_selector_iter.unwrap();

    //     for el in elements {
    //         let node = el.get(parser);
    //         if node.is_none() {
    //             continue;
    //         }

    //         let actual_node = node.unwrap();

    //         match actual_node {
    //             tl::Node::Tag(html) => {
    //                 return Some(html.inner_text(&parser).to_string());
    //             }
    //             _ => continue,
    //         }
    //     }

    //     None
    // }

    // fn query_selector_html_inner(&self, dom: &VDom) -> Option<String> {
    //     let parser = dom.parser();
    //     let query_selector_iter = dom.query_selector("div.p5l.fl.cB");

    //     if query_selector_iter.is_none() {
    //         return None;
    //     }

    //     let elements = query_selector_iter.unwrap();

    //     for el in elements {
    //         let node = el.get(parser);
    //         if node.is_none() {
    //             continue;
    //         }

    //         let actual_node = node.unwrap();

    //         match actual_node {
    //             tl::Node::Tag(html) => {
    //                 return Some(html.inner_text(&parser).to_string());
    //             }
    //             _ => continue,
    //         }
    //     }

    //     None
    // }

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

        let mut html_tags = parser.query_selector_elements("[id^='partofspeech']");

        html_tags = html_tags
            .into_iter()
            .filter(|x| !parser.has_id(x, "partofspeech_100"))
            .collect();

        for tag in html_tags {
            let divs = parser.query_selector(tag, "div");
        }

        return Some(WordDefinition::new(
            &word.unwrap(),
            &pronounce.unwrap_or("".to_string()),
        ));
    }

    fn extract_word_type_definition(
        &self,
        tags: Vec<&HTMLTag>,
        parser: &Parser,
    ) -> Vec<WordTypeDefinition> {
        let mut result = vec![];

        let mut current_word_type: Option<WordTypeDefinition> = None;

        for tag in tags {
            if parser.has_class(tag, "ub") {
                let inner_text = parser.inner_text(tag);
                let word_type = self.parse_word_type(&inner_text);
                if word_type.is_none() {
                    let error = format!("Cannot parse this word type {}", inner_text);
                    println!("{}", error);
                    panic!("encoutered error");
                }

                if current_word_type.is_some() {
                    result.push(current_word_type.unwrap());
                }

                current_word_type = Some(WordTypeDefinition {
                    word_type: word_type.unwrap(),
                    meaning: vec![],
                });
            }

            if parser.has_class(tag, "m") {
                //add word
            }
            if parser.has_class(tag, "e") {
                //add word
            }
            if parser.has_class(tag, "em") {
                //add word
            }
        }

        if current_word_type.is_some() {
            result.push(current_word_type.unwrap());
        }

        result
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
