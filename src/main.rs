mod utils; // Declare the `utils` module
mod modules;
mod structs;
use serde_json::json;
use structs::common::structs::StoriesResponse;
use std::error::Error;
use utils::db_connect::db::upsert_data;
use utils::db_connect::db::DbProperties;
use utils::embedding_utils::get_embedding;
use utils::get_datocms_stories::get_stories;
use modules::chunk_text::chunking::chunk_text;

const QUERY: &str = r#"
query Home($language: SiteLocale, $skip: IntType, $yearStart: Date, $yearEnd: Date) {
  allStories(locale: $language, first: 20, skip: $skip, filter: {date: {gt: $yearStart, lt: $yearEnd}}, orderBy: date_DESC) {
    storyNumber
    date
    description
    storyContent {
      __typename
      ... on ParagraphRecord {
        paragraphText
      }
    }
  }
}
"#;

fn main() {
    let embedding: Result<Vec<f64>, reqwest::Error> = get_embedding("hello");
    println!("{:#?}", embedding);

    let props = DbProperties {
        embedding: embedding,
    };

    let response = upsert_data(props);
    println!("{:#?}", response);

    let variables = json!({
        "language": "en",
        "skip": 0,
        "yearStart": "2018-01-01",
        "yearEnd": "2022-01-01"
    });

    let datocms: Result<StoriesResponse, Box<dyn Error>> = get_stories(QUERY, variables);
    println!("{:#?}", datocms);

    chunk_text();
}
