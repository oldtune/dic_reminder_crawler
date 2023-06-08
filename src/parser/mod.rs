use crate::crawler::WordDefinition;

pub struct Parser<'a> {
    inner_html: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(html: &'a str) -> Self {
        Self { inner_html: html }
    }

    pub fn query_selector<P>(selector: P) -> anyhow::Result<String>
    where
        P: AsRef<&'a str>,
    {
        todo!()
    }
}
