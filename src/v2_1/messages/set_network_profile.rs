use crate::v2_1::datatypes::{CustomDataType, NetworkConnectionProfileType, StatusInfoType};
use crate::v2_1::enumerations::SetNetworkProfileStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetNetworkProfile request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileRequest {
    /// Slot in which the configuration should be stored.
    pub configuration_slot: i32,

    #[validate(nested)]
    pub connection_data: NetworkConnectionProfileType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetNetworkProfileRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `configuration_slot` - Slot in which the configuration should be stored.
    /// * `connection_data` - The connection_data field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(configuration_slot: i32, connection_data: NetworkConnectionProfileType) -> Self {
        Self {
            configuration_slot,
            connection_data,
            custom_data: None,
        }
    }

    /// Sets the configuration_slot field.
    ///
    /// * `configuration_slot` - Slot in which the configuration should be stored.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_configuration_slot(&mut self, configuration_slot: i32) -> &mut Self {
        self.configuration_slot = configuration_slot;
        self
    }

    /// Sets the connection_data field.
    ///
    /// * `connection_data` - The connection_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_connection_data(&mut self, connection_data: NetworkConnectionProfileType) -> &mut Self {
        self.connection_data = connection_data;
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

    /// Gets a reference to the configuration_slot field.
    ///
    /// # Returns
    ///
    /// Slot in which the configuration should be stored.
    pub fn get_configuration_slot(&self) -> &i32 {
        &self.configuration_slot
    }

    /// Gets a reference to the connection_data field.
    ///
    /// # Returns
    ///
    /// The connection_data field
    pub fn get_connection_data(&self) -> &NetworkConnectionProfileType {
        &self.connection_data
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

/// Response body for the SetNetworkProfile response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileResponse {
    pub status: SetNetworkProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetNetworkProfileResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: SetNetworkProfileStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: SetNetworkProfileStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &SetNetworkProfileStatusEnumType {
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
    use crate::v2_1::enumerations::SetNetworkProfileStatusEnumType;
    use serde_json;
    use validator::Validate;

    fn create_test_network_profile() -> NetworkConnectionProfileType {
        NetworkConnectionProfileType::new(
            "Wired0".to_string(),
            "JSON".to_string(),
            "OCPP20".to_string(),
            "wss://example.com/ocpp".to_string(),
            30,
            1,
        )
    }

    #[test]
    fn test_set_network_profile_request_new() {
        let configuration_slot = 1;
        let connection_data = create_test_network_profile();
        let request = SetNetworkProfileRequest::new(configuration_slot, connection_data.clone());

        assert_eq!(request.get_configuration_slot(), &configuration_slot);
        assert_eq!(request.get_connection_data(), &connection_data);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_set_network_profile_request_serialization() {
        let connection_data = create_test_network_profile();
        let request = SetNetworkProfileRequest::new(1, connection_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetNetworkProfileRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_set_network_profile_request_validation() {
        let connection_data = create_test_network_profile();
        let request = SetNetworkProfileRequest::new(1, connection_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_network_profile_request_with_custom_data() {
        let connection_data = create_test_network_profile();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetNetworkProfileRequest::new(1, connection_data)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_network_profile_request_set_methods() {
        let connection_data = create_test_network_profile();
        let new_connection_data = create_test_network_profile();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = SetNetworkProfileRequest::new(1, connection_data);

        request
            .set_configuration_slot(2)
            .set_connection_data(new_connection_data.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_configuration_slot(), &2);
        assert_eq!(request.get_connection_data(), &new_connection_data);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_network_profile_request_builder_pattern() {
        let connection_data = create_test_network_profile();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = SetNetworkProfileRequest::new(1, connection_data)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_network_profile_response_new() {
        let status = SetNetworkProfileStatusEnumType::Accepted;
        let response = SetNetworkProfileResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_set_network_profile_response_serialization() {
        let response = SetNetworkProfileResponse::new(SetNetworkProfileStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetNetworkProfileResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_set_network_profile_response_validation() {
        let response = SetNetworkProfileResponse::new(SetNetworkProfileStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_set_network_profile_response_with_status_info() {
        let status_info = StatusInfoType::new("Info".to_string());
        let response = SetNetworkProfileResponse::new(SetNetworkProfileStatusEnumType::Failed)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_set_network_profile_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetNetworkProfileResponse::new(SetNetworkProfileStatusEnumType::Rejected)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_network_profile_response_set_methods() {
        let status_info = StatusInfoType::new("Failed".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = SetNetworkProfileResponse::new(SetNetworkProfileStatusEnumType::Accepted);

        response
            .set_status(SetNetworkProfileStatusEnumType::Failed)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &SetNetworkProfileStatusEnumType::Failed);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_network_profile_response_builder_pattern() {
        let status_info = StatusInfoType::new("Completed".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = SetNetworkProfileResponse::new(SetNetworkProfileStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_network_profile_request_json_round_trip() {
        let connection_data = create_test_network_profile();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetNetworkProfileRequest::new(1, connection_data)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetNetworkProfileRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_set_network_profile_response_json_round_trip() {
        let status_info = StatusInfoType::new("Applied".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetNetworkProfileResponse::new(SetNetworkProfileStatusEnumType::Accepted)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetNetworkProfileResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_set_network_profile_request_with_negative_slot() {
        let connection_data = create_test_network_profile();
        let request = SetNetworkProfileRequest::new(-1, connection_data);

        assert_eq!(request.get_configuration_slot(), &-1);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_network_profile_response_all_status_variants() {
        let statuses = vec![
            SetNetworkProfileStatusEnumType::Accepted,
            SetNetworkProfileStatusEnumType::Rejected,
            SetNetworkProfileStatusEnumType::Failed,
        ];

        for status in statuses {
            let response = SetNetworkProfileResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: SetNetworkProfileResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_set_network_profile_request_with_custom_data_validation() {
        let connection_data = create_test_network_profile();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = SetNetworkProfileRequest::new(1, connection_data)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_network_profile_response_with_all_optional_fields() {
        let status_info = StatusInfoType::new("Details".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = SetNetworkProfileResponse::new(SetNetworkProfileStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &SetNetworkProfileStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }
}