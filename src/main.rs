mod utils; // Declare the `utils` module

use utils::embedding_utils::embedding::get_embedding;
use utils::db_connect::db::db_get_results;

fn main() {
    get_embedding("hello"); // Call the function
    db_get_results();
}
