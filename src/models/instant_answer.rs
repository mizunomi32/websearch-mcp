use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstantAnswerResponse {
    #[serde(rename = "Abstract")]
    pub abstract_text: String,
    #[serde(rename = "AbstractSource")]
    pub abstract_source: String,
    #[serde(rename = "AbstractURL")]
    pub abstract_url: String,
    #[serde(rename = "Answer")]
    pub answer: String,
    #[serde(rename = "Definition")]
    pub definition: String,
    #[serde(rename = "DefinitionSource")]
    pub definition_source: String,
    #[serde(rename = "DefinitionURL")]
    pub definition_url: String,
    #[serde(rename = "RelatedTopics")]
    pub related_topics: Vec<RelatedTopic>,
    #[serde(rename = "Type")]
    pub response_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RelatedTopic {
    Topic(ResultItem),
    Category {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "Topics")]
        topics: Vec<ResultItem>,
    },
}

#[derive(Debug, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "FirstURL")]
    pub first_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_normal_response() {
        let json = r#"{
            "Abstract": "Rust is a programming language",
            "AbstractSource": "Wikipedia",
            "AbstractURL": "https://en.wikipedia.org/wiki/Rust",
            "Answer": "",
            "Definition": "",
            "DefinitionSource": "",
            "DefinitionURL": "",
            "RelatedTopics": [
                {"Text": "Cargo - package manager", "FirstURL": "https://example.com/cargo"}
            ],
            "Type": "A"
        }"#;
        let resp: InstantAnswerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.abstract_text, "Rust is a programming language");
        assert_eq!(resp.abstract_source, "Wikipedia");
        assert_eq!(resp.response_type, "A");
        assert_eq!(resp.related_topics.len(), 1);
    }

    #[test]
    fn test_deserialize_empty_response() {
        let json = r#"{
            "Abstract": "",
            "AbstractSource": "",
            "AbstractURL": "",
            "Answer": "",
            "Definition": "",
            "DefinitionSource": "",
            "DefinitionURL": "",
            "RelatedTopics": [],
            "Type": ""
        }"#;
        let resp: InstantAnswerResponse = serde_json::from_str(json).unwrap();
        assert!(resp.abstract_text.is_empty());
        assert!(resp.related_topics.is_empty());
    }

    #[test]
    fn test_deserialize_category_related_topic() {
        let json = r#"{
            "Abstract": "",
            "AbstractSource": "",
            "AbstractURL": "",
            "Answer": "",
            "Definition": "",
            "DefinitionSource": "",
            "DefinitionURL": "",
            "RelatedTopics": [
                {"Text": "Topic 1", "FirstURL": "https://example.com/1"},
                {"Name": "Category", "Topics": [
                    {"Text": "Sub topic", "FirstURL": "https://example.com/sub"}
                ]}
            ],
            "Type": "D"
        }"#;
        let resp: InstantAnswerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.related_topics.len(), 2);
    }
}
