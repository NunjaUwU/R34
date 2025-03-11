use std::fmt::Display;

pub enum TagType {
    General,
    Artist,
    Character,
    Copyright,
}

impl Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tagtype = match self {
            Self::General => "general".to_string(),
            Self::Artist => "artist".to_string(),
            Self::Character => "character".to_string(),
            Self::Copyright => "copyright".to_string(),
        };

        write!(f, "{tagtype}")
    }
}

#[allow(unused)]
pub struct SearchResult {
    value: String,
    label: String,
    tag_type: TagType,
}

impl SearchResult {
    pub fn parse(response: &str) -> Vec<Self> {
        let val: serde_json::Value = serde_json::from_str(response).unwrap();
        let vals = val.as_array().unwrap().to_owned();

        let mut search_results: Vec<SearchResult> = Vec::new();

        for mut ob in vals {
            let tag_type = match ob["type"].take().as_str().unwrap() {
                "general" => TagType::General,
                "artist" => TagType::Artist,
                "character" => TagType::Character,
                "copyright" => TagType::Copyright,
                _ => TagType::General,
            };

            let searchresult = Self {
                label: ob["label"].take().to_string(),
                value: ob["value"].take().to_string(),
                tag_type,
            };

            search_results.push(searchresult);
        }

        search_results
    }
}
