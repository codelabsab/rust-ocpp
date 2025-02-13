use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;

use super::{additional_info::AdditionalInfoType, custom_data::CustomDataType};

/// Contains identifier to use for authorization.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Type of the identifier.
    pub r#type: IdTokenEnumType,

    /// Required. The identifier to use for authorization.
    #[validate(length(max = 36))]
    pub id_token: String,

    /// Optional. Additional information about the identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType>>,
}
