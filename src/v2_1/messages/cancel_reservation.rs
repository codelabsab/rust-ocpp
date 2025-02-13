use super::super::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};

/// Request to cancel a reservation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelReservationRequest {
    /// Id of the reservation to cancel.
    pub reservation_id: i32,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

use super::super::datatypes::StatusInfoType;
use super::super::enumerations::CancelReservationStatusEnumType;

/// Response to a CancelReservationRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelReservationResponse {
    /// This indicates the success or failure of the canceling of a reservation by CSMS.
    pub status: CancelReservationStatusEnumType,

    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
