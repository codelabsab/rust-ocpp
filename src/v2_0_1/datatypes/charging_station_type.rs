use super::modem_type::ModemType;
use validator::Validate;
/// The physical system where an Electrical Vehicle (EV) can be charged.
// ChargingStationType is used by: BootNotificationRequest
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    /// Optional. Vendor-specific device identifier.
    #[validate(length(min = 0, max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// Required. Defines the model of the device
    #[validate(length(min = 0, max = 20))]
    pub model: String,
    /// Required. Identifies the vendor (not necessarily in a unique manner).
    #[validate(length(min = 0, max = 50))]
    pub vendor_name: String,
    #[validate(length(min = 0, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional. This contains the firmware version of the Charging Station
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional. Defines the functional parameters of a communication link
    pub modem: Option<ModemType>,
}
