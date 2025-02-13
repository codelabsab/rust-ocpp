use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use super::modem::ModemType;

/// The physical system where an Electrical Vehicle (EV) can be charged.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Vendor-specific device identifier.
    #[validate(length(max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// Required. Defines the model of the device.
    #[validate(length(max = 20))]
    pub model: String,

    /// Required. Identifies the vendor (not necessarily in a unique manner).
    #[validate(length(max = 50))]
    pub vendor_name: String,

    /// This contains the firmware version of the Charging Station.
    #[validate(length(max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,

    /// Defines parameters required for initiating and maintaining wireless communication with other devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>,
}
