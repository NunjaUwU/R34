use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum TagType {
    General(String),
    Artist(String),
    Character(String),
    Copyright(String),
}

impl Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tagtype = match self {
            Self::General(_a) => "general".to_string(),
            Self::Artist(_a) => "artist".to_string(),
            Self::Character(_a) => "character".to_string(),
            Self::Copyright(_a) => "copyright".to_string(),
        };

        write!(f, "{tagtype}")
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub value: String,
    pub label: String,
    pub tag_type: TagType,
}

impl SearchResult {
    pub fn parse(response: &str) -> Vec<Self> {
        let val: serde_json::Value = serde_json::from_str(response).unwrap();
        let vals = val.as_array().unwrap().to_owned();

        let mut search_results: Vec<SearchResult> = Vec::new();

        for mut ob in vals {
            let label = ob["label"].take().to_string().replace('"', "");
            let tag_type = match ob["type"].take().as_str().unwrap() {
                "general" => TagType::General(label.clone()),
                "artist" => TagType::Artist(label.clone()),
                "character" => TagType::Character(label.clone()),
                "copyright" => TagType::Copyright(label.clone()),
                _ => TagType::General(label.clone()),
            };

            let searchresult = Self {
                label,
                value: ob["value"].take().to_string().replace('"', ""),
                tag_type,
            };

            search_results.push(searchresult);
        }

        search_results
    }
}

pub async fn ac_search(input: String) -> String {
    let base_req = String::from("https://ac.rule34.xxx/autocomplete.php?q=");
    let res = match reqwest::get(&format!("{}{}", base_req, input)).await {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    };
    match res.text().await {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}
