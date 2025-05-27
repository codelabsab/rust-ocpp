use crate::v2_1::datatypes::{CustomDataType, SetVariableDataType, SetVariableResultType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub set_variable_data: Vec<SetVariableDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariablesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `set_variable_data` - The set_variable_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(set_variable_data: Vec<SetVariableDataType>) -> Self {
        Self {
            set_variable_data,
            custom_data: None,
        }
    }

    /// Sets the set_variable_data field.
    ///
    /// * `set_variable_data` - The set_variable_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_set_variable_data(&mut self, set_variable_data: Vec<SetVariableDataType>) -> &mut Self {
        self.set_variable_data = set_variable_data;
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

    /// Gets a reference to the set_variable_data field.
    ///
    /// # Returns
    ///
    /// The set_variable_data field
    pub fn get_set_variable_data(&self) -> &Vec<SetVariableDataType> {
        &self.set_variable_data
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

/// Response body for the SetVariables response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub set_variable_result: Vec<SetVariableResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariablesResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `set_variable_result` - The set_variable_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(set_variable_result: Vec<SetVariableResultType>) -> Self {
        Self {
            set_variable_result,
            custom_data: None,
        }
    }

    /// Sets the set_variable_result field.
    ///
    /// * `set_variable_result` - The set_variable_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_set_variable_result(&mut self, set_variable_result: Vec<SetVariableResultType>) -> &mut Self {
        self.set_variable_result = set_variable_result;
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

    /// Gets a reference to the set_variable_result field.
    ///
    /// # Returns
    ///
    /// The set_variable_result field
    pub fn get_set_variable_result(&self) -> &Vec<SetVariableResultType> {
        &self.set_variable_result
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
    use crate::v2_1::enumerations::{AttributeEnumType, SetVariableStatusEnumType};
    use serde_json;
    use validator::Validate;

    // Helper function to create test SetVariableDataType
    fn create_test_variable_data() -> SetVariableDataType {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("CurrentLimit".to_string());
        SetVariableDataType::new(component, variable, "32".to_string())
    }

    // Helper function to create test SetVariableResultType
    fn create_test_variable_result() -> SetVariableResultType {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("CurrentLimit".to_string());
        SetVariableResultType::new(component, variable, SetVariableStatusEnumType::Accepted)
    }

    // Tests for SetVariablesRequest

    #[test]
    fn test_set_variables_request_new() {
        let variable_data = vec![create_test_variable_data()];
        let request = SetVariablesRequest::new(variable_data.clone());

        assert_eq!(request.get_set_variable_data().len(), 1);
        assert_eq!(request.get_set_variable_data()[0].component.name, "Connector");
        assert_eq!(request.get_set_variable_data()[0].variable.name, "CurrentLimit");
        assert_eq!(request.get_set_variable_data()[0].attribute_value, "32");
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_set_variables_request_serialization() {
        let variable_data = vec![create_test_variable_data()];
        let request = SetVariablesRequest::new(variable_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetVariablesRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_set_variables_request_validation() {
        let variable_data = vec![create_test_variable_data()];
        let request = SetVariablesRequest::new(variable_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_variables_request_with_custom_data() {
        let variable_data = vec![create_test_variable_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetVariablesRequest::new(variable_data)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variables_request_set_methods() {
        let variable_data = vec![create_test_variable_data()];
        let new_variable_data = vec![create_test_variable_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = SetVariablesRequest::new(variable_data);

        request
            .set_set_variable_data(new_variable_data.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_set_variable_data(), &new_variable_data);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variables_request_builder_pattern() {
        let variable_data = vec![create_test_variable_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = SetVariablesRequest::new(variable_data)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variables_request_validation_empty_vector() {
        let request = SetVariablesRequest::new(vec![]);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_set_variables_request_with_multiple_data() {
        let data = vec![
            create_test_variable_data(),
            {
                let component = ComponentType::new("EVSE".to_string());
                let variable = VariableType::new("PowerLimit".to_string());
                SetVariableDataType::new(component, variable, "7400".to_string())
                    .with_attribute_type(AttributeEnumType::Actual)
            },
        ];

        let request = SetVariablesRequest::new(data.clone());

        assert_eq!(request.get_set_variable_data().len(), 2);
        assert_eq!(request.get_set_variable_data()[1].component.name, "EVSE");
        assert_eq!(request.get_set_variable_data()[1].variable.name, "PowerLimit");
        assert_eq!(request.get_set_variable_data()[1].attribute_value, "7400");
        assert!(request.validate().is_ok());
    }

    // Tests for SetVariablesResponse

    #[test]
    fn test_set_variables_response_new() {
        let variable_result = vec![create_test_variable_result()];
        let response = SetVariablesResponse::new(variable_result.clone());

        assert_eq!(response.get_set_variable_result().len(), 1);
        assert_eq!(response.get_set_variable_result()[0].component.name, "Connector");
        assert_eq!(response.get_set_variable_result()[0].variable.name, "CurrentLimit");
        assert_eq!(response.get_set_variable_result()[0].attribute_status, SetVariableStatusEnumType::Accepted);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_set_variables_response_serialization() {
        let variable_result = vec![create_test_variable_result()];
        let response = SetVariablesResponse::new(variable_result);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetVariablesResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_set_variables_response_validation() {
        let variable_result = vec![create_test_variable_result()];
        let response = SetVariablesResponse::new(variable_result);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_set_variables_response_with_custom_data() {
        let variable_result = vec![create_test_variable_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetVariablesResponse::new(variable_result)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variables_response_set_methods() {
        let variable_result = vec![create_test_variable_result()];
        let new_variable_result = vec![create_test_variable_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = SetVariablesResponse::new(variable_result);

        response
            .set_set_variable_result(new_variable_result.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_set_variable_result(), &new_variable_result);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variables_response_builder_pattern() {
        let variable_result = vec![create_test_variable_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = SetVariablesResponse::new(variable_result)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_variables_response_validation_empty_vector() {
        let response = SetVariablesResponse::new(vec![]);

        assert!(response.validate().is_err());
    }

    #[test]
    fn test_set_variables_response_with_multiple_results() {
        let results = vec![
            create_test_variable_result(),
            {
                let component = ComponentType::new("EVSE".to_string());
                let variable = VariableType::new("PowerLimit".to_string());
                SetVariableResultType::new(component, variable, SetVariableStatusEnumType::Rejected)
                    .with_attribute_type(AttributeEnumType::Actual)
                    .with_attribute_status_info(StatusInfoType::new("Rejected".to_string()))
            },
        ];

        let response = SetVariablesResponse::new(results.clone());

        assert_eq!(response.get_set_variable_result().len(), 2);
        assert_eq!(response.get_set_variable_result()[1].component.name, "EVSE");
        assert_eq!(response.get_set_variable_result()[1].variable.name, "PowerLimit");
        assert_eq!(response.get_set_variable_result()[1].attribute_status, SetVariableStatusEnumType::Rejected);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_set_variables_request_json_round_trip() {
        let variable_data = vec![create_test_variable_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetVariablesRequest::new(variable_data)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetVariablesRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_set_variables_response_json_round_trip() {
        let variable_result = vec![create_test_variable_result()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetVariablesResponse::new(variable_result)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetVariablesResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_set_variables_all_attribute_types() {
        let attribute_types = vec![
            AttributeEnumType::Actual,
            AttributeEnumType::Target,
            AttributeEnumType::MinSet,
            AttributeEnumType::MaxSet,
        ];

        for attribute_type in attribute_types {
            let component = ComponentType::new("TestComponent".to_string());
            let variable = VariableType::new("TestVariable".to_string());
            let variable_data = SetVariableDataType::new(component, variable, "test_value".to_string())
                .with_attribute_type(attribute_type.clone());

            let request = SetVariablesRequest::new(vec![variable_data]);
            assert!(request.validate().is_ok());

            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: SetVariablesRequest = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request, deserialized);
        }
    }

    #[test]
    fn test_set_variables_all_status_types() {
        let status_types = vec![
            SetVariableStatusEnumType::Accepted,
            SetVariableStatusEnumType::Rejected,
            SetVariableStatusEnumType::UnknownComponent,
            SetVariableStatusEnumType::UnknownVariable,
            SetVariableStatusEnumType::NotSupportedAttributeType,
            SetVariableStatusEnumType::RebootRequired,
        ];

        for status in status_types {
            let component = ComponentType::new("TestComponent".to_string());
            let variable = VariableType::new("TestVariable".to_string());
            let variable_result = SetVariableResultType::new(component, variable, status.clone());

            let response = SetVariablesResponse::new(vec![variable_result]);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: SetVariablesResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_set_variables_with_empty_attribute_value() {
        let component = ComponentType::new("TestComponent".to_string());
        let variable = VariableType::new("TestVariable".to_string());
        let variable_data = SetVariableDataType::new(component, variable, "".to_string());

        let request = SetVariablesRequest::new(vec![variable_data]);
        assert!(request.validate().is_ok());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetVariablesRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
        assert_eq!(deserialized.set_variable_data[0].attribute_value, "");
    }

    #[test]
    fn test_set_variables_with_custom_data_validation() {
        let variable_data = vec![create_test_variable_data()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetVariablesRequest::new(variable_data)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_variables_response_with_all_optional_fields() {
        let component = ComponentType::new("TestComponent".to_string());
        let variable = VariableType::new("TestVariable".to_string());
        let status_info = StatusInfoType::new("Success".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let variable_result = SetVariableResultType::new(component, variable, SetVariableStatusEnumType::Accepted)
            .with_attribute_type(AttributeEnumType::Actual)
            .with_attribute_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        let response = SetVariablesResponse::new(vec![variable_result])
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_set_variable_result()[0].attribute_type, Some(AttributeEnumType::Actual));
        assert_eq!(response.get_set_variable_result()[0].attribute_status_info, Some(status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_set_variables_large_attribute_value() {
        let component = ComponentType::new("TestComponent".to_string());
        let variable = VariableType::new("TestVariable".to_string());
        let large_value = "x".repeat(2500); // Max allowed length
        let variable_data = SetVariableDataType::new(component, variable, large_value.clone());

        let request = SetVariablesRequest::new(vec![variable_data]);
        assert!(request.validate().is_ok());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetVariablesRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
        assert_eq!(deserialized.set_variable_data[0].attribute_value, large_value);
    }
}