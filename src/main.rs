use anyhow::Ok;
use crawler::{
    entovi::EnToViCrawler, Example, Meaning, WordCrawler, WordDefinition, WordTypeDefinition,
};
use std::{collections::HashMap, env, path::Path};
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
    env::set_var("RUST_BACKTRACE", "full");

    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=admin dbname=dictionary",
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

    loop {
        if let Some(line) = lines.next_line().await? {
            println!("Crawl {}", &line);

            let word_definition = crawler.crawl(&line).await;
            match word_definition {
                Err(_) => (),
                std::result::Result::Ok(definition) => {
                    if definition.word != line {
                        continue;
                    }

                    insert_word_into_db(&client, &definition, &word_types).await?;

                    println!("Crawled {}", &definition.word);
                }
            };
        } else {
            break;
        }
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
    let row = client
        .query_one(
            "insert into word_type_link (word, word_type) values($1, $2) returning id",
            &[&word, &word_type_id],
        )
        .await?;

    let word_link_id: i32 = row.get(0);

    for meaning in word_definition.meaning.iter() {
        insert_meaning(client, meaning, word_link_id).await?;
    }

    Ok(())
}

async fn insert_meaning(
    client: &Client,
    word_meaning: &Meaning,
    word_link_id: i32,
) -> anyhow::Result<()> {
    let row = client
        .query_one(
            "insert into word_meaning (word_type_link_id, vi_meaning, en_meaning) values($1,$2,$3) returning id",
            &[&word_link_id, &"".to_string(), &word_meaning.meaning],
        )
        .await?;

    let id: i32 = row.get(0);

    for example in word_meaning.examples.iter() {
        insert_example(client, id, example).await?;
    }

    Ok(())
}

async fn insert_example(
    client: &Client,
    word_meaning_id: i32,
    example: &Example,
) -> anyhow::Result<()> {
    client
        .execute(
            "insert into example (word_meaning_id, en_example, vi_meaning) values ($1, $2, $3)",
            &[&word_meaning_id, &example.sentence, &example.meaning],
        )
        .await?;
    Ok(())
}

async fn get_word_types(client: &Client) -> anyhow::Result<HashMap<String, i32>> {
    let mut result = HashMap::new();
    let word_types = client.query("select * from word_type", &[]).await?;
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
