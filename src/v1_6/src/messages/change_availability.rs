use crate::types::{AvailabilityStatus, AvailabilityType};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    /// Required. The id of the connector for which availability needs to change. Id '0' (zero) is used if the availability of the Charge Point and all its connectors needs to change.
    pub connector_id: u64,
    /// Required. This contains the type of availability change that the Charge Point should perform.
    #[serde(rename = "type")]
    pub kind: AvailabilityType,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ChangeAvailabilityResponse {
    /// Required. This indicates whether the Charge Point is able to perform the availability change.
    pub status: AvailabilityStatus,
}
