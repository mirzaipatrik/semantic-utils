pub mod structs {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct StoryContent {
        #[serde(rename = "__typename")]
        pub typename: String,
        pub paragraph_text: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Story {
        pub date: String,
        pub story_number: u32,
        pub description: String,
        pub story_content: Vec<StoryContent>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AllStories {
        pub all_stories: Vec<Story>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct StoriesResponse {
        pub data: AllStories,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Metadata {
        pub title: String,
        pub date: String,
        pub chunked_text: String,
        pub story_number: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct OutputObject {
        pub id: String,
        pub metadata: Metadata,
        pub values: Vec<f32>
    }
}
