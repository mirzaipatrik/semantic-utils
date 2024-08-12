pub mod chunking {

    use crate::structs::common::structs::{StoriesResponse, Story};
    use std::error::Error;
    use std::fs::File;
    use std::io::Write;
    pub fn chunk_text(strings: Result<StoriesResponse, Box<dyn Error>>) {
        match strings {
            Ok(data) => {

                let pretty_json = serde_json::to_string_pretty(&data).unwrap();
                let mut file = File::create("output.json").expect("Unable to create file");
                file.write_all(pretty_json.as_bytes()).expect("Unable to write data");
                println!("Output written to output.json");

                let stories: Vec<Story> = data.data.all_stories;
                for story in stories {
                    // println!("{:?}", story.story_content);
                    println!("\n\n");
                    for content in &story.story_content {
                        if content.typename == "ParagraphRecord" {
                            println!("hello");
                        }
                    }
                }
                // println!("{:#?}", data.data.all_stories)
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
