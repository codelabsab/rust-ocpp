use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{datatypes::CustomDataType, enumerations::CostDimensionEnumType};

/// Volume consumed of cost dimension.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostDimensionType {
    /// Type of cost dimension: energy, power, time, etc.
    pub r#type: CostDimensionEnumType,

    /// Volume of the dimension consumed, measured according to the dimension type.
    pub volume: f64,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
