mod modules;
mod structs;
mod utils; // Declare the `utils` module
use modules::chunk_text::chunking::chunk_text;
use serde_json::json;
use std::error::Error;
use structs::common::structs::StoriesResponse;
use utils::get_datocms_stories::get_stories;

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
    let mut skip: i32 = 0;

    loop {
        let variables = json!({
            "language": "en",
            "skip": skip,
            "yearStart": "2018-01-01",
            "yearEnd": "2022-12-31"
        });

        let datocms_response: Result<StoriesResponse, Box<dyn Error>> =
            get_stories(QUERY, variables);

        match datocms_response {
            Ok(res) => {
                println!("Skip: {}", skip);
                println!("Data received length: {}", res.data.all_stories.len());

                let res_clone = res.clone();
                chunk_text(res_clone);

                if res.data.all_stories.len() < 20 {
                    break;
                } else {
                    skip += 20;
                }
            }

            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
