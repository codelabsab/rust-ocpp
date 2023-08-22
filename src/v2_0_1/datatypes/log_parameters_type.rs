use chrono::DateTime;
use chrono::Utc;

/// Generic class for the configuration of logging entries.
/// LogParametersType is used by: GetLogRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType<'a> {
    pub remote_location: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<DateTime<Utc>>,
}
