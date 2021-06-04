use super::modem_type::ModemType;

/// The physical system where an Electrical Vehicle (EV) can be charged.
// ChargingStationType is used by: BootNotificationRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    pub model: String,
    pub vendor_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>,
}
