pub mod structs {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StoryContent {
        #[serde(rename = "__typename")]
        typename: String,
        paragraph_text: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Story {
        date: String,
        story_number: u32, 
        description: String,
        story_content: Vec<StoryContent>, 
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AllStories {
        all_stories: Vec<Story>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StoriesResponse {
        data: AllStories,
    }
}
