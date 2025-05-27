use crate::v2_1::datatypes::{AuthorizationData, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{SendLocalListStatusEnumType, UpdateEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SendLocalList request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,

    /// In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    pub version_number: i32,

    pub update_type: UpdateEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SendLocalListRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `version_number` - In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    /// * `update_type` - The update_type field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(version_number: i32, update_type: UpdateEnumType) -> Self {
        Self {
            local_authorization_list: None,
            version_number,
            update_type,
            custom_data: None,
        }
    }

    /// Sets the local_authorization_list field.
    ///
    /// * `local_authorization_list` - The local_authorization_list field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_local_authorization_list(&mut self, local_authorization_list: Option<Vec<AuthorizationData>>) -> &mut Self {
        self.local_authorization_list = local_authorization_list;
        self
    }

    /// Sets the version_number field.
    ///
    /// * `version_number` - In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_version_number(&mut self, version_number: i32) -> &mut Self {
        self.version_number = version_number;
        self
    }

    /// Sets the update_type field.
    ///
    /// * `update_type` - The update_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_update_type(&mut self, update_type: UpdateEnumType) -> &mut Self {
        self.update_type = update_type;
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

    /// Gets a reference to the local_authorization_list field.
    ///
    /// # Returns
    ///
    /// The local_authorization_list field
    pub fn get_local_authorization_list(&self) -> Option<&Vec<AuthorizationData>> {
        self.local_authorization_list.as_ref()
    }

    /// Gets a reference to the version_number field.
    ///
    /// # Returns
    ///
    /// In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    pub fn get_version_number(&self) -> &i32 {
        &self.version_number
    }

    /// Gets a reference to the update_type field.
    ///
    /// # Returns
    ///
    /// The update_type field
    pub fn get_update_type(&self) -> &UpdateEnumType {
        &self.update_type
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the local_authorization_list field and returns self for builder pattern.
    ///
    /// * `local_authorization_list` - The local_authorization_list field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_local_authorization_list(mut self, local_authorization_list: Vec<AuthorizationData>) -> Self {
        self.local_authorization_list = Some(local_authorization_list);
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

/// Response body for the SendLocalList response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    pub status: SendLocalListStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SendLocalListResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: SendLocalListStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: SendLocalListStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &SendLocalListStatusEnumType {
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
    use crate::v2_1::datatypes::{AuthorizationData, CustomDataType, IdTokenType, IdTokenInfoType, StatusInfoType};
    use crate::v2_1::enumerations::{AuthorizationStatusEnumType, SendLocalListStatusEnumType, UpdateEnumType};

    #[test]
    fn test_send_local_list_request_new() {
        let request = SendLocalListRequest::new(1, UpdateEnumType::Full);
        assert_eq!(request.local_authorization_list, None);
        assert_eq!(request.version_number, 1);
        assert_eq!(request.update_type, UpdateEnumType::Full);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_send_local_list_request_serialization() {
        let id_token = IdTokenType::new("test_token".to_string(), "Central".to_string());
        let id_token_info = IdTokenInfoType::new(AuthorizationStatusEnumType::Accepted);
        let auth_data = AuthorizationData::new(id_token, id_token_info);
        let request = SendLocalListRequest::new(2, UpdateEnumType::Differential)
            .with_local_authorization_list(vec![auth_data]);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SendLocalListRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"versionNumber\":2"));
        assert!(json.contains("\"updateType\":\"Differential\""));
    }

    #[test]
    fn test_send_local_list_request_validation() {
        // Test valid request
        let valid_request = SendLocalListRequest::new(1, UpdateEnumType::Full);
        assert!(valid_request.validate().is_ok());

        // Test invalid request with empty authorization list
        let mut invalid_request = SendLocalListRequest::new(1, UpdateEnumType::Full);
        invalid_request.local_authorization_list = Some(vec![]);
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_send_local_list_request_builder_pattern() {
        let id_token = IdTokenType::new("test_token".to_string(), "Central".to_string());
        let id_token_info = IdTokenInfoType::new(AuthorizationStatusEnumType::Accepted);
        let auth_data = AuthorizationData::new(id_token, id_token_info);
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SendLocalListRequest::new(3, UpdateEnumType::Full)
            .with_local_authorization_list(vec![auth_data.clone()])
            .with_custom_data(custom_data.clone());

        assert_eq!(request.version_number, 3);
        assert_eq!(request.update_type, UpdateEnumType::Full);
        assert_eq!(request.local_authorization_list, Some(vec![auth_data]));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_send_local_list_request_setters() {
        let mut request = SendLocalListRequest::new(1, UpdateEnumType::Full);
        let id_token = IdTokenType::new("test_token".to_string(), "Central".to_string());
        let id_token_info = IdTokenInfoType::new(AuthorizationStatusEnumType::Accepted);
        let auth_data = AuthorizationData::new(id_token, id_token_info);
        let custom_data = CustomDataType::new("test_vendor".to_string());

        request.set_version_number(4)
               .set_update_type(UpdateEnumType::Differential)
               .set_local_authorization_list(Some(vec![auth_data.clone()]))
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.version_number, 4);
        assert_eq!(request.update_type, UpdateEnumType::Differential);
        assert_eq!(request.local_authorization_list, Some(vec![auth_data]));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_send_local_list_request_getters() {
        let id_token = IdTokenType::new("test_token".to_string(), "Central".to_string());
        let id_token_info = IdTokenInfoType::new(AuthorizationStatusEnumType::Accepted);
        let auth_data = AuthorizationData::new(id_token, id_token_info);
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SendLocalListRequest::new(5, UpdateEnumType::Full)
            .with_local_authorization_list(vec![auth_data.clone()])
            .with_custom_data(custom_data.clone());

        assert_eq!(*request.get_version_number(), 5);
        assert_eq!(*request.get_update_type(), UpdateEnumType::Full);
        assert_eq!(request.get_local_authorization_list(), Some(&vec![auth_data]));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_send_local_list_response_new() {
        let response = SendLocalListResponse::new(SendLocalListStatusEnumType::Accepted);
        assert_eq!(response.status, SendLocalListStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_send_local_list_response_serialization() {
        let response = SendLocalListResponse::new(SendLocalListStatusEnumType::Failed);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SendLocalListResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(json.contains("\"status\":\"Failed\""));
    }

    #[test]
    fn test_send_local_list_response_builder_pattern() {
        let status_info = StatusInfoType::new("Test reason".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SendLocalListResponse::new(SendLocalListStatusEnumType::VersionMismatch)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, SendLocalListStatusEnumType::VersionMismatch);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_send_local_list_response_setters() {
        let mut response = SendLocalListResponse::new(SendLocalListStatusEnumType::Accepted);
        let status_info = StatusInfoType::new("Updated reason".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());

        response.set_status(SendLocalListStatusEnumType::Failed)
                .set_status_info(Some(status_info.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, SendLocalListStatusEnumType::Failed);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_send_local_list_response_getters() {
        let status_info = StatusInfoType::new("Test reason".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SendLocalListResponse::new(SendLocalListStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*response.get_status(), SendLocalListStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_send_local_list_edge_cases() {
        // Test with multiple authorization data entries
        let id_token1 = IdTokenType::new("token1".to_string(), "Central".to_string());
        let id_token2 = IdTokenType::new("token2".to_string(), "Local".to_string());
        let id_token_info1 = IdTokenInfoType::new(AuthorizationStatusEnumType::Accepted);
        let id_token_info2 = IdTokenInfoType::new(AuthorizationStatusEnumType::Accepted);
        let auth_data1 = AuthorizationData::new(id_token1, id_token_info1);
        let auth_data2 = AuthorizationData::new(id_token2, id_token_info2);

        let request = SendLocalListRequest::new(10, UpdateEnumType::Full)
            .with_local_authorization_list(vec![auth_data1, auth_data2]);

        assert!(request.validate().is_ok());
        assert_eq!(request.local_authorization_list.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_send_local_list_response_validation() {
        let response = SendLocalListResponse::new(SendLocalListStatusEnumType::Accepted);
        assert!(response.validate().is_ok());
    }
}