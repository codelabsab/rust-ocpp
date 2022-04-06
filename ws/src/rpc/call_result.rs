#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallResult {
    pub message_type_id: i64,
    pub message_id: String,
    pub payload: String,
}

impl CallResult {
    pub fn new(message_type_id: i64, message_id: String, payload: String) -> Self {
        Self {
            message_type_id,
            message_id,
            payload,
        }
    }
}
