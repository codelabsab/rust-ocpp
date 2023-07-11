use crate::types::IdTagInfo;
use validator::Validate;

// # From OCPP Specification
// 6.1. Authorize.req
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    /// Required. This contains the identifier that needs to be authorized.
    #[validate(length(min = 1, max = 20))]
    pub id_tag: String, // IdToken, should maybe be a type?
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    /// This contains the field definition of the Authorize.conf PDU sent by the Central System to the Charge Point in response to a Authorize.req PDU. See also Authorize
    pub id_tag_info: IdTagInfo,
}
