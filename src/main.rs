mod utils; // Declare the `utils` module

use utils::embedding_utils::get_embedding;
use utils::db_connect::db::upsert_data;
use utils::db_connect::db::DbProperties;

fn main() {
    let embedding: Result<Vec<f64>, reqwest::Error> = get_embedding("hello");
    println!("{:#?}", embedding);

    let props = DbProperties {
        embedding: embedding,
    };

    let response = upsert_data(props);
    println!("{:#?}", response);
}
