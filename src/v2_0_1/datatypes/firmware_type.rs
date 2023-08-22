use chrono::DateTime;
use chrono::Utc;

/// Represents a copy of the firmware that can be loaded/updated on the Charging Station.
/// FirmwareType is used by: UpdateFirmwareRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareType<'a> {
    pub location: &'a str,
    pub retrieve_date_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_certificate: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<&'a str>,
}
