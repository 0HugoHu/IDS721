use std::error::Error;
use std::io::Cursor;
use std::sync::Arc;

use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use tokio::sync::Semaphore;

const URL: &str = "https://news.ycombinator.com/";
const MAX_CONCURRENT_REQUESTS: usize = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Arc::new(Client::new());
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    let response = client.get(URL).send().await?;
    let bytes = response.bytes().await?;
    let document = Document::from_read(Cursor::new(bytes))?;

    let article_links: Vec<String> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|l| {
            !l.starts_with("item?")
                && !l.starts_with("from?")
                && !l.starts_with("vote?")
                && !l.starts_with("https://news.ycombinator.com")
        })
        .take(30)
        .map(String::from)
        .collect();

    let mut tasks = Vec::new();

    for link in article_links {
        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);
        let task = tokio::spawn(async move {
            let _permit = semaphore.acquire().await;
            match fetch_title(&client, &link).await {
                Ok(title) => {
                    println!("{} - {}", link, title);
                }
                Err(e) => {
                    eprintln!("Error fetching {}: {}", link, e);
                }
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}

async fn fetch_title(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    let document = Document::from_read(Cursor::new(bytes))?;
    let title = document
        .find(Name("h1"))
        .next()
        .ok_or("Title not found")?
        .text();

    Ok(title)
}
