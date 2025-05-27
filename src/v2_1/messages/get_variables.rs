use crate::v2_1::datatypes::{CustomDataType, GetVariableDataType, GetVariableResultType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub get_variable_data: Vec<GetVariableDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetVariablesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `get_variable_data` - The get_variable_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(get_variable_data: Vec<GetVariableDataType>) -> Self {
        Self {
            get_variable_data,
            custom_data: None,
        }
    }

    /// Sets the get_variable_data field.
    ///
    /// * `get_variable_data` - The get_variable_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_get_variable_data(&mut self, get_variable_data: Vec<GetVariableDataType>) -> &mut Self {
        self.get_variable_data = get_variable_data;
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

    /// Gets a reference to the get_variable_data field.
    ///
    /// # Returns
    ///
    /// The get_variable_data field
    pub fn get_get_variable_data(&self) -> &Vec<GetVariableDataType> {
        &self.get_variable_data
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

/// Response body for the GetVariables response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub get_variable_result: Vec<GetVariableResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetVariablesResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `get_variable_result` - The get_variable_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(get_variable_result: Vec<GetVariableResultType>) -> Self {
        Self {
            get_variable_result,
            custom_data: None,
        }
    }

    /// Sets the get_variable_result field.
    ///
    /// * `get_variable_result` - The get_variable_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_get_variable_result(&mut self, get_variable_result: Vec<GetVariableResultType>) -> &mut Self {
        self.get_variable_result = get_variable_result;
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

    /// Gets a reference to the get_variable_result field.
    ///
    /// # Returns
    ///
    /// The get_variable_result field
    pub fn get_get_variable_result(&self) -> &Vec<GetVariableResultType> {
        &self.get_variable_result
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
    use crate::v2_1::datatypes::{ComponentType, VariableType, StatusInfoType};
    use crate::v2_1::enumerations::{AttributeEnumType, GetVariableStatusEnumType};
    use serde_json;

    // Helper function to create test GetVariableDataType
    fn create_test_get_variable_data() -> GetVariableDataType {
        let component = ComponentType::new("Connector".to_string())
            .with_instance("Main".to_string());
        let variable = VariableType::new("CurrentLimit".to_string());
        GetVariableDataType::new(component, variable)
    }

    // Helper function to create test GetVariableResultType
    fn create_test_get_variable_result() -> GetVariableResultType {
        let component = ComponentType::new("Connector".to_string());
        let variable = VariableType::new("CurrentLimit".to_string());
        GetVariableResultType::new(component, variable, GetVariableStatusEnumType::Accepted)
            .with_attribute_value("32".to_string())
    }

    // Tests for GetVariablesRequest
    
    #[test]
    fn test_get_variables_request_new() {
        let get_variable_data = vec![create_test_get_variable_data()];
        let request = GetVariablesRequest::new(get_variable_data.clone());
        
        assert_eq!(request.get_variable_data.len(), 1);
        assert_eq!(request.get_variable_data[0].component.name, "Connector");
        assert_eq!(request.get_variable_data[0].variable.name, "CurrentLimit");
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_variables_request_with_custom_data() {
        let get_variable_data = vec![create_test_get_variable_data()];
        let request = GetVariablesRequest::new(get_variable_data)
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));
        
        assert!(request.custom_data.is_some());
        assert_eq!(request.custom_data.as_ref().unwrap().vendor_id, "TestVendor");
    }

    #[test]
    fn test_get_variables_request_setters() {
        let mut request = GetVariablesRequest::new(vec![create_test_get_variable_data()]);
        
        let new_data = vec![
            create_test_get_variable_data(),
            create_test_get_variable_data()
        ];
        request.set_get_variable_data(new_data);
        request.set_custom_data(Some(CustomDataType::new("NewVendor".to_string())));
        
        assert_eq!(request.get_variable_data.len(), 2);
        assert_eq!(request.custom_data.as_ref().unwrap().vendor_id, "NewVendor");
    }

    #[test]
    fn test_get_variables_request_getters() {
        let get_variable_data = vec![create_test_get_variable_data()];
        let request = GetVariablesRequest::new(get_variable_data.clone())
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));
        
        assert_eq!(request.get_get_variable_data().len(), 1);
        assert_eq!(request.get_custom_data().unwrap().vendor_id, "TestVendor");
    }

    #[test]
    fn test_get_variables_request_serialization() {
        let get_variable_data = vec![create_test_get_variable_data()];
        let request = GetVariablesRequest::new(get_variable_data);
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetVariablesRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_variables_request_deserialization() {
        let json = r#"{
            "getVariableData": [{
                "component": {
                    "name": "Connector",
                    "instance": "Main"
                },
                "variable": {
                    "name": "CurrentLimit"
                }
            }]
        }"#;
        
        let request: GetVariablesRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.get_variable_data.len(), 1);
        assert_eq!(request.get_variable_data[0].component.name, "Connector");
        assert_eq!(request.get_variable_data[0].component.instance, Some("Main".to_string()));
        assert_eq!(request.get_variable_data[0].variable.name, "CurrentLimit");
    }

    #[test]
    fn test_get_variables_request_validation_empty_vector() {
        let request = GetVariablesRequest::new(vec![]);
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_variables_request_with_multiple_variables() {
        let data = vec![
            GetVariableDataType::new(
                ComponentType::new("Connector".to_string()),
                VariableType::new("CurrentLimit".to_string())
            ),
            GetVariableDataType::new(
                ComponentType::new("Meter".to_string()),
                VariableType::new("Energy".to_string())
            ).with_attribute_type(AttributeEnumType::Actual),
            GetVariableDataType::new(
                ComponentType::new("Controller".to_string()),
                VariableType::new("Enabled".to_string())
            ).with_attribute_type(AttributeEnumType::Target)
        ];
        
        let request = GetVariablesRequest::new(data);
        
        assert_eq!(request.get_variable_data.len(), 3);
        assert_eq!(request.get_variable_data[0].component.name, "Connector");
        assert_eq!(request.get_variable_data[1].component.name, "Meter");
        assert_eq!(request.get_variable_data[2].component.name, "Controller");
        assert_eq!(request.get_variable_data[1].attribute_type, Some(AttributeEnumType::Actual));
        assert_eq!(request.get_variable_data[2].attribute_type, Some(AttributeEnumType::Target));
    }

    // Tests for GetVariablesResponse
    
    #[test]
    fn test_get_variables_response_new() {
        let get_variable_result = vec![create_test_get_variable_result()];
        let response = GetVariablesResponse::new(get_variable_result.clone());
        
        assert_eq!(response.get_variable_result.len(), 1);
        assert_eq!(response.get_variable_result[0].component.name, "Connector");
        assert_eq!(response.get_variable_result[0].variable.name, "CurrentLimit");
        assert_eq!(response.get_variable_result[0].attribute_status, GetVariableStatusEnumType::Accepted);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_variables_response_with_custom_data() {
        let get_variable_result = vec![create_test_get_variable_result()];
        let response = GetVariablesResponse::new(get_variable_result)
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));
        
        assert!(response.custom_data.is_some());
        assert_eq!(response.custom_data.as_ref().unwrap().vendor_id, "TestVendor");
    }

    #[test]
    fn test_get_variables_response_setters() {
        let mut response = GetVariablesResponse::new(vec![create_test_get_variable_result()]);
        
        let new_results = vec![
            create_test_get_variable_result(),
            create_test_get_variable_result()
        ];
        response.set_get_variable_result(new_results);
        response.set_custom_data(Some(CustomDataType::new("NewVendor".to_string())));
        
        assert_eq!(response.get_variable_result.len(), 2);
        assert_eq!(response.custom_data.as_ref().unwrap().vendor_id, "NewVendor");
    }

    #[test]
    fn test_get_variables_response_getters() {
        let get_variable_result = vec![create_test_get_variable_result()];
        let response = GetVariablesResponse::new(get_variable_result.clone())
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));
        
        assert_eq!(response.get_get_variable_result().len(), 1);
        assert_eq!(response.get_custom_data().unwrap().vendor_id, "TestVendor");
    }

    #[test]
    fn test_get_variables_response_serialization() {
        let get_variable_result = vec![create_test_get_variable_result()];
        let response = GetVariablesResponse::new(get_variable_result);
        
        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetVariablesResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_variables_response_deserialization() {
        let json = r#"{
            "getVariableResult": [{
                "component": {
                    "name": "Connector"
                },
                "variable": {
                    "name": "CurrentLimit"
                },
                "attributeStatus": "Accepted",
                "attributeValue": "32"
            }]
        }"#;
        
        let response: GetVariablesResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.get_variable_result.len(), 1);
        assert_eq!(response.get_variable_result[0].component.name, "Connector");
        assert_eq!(response.get_variable_result[0].variable.name, "CurrentLimit");
        assert_eq!(response.get_variable_result[0].attribute_status, GetVariableStatusEnumType::Accepted);
        assert_eq!(response.get_variable_result[0].attribute_value, Some("32".to_string()));
    }

    #[test]
    fn test_get_variables_response_validation_empty_vector() {
        let response = GetVariablesResponse::new(vec![]);
        
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_get_variables_response_with_multiple_results() {
        let results = vec![
            GetVariableResultType::new(
                ComponentType::new("Connector".to_string()),
                VariableType::new("CurrentLimit".to_string()),
                GetVariableStatusEnumType::Accepted
            ).with_attribute_value("32".to_string()),
            GetVariableResultType::new(
                ComponentType::new("Meter".to_string()),
                VariableType::new("Energy".to_string()),
                GetVariableStatusEnumType::Rejected
            ),
            GetVariableResultType::new(
                ComponentType::new("Controller".to_string()),
                VariableType::new("Enabled".to_string()),
                GetVariableStatusEnumType::UnknownVariable
            )
        ];
        
        let response = GetVariablesResponse::new(results);
        
        assert_eq!(response.get_variable_result.len(), 3);
        assert_eq!(response.get_variable_result[0].attribute_status, GetVariableStatusEnumType::Accepted);
        assert_eq!(response.get_variable_result[0].attribute_value, Some("32".to_string()));
        assert_eq!(response.get_variable_result[1].attribute_status, GetVariableStatusEnumType::Rejected);
        assert_eq!(response.get_variable_result[1].attribute_value, None);
        assert_eq!(response.get_variable_result[2].attribute_status, GetVariableStatusEnumType::UnknownVariable);
    }

    #[test]
    fn test_get_variables_request_json_round_trip_with_all_fields() {
        let data = vec![
            GetVariableDataType::new(
                ComponentType::new("Connector".to_string()).with_instance("1".to_string()),
                VariableType::new("CurrentLimit".to_string())
            ).with_attribute_type(AttributeEnumType::Actual)
                .with_custom_data(CustomDataType::new("DataVendor".to_string()))
        ];
        
        let request = GetVariablesRequest::new(data)
            .with_custom_data(CustomDataType::new("RequestVendor".to_string()));
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetVariablesRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request, parsed);
        assert_eq!(parsed.get_variable_data[0].attribute_type, Some(AttributeEnumType::Actual));
        assert_eq!(parsed.get_variable_data[0].custom_data.as_ref().unwrap().vendor_id, "DataVendor");
        assert_eq!(parsed.custom_data.as_ref().unwrap().vendor_id, "RequestVendor");
    }

    #[test]
    fn test_get_variables_response_json_round_trip_with_all_fields() {
        let results = vec![
            GetVariableResultType::new(
                ComponentType::new("Connector".to_string()),
                VariableType::new("CurrentLimit".to_string()),
                GetVariableStatusEnumType::Accepted
            ).with_attribute_type(AttributeEnumType::Actual)
                .with_attribute_value("32".to_string())
                .with_attribute_status_info(StatusInfoType::new("Success".to_string())
                    .with_additional_info("Variable retrieved successfully".to_string()))
                .with_custom_data(CustomDataType::new("ResultVendor".to_string()))
        ];
        
        let response = GetVariablesResponse::new(results)
            .with_custom_data(CustomDataType::new("ResponseVendor".to_string()));
        
        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetVariablesResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(response, parsed);
        assert_eq!(parsed.get_variable_result[0].attribute_type, Some(AttributeEnumType::Actual));
        assert_eq!(parsed.get_variable_result[0].attribute_value, Some("32".to_string()));
        assert_eq!(parsed.get_variable_result[0].attribute_status_info.as_ref().unwrap().reason_code, "Success");
        assert_eq!(parsed.get_variable_result[0].custom_data.as_ref().unwrap().vendor_id, "ResultVendor");
        assert_eq!(parsed.custom_data.as_ref().unwrap().vendor_id, "ResponseVendor");
    }
}
