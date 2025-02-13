use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Defines parameters required for initiating and maintaining wireless communication with other devices.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the ICCID of the modem's SIM card.
    #[validate(length(max = 20))]
    pub iccid: String,

    /// Required. This contains the IMSI of the modem's SIM card.
    #[validate(length(max = 20))]
    pub imsi: String,
}
