use std::borrow::Borrow;

use anyhow::bail;
use async_trait::async_trait;
use tl::{ParserOptions, VDom};

use crate::helper::string_helper::join_split;

use super::{WordCrawler, WordDefinition};

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

    fn parse_html_content(&self, html_content: &str) -> anyhow::Result<WordDefinition> {
        let dom = tl::parse(html_content, ParserOptions::default())?;
        let word = self.extract_word(&dom);
        if word.is_none() {
            bail!("Doesn't contain a word")
        }
        let pronounce = self.query_selector_html_inner(&dom);

        Ok(WordDefinition::new(
            &word.unwrap(),
            &pronounce.unwrap_or("".to_string()),
        ))
    }

    fn extract_word(&self, dom: &VDom) -> Option<String> {
        let parser = dom.parser();
        let query_selector_iter = dom.query_selector("div.w.fl");

        if query_selector_iter.is_none() {
            return None;
        }

        let elements = query_selector_iter.unwrap();

        for el in elements {
            let node = el.get(parser);
            if node.is_none() {
                continue;
            }

            let actual_node = node.unwrap();

            match actual_node {
                tl::Node::Tag(html) => {
                    return Some(html.inner_text(&parser).to_string());
                }
                _ => continue,
            }
        }

        None
    }

    fn query_selector_html_inner(&self, dom: &VDom) -> Option<String> {
        let parser = dom.parser();
        let query_selector_iter = dom.query_selector("div.p5l.fl.cB");

        if query_selector_iter.is_none() {
            return None;
        }

        let elements = query_selector_iter.unwrap();

        for el in elements {
            let node = el.get(parser);
            if node.is_none() {
                continue;
            }

            let actual_node = node.unwrap();

            match actual_node {
                tl::Node::Tag(html) => {
                    return Some(html.inner_text(&parser).to_string());
                }
                _ => continue,
            }
        }

        None
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
}

#[async_trait]
impl WordCrawler for EnToViCrawler {
    async fn crawl(&self, word: &str) -> anyhow::Result<super::WordDefinition> {
        let endpoint = self.construct_query_endpoint(word);
        let html = self.fetch_html(&endpoint).await?;
        let definitions = self.parse_html_content(&html);
        definitions
    }
}