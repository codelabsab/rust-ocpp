use crate::v2_0_1::core::{
    datatypes::status_info_type::StatusInfoType,
    enumerations::clear_cache_status_enum_type::ClearCacheStatusEnumType,
};

/// This contains the field definition of the ClearCacheRequest PDU sent by the CSMS to the Charging Station. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {}

/// This contains the field definition of the ClearCacheResponse PDU sent by the Charging Station to the CSMS in response to a ClearCacheRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheResponse {
    pub status: ClearCacheStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
