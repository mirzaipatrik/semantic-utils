pub mod structs {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StoryContent {
        #[serde(rename = "__typename")]
        pub typename: String,
        pub paragraph_text: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Story {
        pub date: String,
        pub story_number: u32, 
        pub description: String,
        pub story_content: Vec<StoryContent>, 
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AllStories {
        pub all_stories: Vec<Story>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StoriesResponse {
        pub data: AllStories,
    }
}
