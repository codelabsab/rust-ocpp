use super::modem_type::ModemType;
use validator::Validate;
/// The physical system where an Electrical Vehicle (EV) can be charged.
// ChargingStationType is used by: BootNotificationRequest
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType<'a> {
    /// Optional. Vendor-specific device identifier.
    #[validate(length(min = 0, max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<&'a str>,
    /// Required. Defines the model of the device
    #[validate(length(min = 0, max = 20))]
    pub model: &'a str,
    /// Required. Identifies the vendor (not necessarily in a unique manner).
    #[validate(length(min = 0, max = 50))]
    pub vendor_name: &'a str,
    #[validate(length(min = 0, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional. This contains the firmware version of the Charging Station
    pub firmware_version: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional. Defines the functional parameters of a communication link
    pub modem: Option<ModemType<'a>>,
}
