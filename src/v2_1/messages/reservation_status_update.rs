use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::custom_data::CustomDataType;
use crate::v2_1::enumerations::reservation_update_status::ReservationUpdateStatusEnumType;

/// Request body for the ReservationStatusUpdate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The ID of the reservation.
    #[validate(range(min = 0))]
    pub reservation_id: i32,

    /// Required. The updated reservation status.
    pub reservation_update_status: ReservationUpdateStatusEnumType,
}

/// Response body for the ReservationStatusUpdate response.
/// This contains no fields as per the OCPP 2.1 specification.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl ReservationStatusUpdateRequest {
    /// Creates a new `ReservationStatusUpdateRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `reservation_id` - The ID of the reservation
    /// * `reservation_update_status` - The updated reservation status
    ///
    /// # Returns
    ///
    /// A new instance of `ReservationStatusUpdateRequest` with optional fields set to `None`
    pub fn new(
        reservation_id: i32,
        reservation_update_status: ReservationUpdateStatusEnumType,
    ) -> Self {
        Self {
            custom_data: None,
            reservation_id,
            reservation_update_status,
        }
    }
}

impl ReservationStatusUpdateResponse {
    /// Creates a new `ReservationStatusUpdateResponse`.
    ///
    /// # Returns
    ///
    /// A new instance of `ReservationStatusUpdateResponse` with optional fields set to `None`
    pub fn new() -> Self {
        Self { custom_data: None }
    }
}
