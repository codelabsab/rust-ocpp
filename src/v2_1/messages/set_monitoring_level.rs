use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetMonitoringLevel request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelRequest {
    /// The Charging Station SHALL only report events with a severity number lower than or equal to this severity. The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.  The severity levels have the following meaning: + *0-Danger* + Indicates lives are potentially in danger. Urgent attention is needed and action should be taken immediately. + *1-Hardware Failure* + Indicates that the Charging Station is unable to continue regular operations due to Hardware issues. Action is required. + *2-System Failure* + Indicates that the Charging Station is unable to continue regular operations due to software or minor hardware issues. Action is required. + *3-Critical* + Indicates a critical error. Action is required. + *4-Error* + Indicates a non-urgent error. Action is required. + *5-Alert* + Indicates an alert event. Default severity for any type of monitoring event.  + *6-Warning* + Indicates a warning event. Action may be required. + *7-Notice* + Indicates an unusual event. No immediate action is required. + *8-Informational* + Indicates a regular operational event. May be used for reporting, measuring throughput, etc. No action is required. + *9-Debug* + Indicates information useful to developers for debugging, not useful during operations.
    #[validate(range(min = 0))]
    pub severity: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetMonitoringLevelRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `severity` - The Charging Station SHALL only report events with a severity number lower than or equal to this severity. The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.  The severity levels have the following meaning: + *0-Danger* + Indicates lives are potentially in danger. Urgent attention is needed and action should be taken immediately. + *1-Hardware Failure* + Indicates that the Charging Station is unable to continue regular operations due to Hardware issues. Action is required. + *2-System Failure* + Indicates that the Charging Station is unable to continue regular operations due to software or minor hardware issues. Action is required. + *3-Critical* + Indicates a critical error. Action is required. + *4-Error* + Indicates a non-urgent error. Action is required. + *5-Alert* + Indicates an alert event. Default severity for any type of monitoring event.  + *6-Warning* + Indicates a warning event. Action may be required. + *7-Notice* + Indicates an unusual event. No immediate action is required. + *8-Informational* + Indicates a regular operational event. May be used for reporting, measuring throughput, etc. No action is required. + *9-Debug* + Indicates information useful to developers for debugging, not useful during operations.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(severity: i32) -> Self {
        Self {
            severity,
            custom_data: None,
        }
    }

    /// Sets the severity field.
    ///
    /// * `severity` - The Charging Station SHALL only report events with a severity number lower than or equal to this severity. The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.  The severity levels have the following meaning: + *0-Danger* + Indicates lives are potentially in danger. Urgent attention is needed and action should be taken immediately. + *1-Hardware Failure* + Indicates that the Charging Station is unable to continue regular operations due to Hardware issues. Action is required. + *2-System Failure* + Indicates that the Charging Station is unable to continue regular operations due to software or minor hardware issues. Action is required. + *3-Critical* + Indicates a critical error. Action is required. + *4-Error* + Indicates a non-urgent error. Action is required. + *5-Alert* + Indicates an alert event. Default severity for any type of monitoring event.  + *6-Warning* + Indicates a warning event. Action may be required. + *7-Notice* + Indicates an unusual event. No immediate action is required. + *8-Informational* + Indicates a regular operational event. May be used for reporting, measuring throughput, etc. No action is required. + *9-Debug* + Indicates information useful to developers for debugging, not useful during operations.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_severity(&mut self, severity: i32) -> &mut Self {
        self.severity = severity;
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

    /// Gets a reference to the severity field.
    ///
    /// # Returns
    ///
    /// The Charging Station SHALL only report events with a severity number lower than or equal to this severity. The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.  The severity levels have the following meaning: + *0-Danger* + Indicates lives are potentially in danger. Urgent attention is needed and action should be taken immediately. + *1-Hardware Failure* + Indicates that the Charging Station is unable to continue regular operations due to Hardware issues. Action is required. + *2-System Failure* + Indicates that the Charging Station is unable to continue regular operations due to software or minor hardware issues. Action is required. + *3-Critical* + Indicates a critical error. Action is required. + *4-Error* + Indicates a non-urgent error. Action is required. + *5-Alert* + Indicates an alert event. Default severity for any type of monitoring event.  + *6-Warning* + Indicates a warning event. Action may be required. + *7-Notice* + Indicates an unusual event. No immediate action is required. + *8-Informational* + Indicates a regular operational event. May be used for reporting, measuring throughput, etc. No action is required. + *9-Debug* + Indicates information useful to developers for debugging, not useful during operations.
    pub fn get_severity(&self) -> &i32 {
        &self.severity
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

/// Response body for the SetMonitoringLevel response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelResponse {
    pub status: GenericStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetMonitoringLevelResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GenericStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: GenericStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &GenericStatusEnumType {
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
    use crate::v2_1::enumerations::GenericStatusEnumType;

    #[test]
    fn test_set_monitoring_level_request_new() {
        let request = SetMonitoringLevelRequest::new(5);
        assert_eq!(request.severity, 5);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_set_monitoring_level_request_serialization() {
        let request = SetMonitoringLevelRequest::new(3);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetMonitoringLevelRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"severity\":3"));
    }

    #[test]
    fn test_set_monitoring_level_request_validation() {
        // Test valid severity levels (0-9)
        for severity in 0..=9 {
            let request = SetMonitoringLevelRequest::new(severity);
            assert!(request.validate().is_ok());
        }

        // Test invalid negative severity
        let invalid_request = SetMonitoringLevelRequest::new(-1);
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_set_monitoring_level_request_builder_pattern() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetMonitoringLevelRequest::new(7)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.severity, 7);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_monitoring_level_request_setters() {
        let mut request = SetMonitoringLevelRequest::new(0);
        let custom_data = CustomDataType::new("TestVendor".to_string());

        request.set_severity(9)
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.severity, 9);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_monitoring_level_request_getters() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetMonitoringLevelRequest::new(4)
            .with_custom_data(custom_data.clone());

        assert_eq!(*request.get_severity(), 4);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_monitoring_level_response_new() {
        let response = SetMonitoringLevelResponse::new(GenericStatusEnumType::Accepted);
        assert_eq!(response.status, GenericStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_set_monitoring_level_response_serialization() {
        let response = SetMonitoringLevelResponse::new(GenericStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetMonitoringLevelResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(json.contains("\"status\":\"Rejected\""));
    }

    #[test]
    fn test_set_monitoring_level_response_builder_pattern() {
        let status_info = StatusInfoType::new("Monitoring level conflict".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetMonitoringLevelResponse::new(GenericStatusEnumType::Rejected)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GenericStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_monitoring_level_response_setters() {
        let mut response = SetMonitoringLevelResponse::new(GenericStatusEnumType::Accepted);
        let status_info = StatusInfoType::new("Updated status".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        response.set_status(GenericStatusEnumType::Rejected)
                .set_status_info(Some(status_info.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, GenericStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_monitoring_level_response_getters() {
        let status_info = StatusInfoType::new("Test status".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetMonitoringLevelResponse::new(GenericStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*response.get_status(), GenericStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_monitoring_level_edge_cases() {
        // Test boundary severity levels
        let danger_level = SetMonitoringLevelRequest::new(0); // Danger
        let debug_level = SetMonitoringLevelRequest::new(9);  // Debug

        assert!(danger_level.validate().is_ok());
        assert!(debug_level.validate().is_ok());
        assert_eq!(danger_level.severity, 0);
        assert_eq!(debug_level.severity, 9);

        // Test all severity levels have specific meanings
        let severity_meanings = vec![
            (0, "Danger"),
            (1, "Hardware Failure"),
            (2, "System Failure"),
            (3, "Critical"),
            (4, "Error"),
            (5, "Alert"),
            (6, "Warning"),
            (7, "Notice"),
            (8, "Informational"),
            (9, "Debug"),
        ];

        for (severity, _meaning) in severity_meanings {
            let request = SetMonitoringLevelRequest::new(severity);
            assert!(request.validate().is_ok());
            assert_eq!(request.severity, severity);
        }
    }

    #[test]
    fn test_set_monitoring_level_response_validation() {
        let response = SetMonitoringLevelResponse::new(GenericStatusEnumType::Accepted);
        assert!(response.validate().is_ok());
    }
}