use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::env;
use std::error::Error;

pub fn get_embedding(query: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    dotenv().ok();

    let hf_token = env::var("HF_TOKEN")?;
    let embeddings_endpoint = env::var("EMBEDDINGS_ENDPOINT")?;

    let http_client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", hf_token).parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let data = json!({
        "inputs": query
    });

    let response = http_client
        .post(embeddings_endpoint)
        .headers(headers)
        .body(data.to_string())
        .send()?;

    if response.status().is_success() {
        let response_text = response.text()?;

        let json_value: Value =
            serde_json::from_str(&response_text).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        if let Some(inner_array) = json_value
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_array())
        {
            let embeddings: Vec<f32> = inner_array
                .iter()
                .filter_map(|v| v.as_f64())
                .map(|v| v as f32)
                .collect();
            Ok(embeddings)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid response format".to_string(),
            )))
        }
    } else {
        let status = response.status();
        let response_text = response
            .text()
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed with status {}: {}", status, response_text),
        )))
    }
}
