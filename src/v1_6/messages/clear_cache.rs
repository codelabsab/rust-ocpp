use crate::v1_6::types::ClearCacheStatus;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ClearCacheRequest {
    // This contains the field definition of the ClearCache.req PDU sent by the Central System to the Charge Point. See also Clear Cache. No fields are defined.
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
/// This contains the field definition of the ClearCache.conf PDU sent by the Charge Point to the Central System in response to a ClearCache.req PDU. See also Clear Cache
pub struct ClearCacheResponse {
    pub status: ClearCacheStatus,
}
