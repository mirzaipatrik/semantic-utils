use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;

pub fn get_embedding(query: &str) -> Result<Vec<f64>, reqwest::Error> {
    dotenv().ok();
    let hf_token = env::var("HF_TOKEN").expect("HF_TOKEN must be set");
    let embeddings_endpoint =
        env::var("EMBEDDINGS_ENDPOINT").expect("EMBEDDINGS_ENDPOINT must be set");
    let http_client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", hf_token).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());
    let data = json!({
        "inputs": query
    });

    let response = http_client
        .post(embeddings_endpoint)
        .headers(headers)
        .body(data.to_string())
        .send()?;

    let response_text = response.text()?;
    // Deserialize the response text into a Vec<f64>
    let embeddings: Vec<f64> = serde_json::from_str(&response_text)
        .expect("Failed to parse response as Vec<f64>");
    
    Ok(embeddings)
}
