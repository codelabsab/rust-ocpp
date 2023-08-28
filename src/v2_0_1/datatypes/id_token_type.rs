use super::additional_info_type::AdditionalInfoType;
use crate::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;

/// Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
/// IdTokenType is used by: Common:AuthorizationData , Common:IdTokenInfoType , RequestStartTransactionRequest, AuthorizeRequest , TransactionEventRequest , ReserveNowRequest , CustomerInformationRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType<'a> {
    pub id_token: &'a str,
    #[serde(rename = "type")]
    pub kind: IdTokenEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType<'a>>>,
}
