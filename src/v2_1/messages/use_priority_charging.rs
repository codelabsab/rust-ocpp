use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::PriorityChargingStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the UsePriorityCharging request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingRequest {
    /// The transaction for which priority charging is requested.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    /// True to request priority charging. False to request stopping priority charging.
    pub activate: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UsePriorityChargingRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `transaction_id` - The transaction for which priority charging is requested.
    /// * `activate` - True to request priority charging. False to request stopping priority charging.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(transaction_id: String, activate: bool) -> Self {
        Self {
            transaction_id,
            activate,
            custom_data: None,
        }
    }

    /// Sets the transaction_id field.
    ///
    /// * `transaction_id` - The transaction for which priority charging is requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_id(&mut self, transaction_id: String) -> &mut Self {
        self.transaction_id = transaction_id;
        self
    }

    /// Sets the activate field.
    ///
    /// * `activate` - True to request priority charging. False to request stopping priority charging.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_activate(&mut self, activate: bool) -> &mut Self {
        self.activate = activate;
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

    /// Gets a reference to the transaction_id field.
    ///
    /// # Returns
    ///
    /// The transaction for which priority charging is requested.
    pub fn get_transaction_id(&self) -> &String {
        &self.transaction_id
    }

    /// Gets a reference to the activate field.
    ///
    /// # Returns
    ///
    /// True to request priority charging. False to request stopping priority charging.
    pub fn get_activate(&self) -> &bool {
        &self.activate
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

/// Response body for the UsePriorityCharging response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingResponse {
    pub status: PriorityChargingStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UsePriorityChargingResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: PriorityChargingStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: PriorityChargingStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &PriorityChargingStatusEnumType {
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
    use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
    use crate::v2_1::enumerations::PriorityChargingStatusEnumType;
    use serde_json;
    use validator::Validate;

    // Tests for UsePriorityChargingRequest

    #[test]
    fn test_use_priority_charging_request_new() {
        let transaction_id = "txn123".to_string();
        let activate = true;
        let request = UsePriorityChargingRequest::new(transaction_id.clone(), activate);

        assert_eq!(request.get_transaction_id(), &transaction_id);
        assert_eq!(request.get_activate(), &activate);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_use_priority_charging_request_serialization() {
        let request = UsePriorityChargingRequest::new("transaction456".to_string(), false);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UsePriorityChargingRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_use_priority_charging_request_validation() {
        let request = UsePriorityChargingRequest::new("valid_transaction".to_string(), true);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_use_priority_charging_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UsePriorityChargingRequest::new("txn789".to_string(), true)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_use_priority_charging_request_set_methods() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = UsePriorityChargingRequest::new("original".to_string(), true);

        request
            .set_transaction_id("updated".to_string())
            .set_activate(false)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_transaction_id(), "updated");
        assert_eq!(request.get_activate(), &false);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_use_priority_charging_request_builder_pattern() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = UsePriorityChargingRequest::new("builder_test".to_string(), false)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_use_priority_charging_request_transaction_id_validation() {
        let long_transaction_id = "a".repeat(36); // Max allowed length
        let request = UsePriorityChargingRequest::new(long_transaction_id, true);

        assert!(request.validate().is_ok());

        let too_long_transaction_id = "a".repeat(37); // Exceeds max length
        let invalid_request = UsePriorityChargingRequest::new(too_long_transaction_id, true);

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_use_priority_charging_request_activate_deactivate() {
        let request_activate = UsePriorityChargingRequest::new("txn_activate".to_string(), true);
        let request_deactivate = UsePriorityChargingRequest::new("txn_deactivate".to_string(), false);

        assert_eq!(request_activate.get_activate(), &true);
        assert_eq!(request_deactivate.get_activate(), &false);
        assert!(request_activate.validate().is_ok());
        assert!(request_deactivate.validate().is_ok());
    }

    // Tests for UsePriorityChargingResponse

    #[test]
    fn test_use_priority_charging_response_new() {
        let status = PriorityChargingStatusEnumType::Accepted;
        let response = UsePriorityChargingResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_use_priority_charging_response_serialization() {
        let response = UsePriorityChargingResponse::new(PriorityChargingStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UsePriorityChargingResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_use_priority_charging_response_validation() {
        let response = UsePriorityChargingResponse::new(PriorityChargingStatusEnumType::NoProfile);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_use_priority_charging_response_with_status_info() {
        let status_info = StatusInfoType::new("Success".to_string());
        let response = UsePriorityChargingResponse::new(PriorityChargingStatusEnumType::Accepted)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_use_priority_charging_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UsePriorityChargingResponse::new(PriorityChargingStatusEnumType::Rejected)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_use_priority_charging_response_set_methods() {
        let status_info = StatusInfoType::new("Error".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = UsePriorityChargingResponse::new(PriorityChargingStatusEnumType::Accepted);

        response
            .set_status(PriorityChargingStatusEnumType::NoProfile)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &PriorityChargingStatusEnumType::NoProfile);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_use_priority_charging_response_builder_pattern() {
        let status_info = StatusInfoType::new("Info".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UsePriorityChargingResponse::new(PriorityChargingStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_use_priority_charging_response_all_status_types() {
        let status_types = vec![
            PriorityChargingStatusEnumType::Accepted,
            PriorityChargingStatusEnumType::Rejected,
            PriorityChargingStatusEnumType::NoProfile,
        ];

        for status in status_types {
            let response = UsePriorityChargingResponse::new(status.clone());
            
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: UsePriorityChargingResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_use_priority_charging_request_json_round_trip() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UsePriorityChargingRequest::new("final_test".to_string(), true)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UsePriorityChargingRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_use_priority_charging_response_json_round_trip() {
        let status_info = StatusInfoType::new("Complete".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UsePriorityChargingResponse::new(PriorityChargingStatusEnumType::Accepted)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UsePriorityChargingResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_use_priority_charging_response_with_all_optional_fields() {
        let status_info = StatusInfoType::new("Done".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UsePriorityChargingResponse::new(PriorityChargingStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &PriorityChargingStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_use_priority_charging_request_empty_transaction_id() {
        let request = UsePriorityChargingRequest::new("".to_string(), true);

        assert!(request.validate().is_ok()); // Empty string is allowed
    }

    #[test]
    fn test_use_priority_charging_request_with_custom_data_validation() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UsePriorityChargingRequest::new("test_transaction".to_string(), false)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }
}