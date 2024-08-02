use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let hf_token = env::var("HF_TOKEN").expect("HF_TOKEN must be set");
    let embeddings_endpoint = env::var("EMBEDDINGS_ENDPOINT").expect("EMBEDDINGS_ENDPOINT must be set");
    // let hf_token = "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";
    let http_client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", hf_token).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, format!("application/json").parse().unwrap());
    let data = json!({
        "inputs": "hello"
        }
    );

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

// async function query(data) {
// 	const response = await fetch(
// 		"https://api-inference.huggingface.co/models/sentence-transformers/all-MiniLM-L6-v2",
// 		{
// 			headers: {
// 				Authorization: "Bearer hf_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
// 				"Content-Type": "application/json",
// 			},
// 			method: "POST",
// 			body: JSON.stringify(data),
// 		}
// 	);
// 	const result = await response.json();
// 	return result;
// }

// query({"inputs": {
// 	"source_sentence": "That is a happy person",
// 	"sentences": [
// 		"That is a happy dog",
// 		"That is a very happy person",
// 		"Today is a sunny day"
// 	]
// }}).then((response) => {
// 	console.log(JSON.stringify(response));
// });
