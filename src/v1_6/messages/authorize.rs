use crate::v1_6::types::IdTagInfo;
use validator::Validate;

// # From OCPP Specification
// 6.1. Authorize.req
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest<'a> {
    /// Required. This contains the identifier that needs to be authorized.
    #[cfg_attr(feature="std", validate(length(min = 1, max = 20)))]
    pub id_tag: &'a str, // IdToken, should maybe be a type?
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse<'a> {
    /// This contains the field definition of the Authorize.conf PDU sent by the Central System to the Charge Point in response to a Authorize.req PDU. See also Authorize
    #[serde(borrow)]
    pub id_tag_info: IdTagInfo<'a>,
}
