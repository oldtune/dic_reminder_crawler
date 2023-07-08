use anyhow::{bail, Context};
use async_trait::async_trait;
use tl::HTMLTag;

use crate::{helper::string_helper::join_split, parser::Parser};

use super::{Example, Meaning, WordCrawler, WordDefinition, WordTypeDefinition};

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

    async fn fetch_html(&self, endpoint: &'_ str) -> anyhow::Result<String> {
        let response = self
            .client
            .get(endpoint)
            .send()
            .await
            .context("Failed to fetch html")?
            .text_with_charset("utf-8")
            .await
            .context("Failed to convert to text with charset utf8")?;
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

        let mut word_type_definition = vec![];

        for type_definition_tag in word_types.iter() {
            let definition = self.extract_type_definition(type_definition_tag, parser);
            if definition.is_some() {
                word_type_definition.push(definition.unwrap());
            }
        }

        return Some(WordDefinition::new(
            &word.unwrap(),
            &pronounce.unwrap_or("".to_string()),
            word_type_definition,
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
                    Some(word_type) => {
                        let last_meaning = word_type.meaning.last_mut();

                        if last_meaning.is_none() {
                            continue;
                        }

                        let last_meaning = last_meaning.unwrap();

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
                    Some(word_type) => {
                        let last_meaning = word_type.meaning.last_mut();
                        if last_meaning.is_none() {
                            continue;
                        };

                        let last_meaning = last_meaning.unwrap();

                        let last_example = last_meaning.examples.last_mut().unwrap();
                        last_example.meaning = parser.inner_text(div_tag);
                    }
                    None => (),
                }
            }
        }

        word_type
    }
}

#[async_trait]
impl WordCrawler for EnToViCrawler {
    async fn crawl(&self, word: &str) -> anyhow::Result<super::WordDefinition> {
        let endpoint = self.construct_query_endpoint(word);
        let html = self
            .fetch_html(&endpoint)
            .await
            .context("Failed to fetch Html")?;
        let parser = Parser::new(&html).context("Failed to create parser html")?;
        let definition = self.get_definition(&parser);
        if definition.is_none() {
            //log not found
            bail!(format!("Word {} not found", word));
        }
        Ok(definition.unwrap())
    }
}
