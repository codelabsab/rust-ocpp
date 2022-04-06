#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallError {
    pub message_type_id: i64,
    pub message_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: String,
}

impl CallError {
    pub fn new(
        message_type_id: i64,
        message_id: String,
        error_code: String,
        error_description: String,
        error_details: String,
    ) -> Self {
        Self {
            message_type_id,
            message_id,
            error_code,
            error_description,
            error_details,
        }
    }
}
