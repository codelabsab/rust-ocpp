use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, der_curve::DERCurveType};
use crate::v2_1::enumerations::der_control::DERControlEnumType;

/// DER curve get type for retrieving DER curve information.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERCurveGetType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The DER curve.
    pub curve: DERCurveType,

    /// Id of DER curve.
    #[validate(length(max = 36))]
    pub id: String,

    /// Type of DER curve.
    pub curve_type: DERControlEnumType,

    /// True if this is a default curve.
    pub is_default: bool,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,
}
