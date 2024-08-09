use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, ACCEPT, CONTENT_TYPE};
use serde_json::json;
use std::env;
use std::error::Error;

pub fn get_stories(query: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let dato_cms_token = env::var("DATOCMS_API_TOKEN")?;
    let embeddings_endpoint = "https://graphql.datocms.com/";
    let http_client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", dato_cms_token).parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(ACCEPT, "application/json".parse()?);

    let query_body = json!({
        "query": query
    });

    let response = http_client
        .post(embeddings_endpoint)
        .headers(headers)
        .body(query_body.to_string())
        .send()?;

    let response_text = response.text()?;
    Ok(response_text)
}
