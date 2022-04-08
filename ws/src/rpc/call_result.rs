#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// CallResult: [<MessageTypeId>, "<MessageId>", {<Payload>}]
pub struct CallResult {
    pub message_type_id: usize,
    pub message_id: String,
    pub payload: serde_json::Value,
}

impl CallResult {
    pub fn new(message_type_id: usize, message_id: String, payload: serde_json::Value) -> Self {
        Self {
            message_type_id,
            message_id,
            payload,
        }
    }
}
