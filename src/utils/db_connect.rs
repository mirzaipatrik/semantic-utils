pub mod db {
    use dotenv::dotenv;
    use reqwest::blocking::Client;
    use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
    use serde_json::json;
    use std::env;

    pub fn db_get_results() {
        let query = "hello";

        dotenv().ok();

        let pinecone_api_key = env::var("PINECONE_API_KEY").expect("PINECONE_API_KEY must be set");
        let api_endpoint = env::var("PINECONE_ENDPOINT").expect("PINECONE_ENDPOINT must be set");

        let client: Client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", pinecone_api_key).parse().unwrap(),
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let data = json!({
            "inputs": &query
        });
    }
}
