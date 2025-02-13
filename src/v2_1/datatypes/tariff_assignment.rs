use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Tariff assignment to a charging profile.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffAssignmentType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unique identifier used to identify one or more tariffs.
    #[validate(length(max = 36))]
    pub id: String,

    /// Required. Start date and time of the tariff assignment.
    pub start: String,

    /// Optional. End date and time of the tariff assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
}
