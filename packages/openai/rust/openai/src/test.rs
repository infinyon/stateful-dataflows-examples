use std::fs::read_to_string;

use serde_json::Value;

const EXAMPLE_REQUEST: &str = r#"{
    "model": "gpt-3.5-turbo",
    "messages": [
        {
            "role": "user",
            "content": "tell me a story"
        }
    ]
}"#;


/// test good serialization 
#[test]
fn serialize_good() {
    
    use crate::bindings::example::openai_types::types::{OpenaiRequest,Message};

    
    let req = OpenaiRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
          Message {
            role: "user".to_string(),
            content: "tell me a story".to_string()
          }
        ]
      };

    let generated: Value = serde_json::to_value(&req).unwrap();

    // this should be the same as the original
    let original: Value = serde_json::from_str(EXAMPLE_REQUEST).unwrap();
    let diff = sjdiff::DiffBuilder::default()
        .source(generated)
        .target(original)
        .build()
        .unwrap();
    let diff = diff.compare();
    assert!(diff.is_none());


}

/// test bad serialization and print diff
#[test]
fn sieralize_bad() {
    
    use crate::bindings::example::openai_types::types::{OpenaiRequest,Message};

    
    let req = OpenaiRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
          Message {
            role: "user2".to_string(),
            content: "tell me a story".to_string()
          }
        ]
      };

    let generated: Value = serde_json::to_value(&req).unwrap();

    // this should be the same as the original
    let original: Value = serde_json::from_str(EXAMPLE_REQUEST).unwrap();
    let diff = sjdiff::DiffBuilder::default()
        .source(generated)
        .target(original)
        .build()
        .unwrap();
    let diff = diff.compare();
    assert!(diff.is_some());

    // print diff
    serde_json::to_writer_pretty(std::io::stdout(), &diff).unwrap();

}




/// test good deserialization
#[test]
fn deserialize_good() {
    
    use crate::bindings::example::openai_types::types::{OpenaiResponse};

    let original_json = read_to_string("../../response.json").unwrap();
    let resp: OpenaiResponse = serde_json::from_str(&original_json).unwrap();

    assert_eq!(resp.id,"12");
    assert_eq!(resp.model,"gpt-4o-2024-08-06");
    assert_eq!(resp.choices.len(),1);

}