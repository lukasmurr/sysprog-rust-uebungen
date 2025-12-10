use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub username: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

pub fn new_message(username: &str, content: &str) -> ChatMessage {
    ChatMessage {
        username: username.to_string(),
        content: content.to_string(),
        timestamp: Utc::now(),
    }
}

pub fn serialize_message(msg: &ChatMessage) -> serde_json::Result<String> {
    let mut json = serde_json::to_string(msg)?;
    json.push('\n');
    Ok(json)
}

pub fn deserialize_message(line: &str) -> serde_json::Result<ChatMessage> {
    serde_json::from_str(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let msg = new_message("Alice", "Hello World");
        let serialized = serialize_message(&msg).expect("Serialization failed");
        let deserialized = deserialize_message(&serialized).expect("Deserialization failed");

        assert_eq!(msg.username, deserialized.username);
        assert_eq!(msg.content, deserialized.content);
        assert_eq!(msg.timestamp, deserialized.timestamp);
    }
}
