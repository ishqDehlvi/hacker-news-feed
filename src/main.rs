use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Story {
    title: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    hits: Vec<Story>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = "https://hn.algolia.com/api/v1/search?tags=front_page";
    let res = client.get(url).send().await?.json::<ApiResponse>().await?;

    for story in res.hits {
        println!("{}", story.title);
    }

    Ok(())
}
