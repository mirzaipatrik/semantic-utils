use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value}; // Import serde_json and Value.
use std::env;
use std::error::Error;

use serde::{Deserialize, Serialize};

// Represents the `storyContent` field in a Story.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryContent {
    #[serde(rename = "__typename")]
    typename: String,
    #[serde(rename = "paragraphText")]
    paragraph_text: Option<String>,
}

// Represents a story item.
#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    date: String,
    #[serde(rename = "storyNumber")]
    story_number: u32, // Updated to snake_case
    description: String,
    #[serde(rename = "storyContent")]
    story_content: Vec<StoryContent>, // Updated to snake_case
}

// Represents the `allStories` field in the GraphQL response.
#[derive(Debug, Serialize, Deserialize)]
pub struct AllStories {
    #[serde(rename = "allStories")]
    all_stories: Vec<Story>, // Updated to snake_case
}

// Represents the top-level response object containing `data`.
#[derive(Debug, Serialize, Deserialize)]
pub struct StoriesResponse {
    data: AllStories,
}

pub fn get_stories(query: &str, variables: Value) -> Result<StoriesResponse, Box<dyn Error>> {
    // Return a Value instead of String.
    dotenv().ok();

    let dato_cms_token = env::var("DATOCMS_API_TOKEN")?;
    let embeddings_endpoint = "https://graphql.datocms.com/";
    let http_client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", dato_cms_token).parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(ACCEPT, "application/json".parse()?);

    let query_body = json!({
        "query": query,
        "variables": variables

    });

    let response = http_client
        .post(embeddings_endpoint)
        .headers(headers)
        .body(query_body.to_string())
        .send()?;

    let response_text: String = response.text()?; // Get the response as a string.

    // Deserialize the string into a serde_json::Value
    let json_response: StoriesResponse = serde_json::from_str(&response_text)?;
    Ok(json_response) // Return the JSON object.
}
