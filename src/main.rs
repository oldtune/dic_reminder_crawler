use reqwest::Client;
use tl::ParserOptions;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let client = build_http_client()?;
    let endpoint = construct_query_endpoint("access");
    let html_content = fetch_html(&client, &endpoint).await?;
    parse_html_content(&html_content);
    Ok(())
}

fn build_http_client() -> reqwest::Result<reqwest::Client> {
    let client_builder = reqwest::ClientBuilder::new().gzip(true);
    let client = client_builder.build();
    client
}

fn parse_html_content(html_content: &str) {
    let dom = tl::parse(html_content, ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let query_selector_iter = dom.query_selector("h2.fl");
    if let Some(elements) = query_selector_iter {
        for el in elements {
            let node = el.get(parser);
            if let Some(actual_node) = node {
                match actual_node {
                    tl::Node::Tag(html) => println!("{}", html.inner_text(parser)),
                    _ => (),
                }
            }
        }
    }
}

#[inline]
fn construct_query_endpoint(word: &str) -> String {
    format!("https://dict.laban.vn/find?type=1&query=access")
}

async fn fetch_html(client: &Client, endpoint: &'_ str) -> reqwest::Result<String> {
    let response = client
        .get(endpoint)
        .send()
        .await?
        .text_with_charset("utf-8")
        .await?;
    Ok(response)
}
