use crate::models::instant_answer::{InstantAnswerResponse, RelatedTopic};

pub fn format_instant_answer(query: &str, response: &InstantAnswerResponse) -> String {
    let mut output = format!("## Instant Answer for \"{query}\"\n\n");

    let has_abstract = !response.abstract_text.is_empty();
    let has_related = !response.related_topics.is_empty();

    if !has_abstract && !has_related {
        output.push_str("No instant answer available for this query.\n\n");
        output.push_str("_Source: DuckDuckGo Instant Answer API_");
        return output;
    }

    if has_abstract {
        output.push_str("### Abstract\n");
        output.push_str(&response.abstract_text);
        output.push_str("\n\n");
        output.push_str(&format!("**Source:** {}\n", response.abstract_source));
        output.push_str(&format!("**URL:** {}\n\n", response.abstract_url));
    }

    if has_related {
        output.push_str("### Related Topics\n");
        for topic in &response.related_topics {
            match topic {
                RelatedTopic::Topic(item) => {
                    output.push_str(&format!("- **{}**\n", item.text));
                }
                RelatedTopic::Category { name, topics } => {
                    output.push_str(&format!("\n**{}**\n", name));
                    for item in topics {
                        output.push_str(&format!("- **{}**\n", item.text));
                    }
                }
            }
        }
        output.push('\n');
    }

    output.push_str("_Source: DuckDuckGo Instant Answer API_");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::instant_answer::ResultItem;

    #[test]
    fn test_format_normal() {
        let json = include_str!("../../tests/fixtures/instant_answer.json");
        let response: InstantAnswerResponse = serde_json::from_str(json).unwrap();
        let output = format_instant_answer("Rust programming language", &response);
        assert!(output.contains("## Instant Answer for \"Rust programming language\""));
        assert!(output.contains("### Abstract"));
        assert!(output.contains("performance, type safety, and concurrency"));
        assert!(output.contains("**Source:** Wikipedia"));
        assert!(output.contains("**URL:** https://en.wikipedia.org/wiki/Rust_(programming_language)"));
        assert!(output.contains("### Related Topics"));
        assert!(output.contains("Cargo"));
        assert!(output.contains("_Source: DuckDuckGo Instant Answer API_"));
    }

    #[test]
    fn test_format_empty() {
        let json = include_str!("../../tests/fixtures/instant_answer_empty.json");
        let response: InstantAnswerResponse = serde_json::from_str(json).unwrap();
        let output = format_instant_answer("xyzzy12345noresult", &response);
        assert!(output.contains("## Instant Answer for \"xyzzy12345noresult\""));
        assert!(output.contains("No instant answer available for this query."));
        assert!(output.contains("_Source: DuckDuckGo Instant Answer API_"));
        assert!(!output.contains("### Abstract"));
    }

    #[test]
    fn test_format_disambig() {
        let json = include_str!("../../tests/fixtures/instant_answer_disambig.json");
        let response: InstantAnswerResponse = serde_json::from_str(json).unwrap();
        let output = format_instant_answer("java", &response);
        assert!(output.contains("### Related Topics"));
        assert!(output.contains("Java (programming language)"));
        assert!(output.contains("Programming"));
        assert!(output.contains("_Source: DuckDuckGo Instant Answer API_"));
    }

    #[test]
    fn test_format_abstract_only_no_related() {
        let response = InstantAnswerResponse {
            abstract_text: "Some abstract text".to_string(),
            abstract_source: "TestSource".to_string(),
            abstract_url: "https://example.com".to_string(),
            answer: String::new(),
            definition: String::new(),
            definition_source: String::new(),
            definition_url: String::new(),
            related_topics: vec![],
            response_type: "A".to_string(),
        };
        let output = format_instant_answer("test", &response);
        assert!(output.contains("### Abstract"));
        assert!(output.contains("Some abstract text"));
        assert!(!output.contains("### Related Topics"));
        assert!(output.contains("_Source: DuckDuckGo Instant Answer API_"));
    }

    #[test]
    fn test_format_related_topic_items() {
        let response = InstantAnswerResponse {
            abstract_text: String::new(),
            abstract_source: String::new(),
            abstract_url: String::new(),
            answer: String::new(),
            definition: String::new(),
            definition_source: String::new(),
            definition_url: String::new(),
            related_topics: vec![
                RelatedTopic::Topic(ResultItem {
                    text: "Topic One - Description one".to_string(),
                    first_url: "https://example.com/1".to_string(),
                }),
                RelatedTopic::Topic(ResultItem {
                    text: "Topic Two - Description two".to_string(),
                    first_url: "https://example.com/2".to_string(),
                }),
            ],
            response_type: "A".to_string(),
        };
        let output = format_instant_answer("test", &response);
        assert!(output.contains("### Related Topics"));
        assert!(output.contains("- **Topic One - Description one**"));
        assert!(output.contains("- **Topic Two - Description two**"));
    }

    #[test]
    fn test_format_category_topics() {
        let response = InstantAnswerResponse {
            abstract_text: String::new(),
            abstract_source: String::new(),
            abstract_url: String::new(),
            answer: String::new(),
            definition: String::new(),
            definition_source: String::new(),
            definition_url: String::new(),
            related_topics: vec![
                RelatedTopic::Category {
                    name: "Science".to_string(),
                    topics: vec![
                        ResultItem {
                            text: "Physics - Study of matter".to_string(),
                            first_url: "https://example.com/physics".to_string(),
                        },
                    ],
                },
            ],
            response_type: "D".to_string(),
        };
        let output = format_instant_answer("test", &response);
        assert!(output.contains("### Related Topics"));
        assert!(output.contains("**Science**"));
        assert!(output.contains("- **Physics - Study of matter**"));
    }
}
