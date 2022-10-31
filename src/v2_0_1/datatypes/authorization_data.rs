//! Contains the identifier to use for authorization
use super::{id_token_info_type::IdTokenInfoType, id_token_type::IdTokenType};

/// Contains the identifier to use for authorization
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    /// Required when UpdateType is Full. This contains information about
    /// authorization status, expiry and group id. For a Differential update
    ///  the following applies: If this element is present, then this entry
    /// SHALL be added or updated in the Local Authorization List. If this
    /// element is absent, the entry for this IdToken in the Local
    /// Authorization List SHALL be deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,
    /// This contains the identifier which needs to be stored for authorization.
    pub id_token: IdTokenType,
}
