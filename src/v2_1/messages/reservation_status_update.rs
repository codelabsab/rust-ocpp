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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::ReservationUpdateStatusEnumType;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    #[test]
    fn test_reservation_status_update_request_new() {
        let request = ReservationStatusUpdateRequest::new(
            123,
            ReservationUpdateStatusEnumType::Accepted
        );

        assert_eq!(request.reservation_id, 123);
        assert_eq!(request.reservation_update_status, ReservationUpdateStatusEnumType::Accepted);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_reservation_status_update_request_serialization() {
        let request = ReservationStatusUpdateRequest::new(
            456,
            ReservationUpdateStatusEnumType::Rejected
        )
        .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ReservationStatusUpdateRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.reservation_id, 456);
        assert_eq!(deserialized.reservation_update_status, ReservationUpdateStatusEnumType::Rejected);
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_reservation_status_update_request_validation() {
        // Valid request
        let valid_request = ReservationStatusUpdateRequest::new(
            0,
            ReservationUpdateStatusEnumType::Accepted
        );
        assert!(valid_request.validate().is_ok());

        // Invalid reservation_id (negative)
        let invalid_request = ReservationStatusUpdateRequest {
            reservation_id: -1,
            reservation_update_status: ReservationUpdateStatusEnumType::Accepted,
            custom_data: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_reservation_status_update_request_builder_pattern() {
        let custom_data = create_test_custom_data();
        let request = ReservationStatusUpdateRequest::new(
            789,
            ReservationUpdateStatusEnumType::Failed
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(request.reservation_id, 789);
        assert_eq!(request.reservation_update_status, ReservationUpdateStatusEnumType::Failed);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_reservation_status_update_request_setters_getters() {
        let mut request = ReservationStatusUpdateRequest::new(
            100,
            ReservationUpdateStatusEnumType::Accepted
        );
        let custom_data = create_test_custom_data();

        // Test setters
        request.set_reservation_id(200);
        request.set_reservation_update_status(ReservationUpdateStatusEnumType::Rejected);
        request.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*request.get_reservation_id(), 200);
        assert_eq!(*request.get_reservation_update_status(), ReservationUpdateStatusEnumType::Rejected);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_reservation_status_update_response_new() {
        let response = ReservationStatusUpdateResponse::new();

        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_reservation_status_update_response_serialization() {
        let response = ReservationStatusUpdateResponse::new()
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ReservationStatusUpdateResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_reservation_status_update_response_validation() {
        let valid_response = ReservationStatusUpdateResponse::new();
        assert!(valid_response.validate().is_ok());

        let response_with_custom_data = ReservationStatusUpdateResponse::new()
            .with_custom_data(create_test_custom_data());
        assert!(response_with_custom_data.validate().is_ok());
    }

    #[test]
    fn test_reservation_status_update_response_builder_pattern() {
        let custom_data = create_test_custom_data();
        let response = ReservationStatusUpdateResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_reservation_status_update_response_setters_getters() {
        let mut response = ReservationStatusUpdateResponse::new();
        let custom_data = create_test_custom_data();

        // Test setter
        response.set_custom_data(Some(custom_data.clone()));

        // Test getter
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_reservation_status_update_request_enum_variants() {
        let accepted_request = ReservationStatusUpdateRequest::new(
            1,
            ReservationUpdateStatusEnumType::Accepted
        );
        assert_eq!(accepted_request.reservation_update_status, ReservationUpdateStatusEnumType::Accepted);

        let failed_request = ReservationStatusUpdateRequest::new(
            2,
            ReservationUpdateStatusEnumType::Failed
        );
        assert_eq!(failed_request.reservation_update_status, ReservationUpdateStatusEnumType::Failed);

        let rejected_request = ReservationStatusUpdateRequest::new(
            3,
            ReservationUpdateStatusEnumType::Rejected
        );
        assert_eq!(rejected_request.reservation_update_status, ReservationUpdateStatusEnumType::Rejected);
    }

    #[test]
    fn test_reservation_status_update_request_edge_cases() {
        // Test minimum valid reservation_id
        let min_request = ReservationStatusUpdateRequest::new(
            0,
            ReservationUpdateStatusEnumType::Accepted
        );
        assert!(min_request.validate().is_ok());

        // Test large reservation_id
        let large_request = ReservationStatusUpdateRequest::new(
            i32::MAX,
            ReservationUpdateStatusEnumType::Failed
        );
        assert!(large_request.validate().is_ok());
    }

    #[test]
    fn test_reservation_status_update_json_round_trip() {
        let original_request = ReservationStatusUpdateRequest::new(
            12345,
            ReservationUpdateStatusEnumType::Accepted
        )
        .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_request).expect("Failed to serialize request");
        let parsed_request: ReservationStatusUpdateRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");

        assert_eq!(original_request, parsed_request);

        let original_response = ReservationStatusUpdateResponse::new()
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_response).expect("Failed to serialize response");
        let parsed_response: ReservationStatusUpdateResponse =
            serde_json::from_str(&json).expect("Failed to deserialize response");

        assert_eq!(original_response, parsed_response);
    }
}