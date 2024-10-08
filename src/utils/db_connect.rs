pub mod db {
    use dotenv::dotenv;
    use reqwest::blocking::Client;
    use reqwest::header::HeaderMap;
    use serde_json::json;
    use std::env;

    use crate::structs::common::structs::OutputObject;

    pub fn upsert_data(props: OutputObject) {
        dotenv().ok();
        println!("{:#?}", props.values);


        let pinecone_api_key: String =
            env::var("PINECONE_API_KEY").expect("PINECONE_API_KEY must be set");
        let pinecone_host: String = env::var("PINECONE_HOST").expect("PINECONE_HOST must be set");

        let http_client = Client::new();

        let upsert_endpoint = format!("https://{}/vectors/upsert", pinecone_host);

        let mut headers = HeaderMap::new();
        headers.insert("Api-Key", format!("{}", pinecone_api_key).parse().unwrap());
        headers.insert("content-type", format!("application/json").parse().unwrap());
        headers.insert(
            "X-Pinecone-API-Version",
            format!("2024-07").parse().unwrap(),
        );

        let data = json!({
            "vectors": [
                {
                    "id": props.id,
                    "values": props.values,
                    "metadata": props.metadata
                }
            ]
        });

        let http_result = http_client
            .post(upsert_endpoint)
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
