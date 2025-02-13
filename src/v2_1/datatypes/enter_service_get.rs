use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, enter_service::EnterServiceType};

/// Type for getting EnterService DER control function parameters.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnterServiceGetType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The EnterService parameters.
    pub enter_service: EnterServiceType,

    /// Id of setting.
    #[validate(length(max = 36))]
    pub id: String,
}
