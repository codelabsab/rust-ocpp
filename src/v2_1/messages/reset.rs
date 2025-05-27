use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{ResetEnumType, ResetStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the Reset request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    #[serde(rename = "type")]
    pub type_: ResetEnumType,

    /// This contains the ID of a specific EVSE that needs to be reset, instead of the entire Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ResetRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `type_` - The type_ field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(type_: ResetEnumType) -> Self {
        Self {
            type_,
            evse_id: None,
            custom_data: None,
        }
    }

    /// Sets the type_ field.
    ///
    /// * `type_` - The type_ field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_type_(&mut self, type_: ResetEnumType) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - This contains the ID of a specific EVSE that needs to be reset, instead of the entire Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
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

    /// Gets a reference to the type_ field.
    ///
    /// # Returns
    ///
    /// The type_ field
    pub fn get_type_(&self) -> &ResetEnumType {
        &self.type_
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// This contains the ID of a specific EVSE that needs to be reset, instead of the entire Charging Station.
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - This contains the ID of a specific EVSE that needs to be reset, instead of the entire Charging Station.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
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

/// Response body for the Reset response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    pub status: ResetStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ResetResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ResetStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: ResetStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &ResetStatusEnumType {
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
    use crate::v2_1::datatypes::CustomDataType;
    use crate::v2_1::enumerations::{ResetEnumType, ResetStatusEnumType};

    #[test]
    fn test_reset_request_new() {
        let request = ResetRequest::new(ResetEnumType::Immediate);
        assert_eq!(request.type_, ResetEnumType::Immediate);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_reset_request_serialization() {
        let request = ResetRequest::new(ResetEnumType::OnIdle)
            .with_evse_id(1);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ResetRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"type\":\"OnIdle\""));
        assert!(json.contains("\"evseId\":1"));
    }

    #[test]
    fn test_reset_request_validation() {
        let mut request = ResetRequest::new(ResetEnumType::Immediate);
        request.evse_id = Some(-1); // Invalid negative value

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_reset_request_builder_pattern() {
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = ResetRequest::new(ResetEnumType::Immediate)
            .with_evse_id(2)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.type_, ResetEnumType::Immediate);
        assert_eq!(request.evse_id, Some(2));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_reset_request_setters() {
        let mut request = ResetRequest::new(ResetEnumType::OnIdle);
        let custom_data = CustomDataType::new("test_vendor".to_string());

        request.set_type_(ResetEnumType::Immediate)
               .set_evse_id(Some(3))
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.type_, ResetEnumType::Immediate);
        assert_eq!(request.evse_id, Some(3));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_reset_request_getters() {
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = ResetRequest::new(ResetEnumType::OnIdle)
            .with_evse_id(4)
            .with_custom_data(custom_data.clone());

        assert_eq!(*request.get_type_(), ResetEnumType::OnIdle);
        assert_eq!(request.get_evse_id(), Some(&4));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_reset_response_new() {
        let response = ResetResponse::new(ResetStatusEnumType::Accepted);
        assert_eq!(response.status, ResetStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_reset_response_serialization() {
        let response = ResetResponse::new(ResetStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ResetResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(json.contains("\"status\":\"Rejected\""));
    }

    #[test]
    fn test_reset_response_builder_pattern() {
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = ResetResponse::new(ResetStatusEnumType::Scheduled)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, ResetStatusEnumType::Scheduled);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_reset_response_setters() {
        let mut response = ResetResponse::new(ResetStatusEnumType::Accepted);
        let custom_data = CustomDataType::new("test_vendor".to_string());

        response.set_status(ResetStatusEnumType::Rejected)
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, ResetStatusEnumType::Rejected);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_reset_response_getters() {
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = ResetResponse::new(ResetStatusEnumType::Scheduled)
            .with_custom_data(custom_data.clone());

        assert_eq!(*response.get_status(), ResetStatusEnumType::Scheduled);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_reset_request_edge_cases() {
        // Test with zero evse_id (should be valid)
        let request = ResetRequest::new(ResetEnumType::Immediate)
            .with_evse_id(0);
        assert!(request.validate().is_ok());

        // Test without optional fields
        let minimal_request = ResetRequest::new(ResetEnumType::OnIdle);
        assert!(minimal_request.validate().is_ok());
    }

    #[test]
    fn test_reset_response_validation() {
        let response = ResetResponse::new(ResetStatusEnumType::Accepted);
        assert!(response.validate().is_ok());
    }
}