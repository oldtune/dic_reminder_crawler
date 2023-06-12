use std::vec;

use tl::{HTMLTag, ParserOptions, VDom};

pub type InnerText = String;

pub struct Parser<'a> {
    dom: VDom<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(html: &'a str) -> anyhow::Result<Self> {
        let dom = tl::parse(html, ParserOptions::default())?;
        Ok(Self { dom })
    }

    pub fn query_selector<P>(&self, tag: &HTMLTag, selector: P) -> Vec<&HTMLTag>
    where
        P: AsRef<str>,
    {
        let query_selector_iter = tag.query_selector(self.dom.parser(), selector.as_ref());

        let mut result = vec![];
        if query_selector_iter.is_none() {
            return result;
        }
        let node_handles = query_selector_iter.unwrap();

        for node_handle in node_handles {
            let actual_node = node_handle.get(self.dom.parser());
            if actual_node.is_none() {
                continue;
            }

            //try to convert node -> html tag, if not a tag, just ignore it
            let html_tag = actual_node.unwrap().as_tag();
            if html_tag.is_none() {
                continue;
            }

            result.push(html_tag.unwrap());
        }

        result
    }

    pub fn inner_text(&self, tag: &HTMLTag) -> String {
        tag.inner_text(self.dom.parser()).to_string()
    }

    pub fn has_class(&self, tag: &HTMLTag, class: &str) -> bool {
        let attributes = tag.attributes();
        for attribute in attributes.iter() {
            if attribute.0 == "class"
                && attribute.1.is_some()
                && attribute.1.unwrap().contains(class)
            {
                return true;
            }
        }
        false
    }

    pub fn query_selector_first_element_inner_text<P>(&self, selector: P) -> Option<InnerText>
    where
        P: AsRef<str>,
    {
        let tags = self.query_selector_elements(selector);

        if let Some(tag) = tags.first() {
            return Some((*tag.inner_text(self.dom.parser())).to_string());
        }

        None
    }

    //get html tag from query selector, ignores all nodes even we use nodes api
    pub fn query_selector_elements<P>(&self, selector: P) -> Vec<&HTMLTag>
    where
        P: AsRef<str>,
    {
        let parser = self.dom.parser();
        let query_selector_iter = self.dom.query_selector(selector.as_ref());

        let mut result = vec![];
        if query_selector_iter.is_none() {
            return result;
        }
        let node_handles = query_selector_iter.unwrap();

        for node_handle in node_handles {
            let actual_node = node_handle.get(parser);
            if actual_node.is_none() {
                continue;
            }

            //try to convert node -> html tag, if not a tag, just ignore it
            let html_tag = actual_node.unwrap().as_tag();
            if html_tag.is_none() {
                continue;
            }

            result.push(html_tag.unwrap());
        }

        result
    }

    pub fn has_id(&self, tag: &HTMLTag, id: &str) -> bool {
        let attributes = tag.attributes();
        for attribute in attributes.iter() {
            if attribute.0 == "id" && !attribute.1.is_none() && attribute.1.unwrap() == id {
                return true;
            }
        }

        false
    }
}
