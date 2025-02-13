use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, id_token::IdTokenType, id_token_info::IdTokenInfoType};

/// Contains the identifier to use for authorization.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The identifier to be authorized.
    pub id_token: IdTokenType,

    /// Required. Status information about the identifier.
    pub id_token_info: IdTokenInfoType,
}
