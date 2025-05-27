use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::CancelReservationStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the CancelReservation request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationRequest {
    /// Id of the reservation to cancel.
    #[validate(range(min = 0))]
    pub reservation_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CancelReservationRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `reservation_id` - Id of the reservation to cancel.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(reservation_id: i32) -> Self {
        Self {
            reservation_id,
            custom_data: None,
        }
    }

    /// Sets the reservation_id field.
    ///
    /// * `reservation_id` - Id of the reservation to cancel.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_reservation_id(&mut self, reservation_id: i32) -> &mut Self {
        self.reservation_id = reservation_id;
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
    /// Id of the reservation to cancel.
    pub fn get_reservation_id(&self) -> &i32 {
        &self.reservation_id
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

/// Response body for the CancelReservation response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationResponse {
    pub status: CancelReservationStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CancelReservationResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: CancelReservationStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: CancelReservationStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the status_info field.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
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

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &CancelReservationStatusEnumType {
        &self.status
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the status_info field and returns self for builder pattern.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
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
    use serde_json;
    use validator::Validate;

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("200".to_string())
    }

    #[test]
    fn test_cancel_reservation_request_new() {
        let reservation_id = 123;
        let request = CancelReservationRequest::new(reservation_id);

        assert_eq!(request.get_reservation_id(), &reservation_id);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_cancel_reservation_request_validation() {
        let reservation_id = 123;
        let request = CancelReservationRequest::new(reservation_id);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_cancel_reservation_request_validation_invalid_id() {
        let reservation_id = -1; // Invalid: must be >= 0
        let request = CancelReservationRequest::new(reservation_id);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_cancel_reservation_request_serialization() {
        let reservation_id = 123;
        let request = CancelReservationRequest::new(reservation_id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CancelReservationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_cancel_reservation_request_with_custom_data() {
        let reservation_id = 123;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = CancelReservationRequest::new(reservation_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_cancel_reservation_request_set_methods() {
        let reservation_id = 123;
        let new_reservation_id = 456;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = CancelReservationRequest::new(reservation_id);

        request
            .set_reservation_id(new_reservation_id)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_reservation_id(), &new_reservation_id);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_cancel_reservation_request_edge_cases() {
        // Test with minimum valid reservation_id
        let request = CancelReservationRequest::new(0);
        assert_eq!(request.get_reservation_id(), &0);
        assert!(request.validate().is_ok());

        // Test with large reservation_id
        let request = CancelReservationRequest::new(i32::MAX);
        assert_eq!(request.get_reservation_id(), &i32::MAX);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_cancel_reservation_response_new() {
        let status = CancelReservationStatusEnumType::Accepted;
        let response = CancelReservationResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_cancel_reservation_response_validation() {
        let status = CancelReservationStatusEnumType::Accepted;
        let response = CancelReservationResponse::new(status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_cancel_reservation_response_serialization() {
        let status = CancelReservationStatusEnumType::Accepted;
        let response = CancelReservationResponse::new(status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CancelReservationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_cancel_reservation_response_with_status_info() {
        let status = CancelReservationStatusEnumType::Accepted;
        let status_info = create_test_status_info();

        let response = CancelReservationResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_cancel_reservation_response_with_custom_data() {
        let status = CancelReservationStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = CancelReservationResponse::new(status)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_cancel_reservation_response_set_methods() {
        let status = CancelReservationStatusEnumType::Accepted;
        let new_status = CancelReservationStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = CancelReservationResponse::new(status);

        response
            .set_status(new_status.clone())
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &new_status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_cancel_reservation_response_all_status_values() {
        let status_values = vec![
            CancelReservationStatusEnumType::Accepted,
            CancelReservationStatusEnumType::Rejected,
        ];

        for status in status_values {
            let response = CancelReservationResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_cancel_reservation_response_builder_pattern() {
        let status = CancelReservationStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = CancelReservationResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_cancel_reservation_request_json_round_trip() {
        let reservation_id = 123;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = CancelReservationRequest::new(reservation_id)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CancelReservationRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_cancel_reservation_response_json_round_trip() {
        let status = CancelReservationStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = CancelReservationResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CancelReservationResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_cancel_reservation_response_with_detailed_status_info() {
        let status = CancelReservationStatusEnumType::Rejected;
        let status_info = StatusInfoType::new("NoReservation".to_string())
            .with_additional_info("No active reservation found with the given ID".to_string());

        let response = CancelReservationResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());

        // Test serialization
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CancelReservationResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_cancel_reservation_request_clear_optional_fields() {
        let reservation_id = 123;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = CancelReservationRequest::new(reservation_id)
            .with_custom_data(custom_data);

        // Verify custom data is set
        assert!(request.get_custom_data().is_some());

        // Clear custom data
        request.set_custom_data(None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_cancel_reservation_response_clear_optional_fields() {
        let status = CancelReservationStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = CancelReservationResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        // Verify fields are set
        assert!(response.get_status_info().is_some());
        assert!(response.get_custom_data().is_some());

        // Clear optional fields
        response.set_status_info(None);
        response.set_custom_data(None);

        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_cancel_reservation_request_with_complex_custom_data() {
        use serde_json::json;

        let reservation_id = 123;
        let custom_data = CustomDataType::new("ReservationVendor".to_string())
            .with_property("station_id".to_string(), json!("STATION_001"))
            .with_property("operator".to_string(), json!("John Doe"))
            .with_property("metadata".to_string(), json!({
                "cancel_reason": "user_request",
                "timestamp": "2023-12-25T10:30:00Z"
            }));

        let request = CancelReservationRequest::new(reservation_id)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());

        // Test serialization with complex custom data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CancelReservationRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_cancel_reservation_request_reservation_id_ranges() {
        // Test typical reservation ID values
        let reservation_ids = vec![0, 1, 100, 1000, 9999, 123456, i32::MAX];

        for reservation_id in reservation_ids {
            let request = CancelReservationRequest::new(reservation_id);
            assert_eq!(request.get_reservation_id(), &reservation_id);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_cancel_reservation_response_status_semantics() {
        // Test Accepted status - reservation was successfully cancelled
        let accepted_response = CancelReservationResponse::new(CancelReservationStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("Success".to_string())
                .with_additional_info("Reservation cancelled successfully".to_string()));

        assert_eq!(accepted_response.get_status(), &CancelReservationStatusEnumType::Accepted);
        assert!(accepted_response.validate().is_ok());

        // Test Rejected status - reservation could not be cancelled
        let rejected_response = CancelReservationResponse::new(CancelReservationStatusEnumType::Rejected)
            .with_status_info(StatusInfoType::new("NoReservation".to_string())
                .with_additional_info("No active reservation found".to_string()));

        assert_eq!(rejected_response.get_status(), &CancelReservationStatusEnumType::Rejected);
        assert!(rejected_response.validate().is_ok());
    }

    #[test]
    fn test_cancel_reservation_request_minimal_valid() {
        // Test minimal valid request (only required fields)
        let request = CancelReservationRequest::new(1);

        assert_eq!(request.get_reservation_id(), &1);
        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());

        // Test serialization of minimal request
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CancelReservationRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_cancel_reservation_response_minimal_valid() {
        // Test minimal valid response (only required fields)
        let response = CancelReservationResponse::new(CancelReservationStatusEnumType::Accepted);

        assert_eq!(response.get_status(), &CancelReservationStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());

        // Test serialization of minimal response
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: CancelReservationResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }
}