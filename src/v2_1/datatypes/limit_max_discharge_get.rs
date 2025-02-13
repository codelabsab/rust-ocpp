use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, limit_max_discharge::LimitMaxDischargeType};

/// Limit max discharge get type for retrieving limit max discharge settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LimitMaxDischargeGetType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The limit max discharge settings.
    pub limit_max_discharge: LimitMaxDischargeType,

    /// Id of the setting.
    #[validate(length(max = 36))]
    pub id: String,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,
}
