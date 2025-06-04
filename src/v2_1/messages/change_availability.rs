use crate::v2_1::datatypes::{CustomDataType, EVSEType, StatusInfoType};
use crate::v2_1::enumerations::{ChangeAvailabilityStatusEnumType, OperationalStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ChangeAvailability request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub evse: Option<EVSEType>,

    pub operational_status: OperationalStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChangeAvailabilityRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `operational_status` - The operational_status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(operational_status: OperationalStatusEnumType) -> Self {
        Self {
            evse: None,
            operational_status,
            custom_data: None,
        }
    }

    /// Sets the evse field.
    ///
    /// * `evse` - The evse field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse(&mut self, evse: Option<EVSEType>) -> &mut Self {
        self.evse = evse;
        self
    }

    /// Sets the operational_status field.
    ///
    /// * `operational_status` - The operational_status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_operational_status(&mut self, operational_status: OperationalStatusEnumType) -> &mut Self {
        self.operational_status = operational_status;
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

    /// Gets a reference to the evse field.
    ///
    /// # Returns
    ///
    /// The evse field
    pub fn get_evse(&self) -> Option<&EVSEType> {
        self.evse.as_ref()
    }

    /// Gets a reference to the operational_status field.
    ///
    /// # Returns
    ///
    /// The operational_status field
    pub fn get_operational_status(&self) -> &OperationalStatusEnumType {
        &self.operational_status
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the evse field and returns self for builder pattern.
    ///
    /// * `evse` - The evse field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
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

/// Response body for the ChangeAvailability response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    pub status: ChangeAvailabilityStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChangeAvailabilityResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ChangeAvailabilityStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: ChangeAvailabilityStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &ChangeAvailabilityStatusEnumType {
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

    fn create_test_evse() -> EVSEType {
        EVSEType::new(1).with_connector_id(1)
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("200".to_string())
    }

    #[test]
    fn test_change_availability_request_new() {
        let operational_status = OperationalStatusEnumType::Operative;
        let request = ChangeAvailabilityRequest::new(operational_status.clone());

        assert_eq!(request.get_operational_status(), &operational_status);
        assert_eq!(request.get_evse(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_change_availability_request_validation() {
        let operational_status = OperationalStatusEnumType::Operative;
        let request = ChangeAvailabilityRequest::new(operational_status);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_change_availability_request_validation_with_evse() {
        let operational_status = OperationalStatusEnumType::Operative;
        let evse = create_test_evse();
        let request = ChangeAvailabilityRequest::new(operational_status)
            .with_evse(evse);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_change_availability_request_validation_invalid_evse() {
        let operational_status = OperationalStatusEnumType::Operative;
        let invalid_evse = EVSEType {
            id: -1, // Invalid: must be >= 0
            connector_id: None,
            custom_data: None,
        };
        let mut request = ChangeAvailabilityRequest::new(operational_status);
        request.set_evse(Some(invalid_evse));

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_change_availability_request_serialization() {
        let operational_status = OperationalStatusEnumType::Operative;
        let request = ChangeAvailabilityRequest::new(operational_status);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_change_availability_request_with_evse() {
        let operational_status = OperationalStatusEnumType::Operative;
        let evse = create_test_evse();

        let request = ChangeAvailabilityRequest::new(operational_status)
            .with_evse(evse.clone());

        assert_eq!(request.get_evse(), Some(&evse));
    }

    #[test]
    fn test_change_availability_request_with_custom_data() {
        let operational_status = OperationalStatusEnumType::Operative;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = ChangeAvailabilityRequest::new(operational_status)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_change_availability_request_set_methods() {
        let operational_status = OperationalStatusEnumType::Operative;
        let new_operational_status = OperationalStatusEnumType::Inoperative;
        let evse = create_test_evse();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = ChangeAvailabilityRequest::new(operational_status);

        request
            .set_operational_status(new_operational_status.clone())
            .set_evse(Some(evse.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_operational_status(), &new_operational_status);
        assert_eq!(request.get_evse(), Some(&evse));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_change_availability_request_all_operational_status_values() {
        let operational_status_values = vec![
            OperationalStatusEnumType::Inoperative,
            OperationalStatusEnumType::Operative,
        ];

        for operational_status in operational_status_values {
            let request = ChangeAvailabilityRequest::new(operational_status.clone());
            assert_eq!(request.get_operational_status(), &operational_status);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_change_availability_request_with_complex_evse() {
        let operational_status = OperationalStatusEnumType::Operative;
        let custom_data = CustomDataType::new("EVSEVendor".to_string());
        let evse = EVSEType::new(5)
            .with_connector_id(3)
            .with_custom_data(custom_data);

        let request = ChangeAvailabilityRequest::new(operational_status)
            .with_evse(evse.clone());

        assert_eq!(request.get_evse(), Some(&evse));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_change_availability_response_new() {
        let status = ChangeAvailabilityStatusEnumType::Accepted;
        let response = ChangeAvailabilityResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_change_availability_response_validation() {
        let status = ChangeAvailabilityStatusEnumType::Accepted;
        let response = ChangeAvailabilityResponse::new(status);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_change_availability_response_serialization() {
        let status = ChangeAvailabilityStatusEnumType::Accepted;
        let response = ChangeAvailabilityResponse::new(status);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_change_availability_response_with_status_info() {
        let status = ChangeAvailabilityStatusEnumType::Accepted;
        let status_info = create_test_status_info();

        let response = ChangeAvailabilityResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_change_availability_response_with_custom_data() {
        let status = ChangeAvailabilityStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = ChangeAvailabilityResponse::new(status)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_change_availability_response_set_methods() {
        let status = ChangeAvailabilityStatusEnumType::Accepted;
        let new_status = ChangeAvailabilityStatusEnumType::Rejected;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = ChangeAvailabilityResponse::new(status);

        response
            .set_status(new_status.clone())
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &new_status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_change_availability_response_all_status_values() {
        let status_values = vec![
            ChangeAvailabilityStatusEnumType::Accepted,
            ChangeAvailabilityStatusEnumType::Rejected,
            ChangeAvailabilityStatusEnumType::Scheduled,
        ];

        for status in status_values {
            let response = ChangeAvailabilityResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_change_availability_response_builder_pattern() {
        let status = ChangeAvailabilityStatusEnumType::Scheduled;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = ChangeAvailabilityResponse::new(status.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_change_availability_request_json_round_trip() {
        let operational_status = OperationalStatusEnumType::Inoperative;
        let evse = create_test_evse();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = ChangeAvailabilityRequest::new(operational_status)
            .with_evse(evse)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_change_availability_response_json_round_trip() {
        let status = ChangeAvailabilityStatusEnumType::Scheduled;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = ChangeAvailabilityResponse::new(status)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_change_availability_response_with_detailed_status_info() {
        let status = ChangeAvailabilityStatusEnumType::Rejected;
        let status_info = StatusInfoType::new("EVSEBusy".to_string())
            .with_additional_info("EVSE is currently in use and cannot be made inoperative".to_string());

        let response = ChangeAvailabilityResponse::new(status)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());

        // Test serialization
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_change_availability_request_clear_optional_fields() {
        let operational_status = OperationalStatusEnumType::Operative;
        let evse = create_test_evse();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = ChangeAvailabilityRequest::new(operational_status)
            .with_evse(evse)
            .with_custom_data(custom_data);

        // Verify fields are set
        assert!(request.get_evse().is_some());
        assert!(request.get_custom_data().is_some());

        // Clear optional fields
        request.set_evse(None);
        request.set_custom_data(None);

        assert_eq!(request.get_evse(), None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_change_availability_response_clear_optional_fields() {
        let status = ChangeAvailabilityStatusEnumType::Accepted;
        let status_info = create_test_status_info();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = ChangeAvailabilityResponse::new(status)
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
    fn test_change_availability_request_with_complex_custom_data() {
        use serde_json::json;

        let operational_status = OperationalStatusEnumType::Inoperative;
        let custom_data = CustomDataType::new("AvailabilityVendor".to_string())
            .with_property("reason".to_string(), json!("maintenance"))
            .with_property("duration".to_string(), json!("2 hours"))
            .with_property("metadata".to_string(), json!({
                "scheduled_by": "operator",
                "priority": "high"
            }));

        let request = ChangeAvailabilityRequest::new(operational_status)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());

        // Test serialization with complex custom data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_change_availability_request_evse_edge_cases() {
        let operational_status = OperationalStatusEnumType::Operative;

        // Test with EVSE ID 0 (minimum valid value)
        let evse_zero = EVSEType::new(0);
        let request = ChangeAvailabilityRequest::new(operational_status.clone())
            .with_evse(evse_zero);
        assert!(request.validate().is_ok());

        // Test with large EVSE ID
        let evse_large = EVSEType::new(i32::MAX);
        let request = ChangeAvailabilityRequest::new(operational_status.clone())
            .with_evse(evse_large);
        assert!(request.validate().is_ok());

        // Test with EVSE without connector ID
        let evse_no_connector = EVSEType::new(1);
        let request = ChangeAvailabilityRequest::new(operational_status)
            .with_evse(evse_no_connector);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_change_availability_response_status_semantics() {
        // Test Accepted status - change was accepted and will be executed immediately
        let accepted_response = ChangeAvailabilityResponse::new(ChangeAvailabilityStatusEnumType::Accepted)
            .with_status_info(StatusInfoType::new("Success".to_string())
                .with_additional_info("Availability change executed immediately".to_string()));

        assert_eq!(accepted_response.get_status(), &ChangeAvailabilityStatusEnumType::Accepted);
        assert!(accepted_response.validate().is_ok());

        // Test Rejected status - change was rejected
        let rejected_response = ChangeAvailabilityResponse::new(ChangeAvailabilityStatusEnumType::Rejected)
            .with_status_info(StatusInfoType::new("EVSEBusy".to_string())
                .with_additional_info("EVSE is currently in use".to_string()));

        assert_eq!(rejected_response.get_status(), &ChangeAvailabilityStatusEnumType::Rejected);
        assert!(rejected_response.validate().is_ok());

        // Test Scheduled status - change was accepted but will be executed later
        let scheduled_response = ChangeAvailabilityResponse::new(ChangeAvailabilityStatusEnumType::Scheduled)
            .with_status_info(StatusInfoType::new("Scheduled".to_string())
                .with_additional_info("Change will be executed when current transaction ends".to_string()));

        assert_eq!(scheduled_response.get_status(), &ChangeAvailabilityStatusEnumType::Scheduled);
        assert!(scheduled_response.validate().is_ok());
    }

    #[test]
    fn test_change_availability_request_minimal_valid() {
        // Test minimal valid request (only required fields)
        let request = ChangeAvailabilityRequest::new(OperationalStatusEnumType::Operative);

        assert_eq!(request.get_operational_status(), &OperationalStatusEnumType::Operative);
        assert_eq!(request.get_evse(), None);
        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());

        // Test serialization of minimal request
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_change_availability_response_minimal_valid() {
        // Test minimal valid response (only required fields)
        let response = ChangeAvailabilityResponse::new(ChangeAvailabilityStatusEnumType::Accepted);

        assert_eq!(response.get_status(), &ChangeAvailabilityStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());

        // Test serialization of minimal response
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_change_availability_request_full_configuration() {
        // Test request with all optional fields set
        let operational_status = OperationalStatusEnumType::Inoperative;
        let evse = create_test_evse();
        let custom_data = CustomDataType::new("FullConfigVendor".to_string());

        let request = ChangeAvailabilityRequest::new(operational_status.clone())
            .with_evse(evse.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_operational_status(), &operational_status);
        assert_eq!(request.get_evse(), Some(&evse));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
        assert!(request.validate().is_ok());

        // Test serialization with all fields
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ChangeAvailabilityRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_change_availability_operational_status_use_cases() {
        // Test Operative status - make EVSE available for charging
        let operative_request = ChangeAvailabilityRequest::new(OperationalStatusEnumType::Operative);
        assert_eq!(operative_request.get_operational_status(), &OperationalStatusEnumType::Operative);
        assert!(operative_request.validate().is_ok());

        // Test Inoperative status - make EVSE unavailable for charging
        let inoperative_request = ChangeAvailabilityRequest::new(OperationalStatusEnumType::Inoperative);
        assert_eq!(inoperative_request.get_operational_status(), &OperationalStatusEnumType::Inoperative);
        assert!(inoperative_request.validate().is_ok());
    }

    #[test]
    fn test_change_availability_request_evse_targeting() {
        let operational_status = OperationalStatusEnumType::Inoperative;

        // Test targeting entire charging station (no EVSE specified)
        let station_request = ChangeAvailabilityRequest::new(operational_status.clone());
        assert_eq!(station_request.get_evse(), None);
        assert!(station_request.validate().is_ok());

        // Test targeting specific EVSE
        let evse = EVSEType::new(2);
        let evse_request = ChangeAvailabilityRequest::new(operational_status.clone())
            .with_evse(evse.clone());
        assert_eq!(evse_request.get_evse(), Some(&evse));
        assert!(evse_request.validate().is_ok());

        // Test targeting specific connector on EVSE
        let evse_with_connector = EVSEType::new(2).with_connector_id(1);
        let connector_request = ChangeAvailabilityRequest::new(operational_status)
            .with_evse(evse_with_connector.clone());
        assert_eq!(connector_request.get_evse(), Some(&evse_with_connector));
        assert!(connector_request.validate().is_ok());
    }
}