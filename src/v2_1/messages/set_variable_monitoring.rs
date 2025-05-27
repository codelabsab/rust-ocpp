use crate::v2_1::datatypes::{CustomDataType, SetMonitoringDataType, SetMonitoringResultType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub set_monitoring_data: Vec<SetMonitoringDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariableMonitoringRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `set_monitoring_data` - The set_monitoring_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(set_monitoring_data: Vec<SetMonitoringDataType>) -> Self {
        Self {
            set_monitoring_data,
            custom_data: None,
        }
    }

    /// Sets the set_monitoring_data field.
    ///
    /// * `set_monitoring_data` - The set_monitoring_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_set_monitoring_data(&mut self, set_monitoring_data: Vec<SetMonitoringDataType>) -> &mut Self {
        self.set_monitoring_data = set_monitoring_data;
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

    /// Gets a reference to the set_monitoring_data field.
    ///
    /// # Returns
    ///
    /// The set_monitoring_data field
    pub fn get_set_monitoring_data(&self) -> &Vec<SetMonitoringDataType> {
        &self.set_monitoring_data
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

/// Response body for the SetVariableMonitoring response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub set_monitoring_result: Vec<SetMonitoringResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariableMonitoringResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `set_monitoring_result` - The set_monitoring_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(set_monitoring_result: Vec<SetMonitoringResultType>) -> Self {
        Self {
            set_monitoring_result,
            custom_data: None,
        }
    }

    /// Sets the set_monitoring_result field.
    ///
    /// * `set_monitoring_result` - The set_monitoring_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_set_monitoring_result(&mut self, set_monitoring_result: Vec<SetMonitoringResultType>) -> &mut Self {
        self.set_monitoring_result = set_monitoring_result;
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

    /// Gets a reference to the set_monitoring_result field.
    ///
    /// # Returns
    ///
    /// The set_monitoring_result field
    pub fn get_set_monitoring_result(&self) -> &Vec<SetMonitoringResultType> {
        &self.set_monitoring_result
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
    use crate::v2_1::datatypes::{ComponentType, CustomDataType, StatusInfoType, VariableType};
    use crate::v2_1::enumerations::{MonitorEnumType, SetMonitoringStatusEnumType};
    use rust_decimal::Decimal;
    use serde_json;
    use validator::Validate;

    // Helper function to create test SetMonitoringDataType
    fn create_test_monitoring_data() -> SetMonitoringDataType {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("Current".to_string());
        SetMonitoringDataType::new(
            Decimal::new(100, 0), // value: 100
            MonitorEnumType::UpperThreshold,
            5, // severity
            component,
            variable,
        )
    }

    // Helper function to create test SetMonitoringResultType
    fn create_test_monitoring_result() -> SetMonitoringResultType {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("Current".to_string());
        SetMonitoringResultType::new(
            SetMonitoringStatusEnumType::Accepted,
            component,
            variable,
            MonitorEnumType::UpperThreshold,
            5, // severity
        )
    }

    // Tests for SetVariableMonitoringRequest

    #[test]
    fn test_set_variable_monitoring_request_new() {
        let monitoring_data = vec![create_test_monitoring_data()];
        let request = SetVariableMonitoringRequest::new(monitoring_data.clone());

        assert_eq!(request.get_set_monitoring_data().len(), 1);
        assert_eq!(request.get_set_monitoring_data()[0].component.name, "Connector");
        assert_eq!(request.get_set_monitoring_data()[0].variable.name, "Current");
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_set_variable_monitoring_request_serialization() {
        let monitoring_data = vec![create_test_monitoring_data()];
        let request = SetVariableMonitoringRequest::new(monitoring_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetVariableMonitoringRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_set_variable_monitoring_request_validation() {
        let monitoring_data = vec![create_test_monitoring_data()];
        let request = SetVariableMonitoringRequest::new(monitoring_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_variable_monitoring_request_with_custom_data() {
        let monitoring_data = vec![create_test_monitoring_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetVariableMonitoringRequest::new(monitoring_data)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variable_monitoring_request_set_methods() {
        let monitoring_data = vec![create_test_monitoring_data()];
        let new_monitoring_data = vec![create_test_monitoring_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = SetVariableMonitoringRequest::new(monitoring_data);

        request
            .set_set_monitoring_data(new_monitoring_data.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_set_monitoring_data(), &new_monitoring_data);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variable_monitoring_request_builder_pattern() {
        let monitoring_data = vec![create_test_monitoring_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = SetVariableMonitoringRequest::new(monitoring_data)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variable_monitoring_request_validation_empty_vector() {
        let request = SetVariableMonitoringRequest::new(vec![]);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_set_variable_monitoring_request_with_multiple_data() {
        let data = vec![
            create_test_monitoring_data(),
            {
                let component = ComponentType::new("EVSE".to_string());
                let variable = VariableType::new("Power".to_string());
                SetMonitoringDataType::new(
                    Decimal::new(500, 0),
                    MonitorEnumType::LowerThreshold,
                    3,
                    component,
                    variable,
                )
            },
        ];

        let request = SetVariableMonitoringRequest::new(data.clone());

        assert_eq!(request.get_set_monitoring_data().len(), 2);
        assert_eq!(request.get_set_monitoring_data()[1].component.name, "EVSE");
        assert_eq!(request.get_set_monitoring_data()[1].variable.name, "Power");
        assert!(request.validate().is_ok());
    }

    // Tests for SetVariableMonitoringResponse

    #[test]
    fn test_set_variable_monitoring_response_new() {
        let monitoring_result = vec![create_test_monitoring_result()];
        let response = SetVariableMonitoringResponse::new(monitoring_result.clone());

        assert_eq!(response.get_set_monitoring_result().len(), 1);
        assert_eq!(response.get_set_monitoring_result()[0].component.name, "Connector");
        assert_eq!(response.get_set_monitoring_result()[0].variable.name, "Current");
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_set_variable_monitoring_response_serialization() {
        let monitoring_result = vec![create_test_monitoring_result()];
        let response = SetVariableMonitoringResponse::new(monitoring_result);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetVariableMonitoringResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_set_variable_monitoring_response_validation() {
        let monitoring_result = vec![create_test_monitoring_result()];
        let response = SetVariableMonitoringResponse::new(monitoring_result);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_set_variable_monitoring_response_with_custom_data() {
        let monitoring_result = vec![create_test_monitoring_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetVariableMonitoringResponse::new(monitoring_result)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variable_monitoring_response_set_methods() {
        let monitoring_result = vec![create_test_monitoring_result()];
        let new_monitoring_result = vec![create_test_monitoring_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = SetVariableMonitoringResponse::new(monitoring_result);

        response
            .set_set_monitoring_result(new_monitoring_result.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_set_monitoring_result(), &new_monitoring_result);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variable_monitoring_response_builder_pattern() {
        let monitoring_result = vec![create_test_monitoring_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = SetVariableMonitoringResponse::new(monitoring_result)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variable_monitoring_response_validation_empty_vector() {
        let response = SetVariableMonitoringResponse::new(vec![]);

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_set_variable_monitoring_response_with_multiple_results() {
        let results = vec![
            create_test_monitoring_result(),
            {
                let component = ComponentType::new("EVSE".to_string());
                let variable = VariableType::new("Power".to_string());
                SetMonitoringResultType::new(
                    SetMonitoringStatusEnumType::Rejected,
                    component,
                    variable,
                    MonitorEnumType::Delta,
                    4,
                ).with_status_info(StatusInfoType::new("Rejected".to_string()))
            },
        ];

        let response = SetVariableMonitoringResponse::new(results.clone());

        assert_eq!(response.get_set_monitoring_result().len(), 2);
        assert_eq!(response.get_set_monitoring_result()[1].component.name, "EVSE");
        assert_eq!(response.get_set_monitoring_result()[1].variable.name, "Power");
        assert_eq!(response.get_set_monitoring_result()[1].status, SetMonitoringStatusEnumType::Rejected);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_set_variable_monitoring_request_json_round_trip() {
        let monitoring_data = vec![create_test_monitoring_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetVariableMonitoringRequest::new(monitoring_data)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetVariableMonitoringRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_set_variable_monitoring_response_json_round_trip() {
        let monitoring_result = vec![create_test_monitoring_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetVariableMonitoringResponse::new(monitoring_result)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetVariableMonitoringResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_set_variable_monitoring_all_monitor_types() {
        let monitor_types = vec![
            MonitorEnumType::UpperThreshold,
            MonitorEnumType::LowerThreshold,
            MonitorEnumType::Delta,
            MonitorEnumType::Periodic,
            MonitorEnumType::PeriodicClockAligned,
        ];

        for monitor_type in monitor_types {
            let component = ComponentType::new("TestComponent".to_string());
            let variable = VariableType::new("TestVariable".to_string());
            let monitoring_data = SetMonitoringDataType::new(
                Decimal::new(100, 0),
                monitor_type.clone(),
                5,
                component,
                variable,
            );

            let request = SetVariableMonitoringRequest::new(vec![monitoring_data]);
            assert!(request.validate().is_ok());

            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: SetVariableMonitoringRequest = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request, deserialized);
        }
    }

    #[test]
    fn test_set_variable_monitoring_all_status_types() {
        let status_types = vec![
            SetMonitoringStatusEnumType::Accepted,
            SetMonitoringStatusEnumType::UnknownComponent,
            SetMonitoringStatusEnumType::UnknownVariable,
            SetMonitoringStatusEnumType::UnsupportedMonitorType,
            SetMonitoringStatusEnumType::Rejected,
            SetMonitoringStatusEnumType::OutOfRange,
        ];

        for status in status_types {
            let component = ComponentType::new("TestComponent".to_string());
            let variable = VariableType::new("TestVariable".to_string());
            let monitoring_result = SetMonitoringResultType::new(
                status.clone(),
                component,
                variable,
                MonitorEnumType::UpperThreshold,
                5,
            );

            let response = SetVariableMonitoringResponse::new(vec![monitoring_result]);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: SetVariableMonitoringResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_set_variable_monitoring_with_custom_data_validation() {
        let monitoring_data = vec![create_test_monitoring_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetVariableMonitoringRequest::new(monitoring_data)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_variable_monitoring_response_with_all_optional_fields() {
        let component = ComponentType::new("TestComponent".to_string());
        let variable = VariableType::new("TestVariable".to_string());
        let status_info = StatusInfoType::new("Success".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let monitoring_result = SetMonitoringResultType::new(
            SetMonitoringStatusEnumType::Accepted,
            component,
            variable,
            MonitorEnumType::UpperThreshold,
            5,
        )
        .with_id(123)
        .with_status_info(status_info.clone())
        .with_custom_data(custom_data.clone());

        let response = SetVariableMonitoringResponse::new(vec![monitoring_result])
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_set_monitoring_result()[0].id, Some(123));
        assert_eq!(response.get_set_monitoring_result()[0].status_info, Some(status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }
}