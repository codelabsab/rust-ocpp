use crate::v1_6::types::CancelReservationStatus;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationRequest {
    /// Required. Id of the reservation to cancel.
    pub reservation_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationResponse {
    /// Required. This indicates the success or failure of the cancelling of a reservation by Central System.
    pub status: CancelReservationStatus,
}
