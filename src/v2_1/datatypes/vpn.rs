use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// VPN Configuration settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VPNType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. VPN Server Address.
    #[validate(length(max = 512))]
    pub server: String,

    /// Required. VPN User.
    #[validate(length(max = 20))]
    pub user: String,

    /// Required. VPN Password.
    #[validate(length(max = 20))]
    pub password: String,

    /// Required. VPN Key.
    #[validate(length(max = 255))]
    pub key: String,

    /// Required. VPN Type.
    #[validate(length(max = 32))]
    pub r#type: String,
}
