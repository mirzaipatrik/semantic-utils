pub mod chunking {
    use crate::structs::common::structs::{Metadata, OutputObject, StoriesResponse, Story};
    use crate::utils::db_connect::db::upsert_data;
    use crate::utils::embedding_utils::get_embedding;
    use regex::Regex;

    pub fn chunk_text(stories: StoriesResponse) -> Vec<OutputObject> {
        let output_objects: Vec<OutputObject> = Vec::new();
    
        let stories: Vec<Story> = stories.data.all_stories;

        let re = Regex::new(r"[ ]*?\n[ ]*?\n[ ]*").unwrap();

        for story in stories {
            
            // Upsert title
            match get_embedding(&story.description) {
                Ok(embedding) => {
                    let title_object = OutputObject {
                        id: format!("{}-{}", story.story_number, 999),
                        metadata: Metadata {
                            title: story.description.clone(),
                            date: story.date.clone(),
                            chunked_text: story.description.clone(),
                            story_number: story.story_number.to_string(),
                        },
                        values: embedding,
                    };
                    upsert_data(title_object);
                }
                Err(e) => {
                    eprintln!("Failed to get embedding for description '{}': {}", &story.description, e);
                    continue;
                }
            }

            for content in &story.story_content {
                if content.typename == "ParagraphRecord" {
                    if let Some(paragraph_text) = &content.paragraph_text {
                        let splitted_text: Vec<&str> = re.split(paragraph_text).collect();
    
                        for (i, part) in splitted_text.iter().enumerate() {
                            match get_embedding(part) {
                                Ok(embedding_values) => {
                                    let output_object = OutputObject {
                                        id: format!("{}-{}", story.story_number, i),
                                        metadata: Metadata {
                                            title: story.description.clone(),
                                            date: story.date.clone(),
                                            chunked_text: part.to_string(),
                                            story_number: story.story_number.to_string(),
                                        },
                                        values: embedding_values,
                                    };

                                    println!("{:#?}", output_object);
                                    upsert_data(output_object);
                                }
                                Err(e) => {
                                    eprintln!("Failed to get embedding for part '{}': {}", part, e);
                                    continue;
                                }
                            }
                        }
                    } else {
                        println!("No paragraph text available");
                    }
                }
            }
        }
    
        output_objects // Return the collected OutputObject instances
    }
}
