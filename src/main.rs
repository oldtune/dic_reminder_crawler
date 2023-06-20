use std::{collections::HashMap, path::Path};

use crawler::{entovi::EnToViCrawler, WordCrawler, WordDefinition, WordTypeDefinition};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};
use tokio_postgres::{Client, NoTls};
mod crawler;
mod helper;
mod parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=123456 dbname=Dic_reminder_dictionary",
        NoTls,
    )
    .await?;

    //spawn connection
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprint!("Connection error: {}", err);
        }
    });

    setup(&client).await?;

    let word_types = get_word_types(&client).await?;

    let http_client = build_http_client()?;
    let mut buf_reader = read_dic_file("C:\\Users\\do.tran\\Downloads\\words_alpha.txt").await?;
    let mut lines = buf_reader.lines();
    let crawler = EnToViCrawler::new(http_client);

    if let Some(line) = lines.next_line().await? {
        let word_definition = crawler.crawl(&line).await;
        match word_definition {
            Err(err) => (),
            Ok(definition) => insert_word_into_db(&client, &definition, &word_types).await?,
        };
    }

    Ok(())
}

pub async fn insert_word_into_db(
    client: &Client,
    definition: &WordDefinition,
    word_type_hash: &HashMap<String, i32>,
) -> anyhow::Result<()> {
    client.execute("insert into word (word, en_uk_pronounce, en_us_pronounce, vi_pronounce) values ($1, '', '', $2)", &[&definition.word, &definition.pronounce]).await?;

    for word_definition in definition.type_and_definitions.iter() {
        let word_type: &str = &word_definition.word_type;
        let word_type_id = word_type_hash.get(word_type);
        if word_type_id.is_some() {
            insert_word_definition(
                &definition.word,
                &word_definition,
                *word_type_id.unwrap(),
                client,
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn insert_word_definition(
    word: &str,
    word_definition: &WordTypeDefinition,
    word_type_id: i32,
    client: &Client,
) -> anyhow::Result<()> {
    client
        .execute(
            "insert into word_type_link (word, word_type) values($1, $2)",
            &[&word, &word_type_id],
        )
        .await?;

    todo!()
}

async fn get_word_types(client: &Client) -> anyhow::Result<HashMap<String, i32>> {
    let mut result = HashMap::new();
    let word_types = client.query("SELECT * FROM WORD_TYPE", &[]).await?;
    for row in word_types {
        let id: i32 = row.get(0);
        let vi: String = row.get(1);
        result.insert(vi.to_lowercase(), id);
    }

    Ok(result)
}

async fn setup(client: &Client) -> anyhow::Result<()> {
    let word_types_count = client.query("SELECT count(*) from word_type", &[]).await?;

    let value: i64 = word_types_count[0].get(0);

    if value == 0 {
        client
            .execute(
                "insert into word_type(vi, en)
                    values
                    ('Danh từ', 'Pronounce'),
                    ('Nội động từ', 'Intransitive verb'),
                    ('Ngoại động từ', 'Transitive verb'),
                    ('Tính từ', 'Adjective'),
                    ('Phó từ', 'Adverb'),
                    ('Viết tắt', 'Abbreviation'),
                    ('Tiền tố', 'Prefix'),
                    ('Mạo từ', 'Article'),
                    ('Giới từ', 'Perposition'),
                    ('Đại từ', 'Pronouns'),
                    ('Liên từ','Conjunction'),
                    ('Thán từ','Interjection')",
                &[],
            )
            .await?;
    }

    Ok(())
}

fn build_http_client() -> reqwest::Result<reqwest::Client> {
    let client_builder = reqwest::ClientBuilder::new().gzip(true);
    let client = client_builder.build();
    client
}

async fn read_dic_file<P>(path: P) -> anyhow::Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(path).await?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader)
}
