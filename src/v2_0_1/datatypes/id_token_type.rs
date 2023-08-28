use super::additional_info_type::AdditionalInfoType;
use crate::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;
use crate::Vec;

/// Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
/// IdTokenType is used by: Common:AuthorizationData , Common:IdTokenInfoType , RequestStartTransactionRequest, AuthorizeRequest , TransactionEventRequest , ReserveNowRequest , CustomerInformationRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType<'a, const N_ADDITIONAL_INFOS: usize = { crate::ID_TOKEN_TYPE_N_ADDITIONAL_INFOS }> {
    pub id_token: &'a str,
    #[serde(rename = "type")]
    pub kind: IdTokenEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType<'a>, N_ADDITIONAL_INFOS>>,
}
