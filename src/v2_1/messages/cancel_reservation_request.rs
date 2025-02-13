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
