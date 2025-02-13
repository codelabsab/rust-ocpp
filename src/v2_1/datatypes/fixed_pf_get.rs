use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, fixed_pf::FixedPFType};

/// Fixed power factor get type for retrieving fixed power factor settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FixedPFGetType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The fixed power factor settings.
    pub fixed_pf: FixedPFType,

    /// Id of the setting.
    #[validate(length(max = 36))]
    pub id: String,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,
}
