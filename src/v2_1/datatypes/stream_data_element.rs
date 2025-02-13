use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, variable::VariableType};

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StreamDataElementType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Variable for which the stream data is reported.
    pub variable: VariableType,

    /// Required. The value for the variable.
    #[validate(length(max = 2500))]
    pub value: String,

    /// Required. Sequence number for stream data.
    #[validate(range(min = 0))]
    pub sequence_id: i32,
}
