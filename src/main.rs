mod utils; // Declare the `utils` module

use std::error::Error;

use utils::embedding_utils::get_embedding;
use utils::db_connect::db::upsert_data;
use utils::db_connect::db::DbProperties;
use utils::get_datocms_stories::get_stories;

const QUERY: &str = r#"
query {
  allStories(
    locale: en
    filter: {date: {gt: "2018-01-01", lt: "2022-01-01"}}
    orderBy: date_DESC
  ) {
    storyNumber
    date
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

    let datocms: Result<String, Box<dyn Error>> = get_stories(QUERY);
    println!("{:#?}", datocms);

}
