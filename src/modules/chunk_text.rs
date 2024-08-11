pub mod chunking {
    use crate::structs::common::structs::StoriesResponse;
    use std::error::Error;
    pub fn chunk_text(strings: Result<StoriesResponse, Box<dyn Error>>) {
        println!("{:#?}", strings)
    }
    
}
