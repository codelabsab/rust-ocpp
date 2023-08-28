use chrono::{DateTime, Utc};
#[cfg(feature = "std")]
use validator::Validate;

/// This contains the field definition of the GetDiagnostics.req PDU sent by the Central System to the Charge Point. See also Get Diagnostics
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsRequest<'a> {
    /// Required. This contains the location (directory) where the diagnostics file shall be uploaded to.
    pub location: &'a str,
    /// Optional. This specifies how many times Charge Point must try to upload the diagnostics before giving up. If this field is not present, it is left to Charge Point to decide how many times it wants to retry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    /// Optional. The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charge Point to decide how long to wait between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
    /// Optional. This contains the date and time of the oldest logging information to include in the diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional. This contains the date and time of the latest logging information to include in the diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<DateTime<Utc>>,
}

/// This contains the field definition of the GetDiagnostics.conf PDU sent by the Charge Point to the Central System in response to a GetDiagnosticsRequest PDU. See also Get Diagnostics
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsResponse<'a> {
    /// Optional. This contains the name of the file with diagnostic information that will be uploaded. This field is not present when no diagnostic information is vailable.
    #[cfg_attr(feature="std", validate(length(min = 1, max = 255)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<&'a str>,
}
