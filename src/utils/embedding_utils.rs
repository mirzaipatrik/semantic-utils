pub mod embedding {
    use dotenv::dotenv;
    use reqwest::blocking::Client;
    use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
    use serde_json::json;
    use std::env;

    pub fn get_embedding(query: &str) {
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
            "inputs": &query
        });

        let http_result = http_client
            .post(embeddings_endpoint)
            .headers(headers)
            .body(data.to_string())
            .send();

        if http_result.is_ok() {
            println!("{:#?}", http_result.ok().unwrap().text())
        } else {
            println!("not working")
        }
    }
}
