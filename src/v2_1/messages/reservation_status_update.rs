use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::enumerations::ReservationUpdateStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ReservationStatusUpdate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateRequest {
    /// The ID of the reservation.
    #[validate(range(min = 0))]
    pub reservation_id: i32,

    pub reservation_update_status: ReservationUpdateStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReservationStatusUpdateRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `reservation_id` - The ID of the reservation.
    /// * `reservation_update_status` - The reservation_update_status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(reservation_id: i32, reservation_update_status: ReservationUpdateStatusEnumType) -> Self {
        Self {
            reservation_id,
            reservation_update_status,
            custom_data: None,
        }
    }

    /// Sets the reservation_id field.
    ///
    /// * `reservation_id` - The ID of the reservation.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_reservation_id(&mut self, reservation_id: i32) -> &mut Self {
        self.reservation_id = reservation_id;
        self
    }

    /// Sets the reservation_update_status field.
    ///
    /// * `reservation_update_status` - The reservation_update_status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_reservation_update_status(&mut self, reservation_update_status: ReservationUpdateStatusEnumType) -> &mut Self {
        self.reservation_update_status = reservation_update_status;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the reservation_id field.
    ///
    /// # Returns
    ///
    /// The ID of the reservation.
    pub fn get_reservation_id(&self) -> &i32 {
        &self.reservation_id
    }

    /// Gets a reference to the reservation_update_status field.
    ///
    /// # Returns
    ///
    /// The reservation_update_status field
    pub fn get_reservation_update_status(&self) -> &ReservationUpdateStatusEnumType {
        &self.reservation_update_status
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

/// Response body for the ReservationStatusUpdate response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReservationStatusUpdateResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}
