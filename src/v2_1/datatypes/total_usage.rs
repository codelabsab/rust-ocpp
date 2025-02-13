use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Structure to report total usage of a resource.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TotalUsageType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The total usage of the resource.
    pub usage: f64,

    /// Optional. The total usage of the resource, excluding taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_excl_tax: Option<f64>,

    /// Optional. The total usage of the resource, including taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_incl_tax: Option<f64>,
}
