//! CancelReservation
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::cancel_reservation_status_enum_type::CancelReservationStatusEnumType;

/// `CancelReservationRequest`, sent by the CSMS to the Charging Station
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationRequest {
    /// Id of the reservation to cancel.
    pub reservation_id: i64,
}

/// `CancelReservationResponse`, sent by the Charging Station to the CSMS in response to a [`CancelReservationRequest`]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationResponse<'a> {
    /// This indicates the success or failure of the canceling of a reservation by CSMS.
    pub status: CancelReservationStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none",borrow)]
    pub status_info: Option<StatusInfoType<'a>>,
}
