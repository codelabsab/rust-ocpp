use crate::v2_1::datatypes::{ClearMonitoringResultType, CustomDataType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClearVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest {
    /// List of the monitors to be cleared, identified by there Id.
    #[validate(length(min = 1))]
    pub id: Vec<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearVariableMonitoringRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `id` - List of the monitors to be cleared, identified by there Id.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(id: Vec<i32>) -> Self {
        Self {
            id,
            custom_data: None,
        }
    }

    /// Sets the id field.
    ///
    /// * `id` - List of the monitors to be cleared, identified by there Id.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id(&mut self, id: Vec<i32>) -> &mut Self {
        self.id = id;
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

    /// Gets a reference to the id field.
    ///
    /// # Returns
    ///
    /// List of the monitors to be cleared, identified by there Id.
    pub fn get_id(&self) -> &Vec<i32> {
        &self.id
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

/// Response body for the ClearVariableMonitoring response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub clear_monitoring_result: Vec<ClearMonitoringResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearVariableMonitoringResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `clear_monitoring_result` - The clear_monitoring_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(clear_monitoring_result: Vec<ClearMonitoringResultType>) -> Self {
        Self {
            clear_monitoring_result,
            custom_data: None,
        }
    }

    /// Sets the clear_monitoring_result field.
    ///
    /// * `clear_monitoring_result` - The clear_monitoring_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_clear_monitoring_result(&mut self, clear_monitoring_result: Vec<ClearMonitoringResultType>) -> &mut Self {
        self.clear_monitoring_result = clear_monitoring_result;
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

    /// Gets a reference to the clear_monitoring_result field.
    ///
    /// # Returns
    ///
    /// The clear_monitoring_result field
    pub fn get_clear_monitoring_result(&self) -> &Vec<ClearMonitoringResultType> {
        &self.clear_monitoring_result
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
    use serde_json;
    use validator::Validate;

    fn create_test_clear_monitoring_result() -> ClearMonitoringResultType {
        ClearMonitoringResultType::new(crate::v2_1::enumerations::ClearMonitoringStatusEnumType::Accepted, 1)
    }

    #[test]
    fn test_clear_variable_monitoring_request_new() {
        let ids = vec![1, 2, 3];
        let request = ClearVariableMonitoringRequest::new(ids.clone());
        assert_eq!(request.get_id(), &ids);
        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_variable_monitoring_request_validation_empty_ids() {
        let ids = vec![]; // Invalid: must have at least 1 item
        let request = ClearVariableMonitoringRequest::new(ids);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_clear_variable_monitoring_request_serialization() {
        let ids = vec![10, 20, 30];
        let request = ClearVariableMonitoringRequest::new(ids);
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearVariableMonitoringRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_clear_variable_monitoring_response_new() {
        let results = vec![create_test_clear_monitoring_result()];
        let response = ClearVariableMonitoringResponse::new(results.clone());
        assert_eq!(response.get_clear_monitoring_result(), &results);
        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_variable_monitoring_response_validation_empty_results() {
        let results = vec![]; // Invalid: must have at least 1 item
        let response = ClearVariableMonitoringResponse::new(results);
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_clear_variable_monitoring_request_with_custom_data() {
        let ids = vec![100];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = ClearVariableMonitoringRequest::new(ids)
            .with_custom_data(custom_data.clone());
        assert_eq!(request.get_custom_data(), Some(&custom_data));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_variable_monitoring_response_with_custom_data() {
        let results = vec![create_test_clear_monitoring_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = ClearVariableMonitoringResponse::new(results)
            .with_custom_data(custom_data.clone());
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_variable_monitoring_json_round_trip() {
        let ids = vec![1, 2, 3, 4, 5];
        let request = ClearVariableMonitoringRequest::new(ids)
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearVariableMonitoringRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());

        let results = vec![
            create_test_clear_monitoring_result(),
            ClearMonitoringResultType::new(crate::v2_1::enumerations::ClearMonitoringStatusEnumType::Rejected, 2),
        ];
        let response = ClearVariableMonitoringResponse::new(results)
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearVariableMonitoringResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }
}