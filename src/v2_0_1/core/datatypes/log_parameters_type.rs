use chrono::{DateTime, Utc};

/// Generic class for the configuration of logging entries.
/// LogParametersType is used by: GetLogRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    pub remote_location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<DateTime<Utc>>,
}
