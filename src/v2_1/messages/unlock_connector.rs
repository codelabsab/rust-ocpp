use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::UnlockStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the UnlockConnector request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    /// This contains the identifier of the EVSE for which a connector needs to be unlocked.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// This contains the identifier of the connector that needs to be unlocked.
    #[validate(range(min = 0))]
    pub connector_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UnlockConnectorRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - This contains the identifier of the EVSE for which a connector needs to be unlocked.
    /// * `connector_id` - This contains the identifier of the connector that needs to be unlocked.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32, connector_id: i32) -> Self {
        Self {
            evse_id,
            connector_id,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - This contains the identifier of the EVSE for which a connector needs to be unlocked.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the connector_id field.
    ///
    /// * `connector_id` - This contains the identifier of the connector that needs to be unlocked.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_connector_id(&mut self, connector_id: i32) -> &mut Self {
        self.connector_id = connector_id;
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

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// This contains the identifier of the EVSE for which a connector needs to be unlocked.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the connector_id field.
    ///
    /// # Returns
    ///
    /// This contains the identifier of the connector that needs to be unlocked.
    pub fn get_connector_id(&self) -> &i32 {
        &self.connector_id
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

/// Response body for the UnlockConnector response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorResponse {
    pub status: UnlockStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UnlockConnectorResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: UnlockStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: UnlockStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &UnlockStatusEnumType {
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
    use crate::v2_1::enumerations::UnlockStatusEnumType;
    use serde_json;
    use validator::Validate;

    // Tests for UnlockConnectorRequest

    #[test]
    fn test_unlock_connector_request_new() {
        let evse_id = 1;
        let connector_id = 2;
        let request = UnlockConnectorRequest::new(evse_id, connector_id);

        assert_eq!(request.get_evse_id(), &evse_id);
        assert_eq!(request.get_connector_id(), &connector_id);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_unlock_connector_request_serialization() {
        let request = UnlockConnectorRequest::new(1, 1);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UnlockConnectorRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_unlock_connector_request_validation() {
        let request = UnlockConnectorRequest::new(1, 1);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_unlock_connector_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UnlockConnectorRequest::new(2, 3)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unlock_connector_request_set_methods() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = UnlockConnectorRequest::new(1, 1);

        request
            .set_evse_id(5)
            .set_connector_id(10)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_evse_id(), &5);
        assert_eq!(request.get_connector_id(), &10);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unlock_connector_request_builder_pattern() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = UnlockConnectorRequest::new(3, 2)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unlock_connector_request_negative_evse_id_validation() {
        let mut request = UnlockConnectorRequest::new(1, 1);
        request.set_evse_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_unlock_connector_request_negative_connector_id_validation() {
        let mut request = UnlockConnectorRequest::new(1, 1);
        request.set_connector_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_unlock_connector_request_zero_ids_validation() {
        let request = UnlockConnectorRequest::new(0, 0);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_unlock_connector_request_with_large_ids() {
        let request = UnlockConnectorRequest::new(999999, 888888);

        assert_eq!(request.get_evse_id(), &999999);
        assert_eq!(request.get_connector_id(), &888888);
        assert!(request.validate().is_ok());
    }

    // Tests for UnlockConnectorResponse

    #[test]
    fn test_unlock_connector_response_new() {
        let status = UnlockStatusEnumType::Unlocked;
        let response = UnlockConnectorResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_unlock_connector_response_serialization() {
        let response = UnlockConnectorResponse::new(UnlockStatusEnumType::UnlockFailed);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UnlockConnectorResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_unlock_connector_response_validation() {
        let response = UnlockConnectorResponse::new(UnlockStatusEnumType::Unlocked);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_unlock_connector_response_with_status_info() {
        let status_info = StatusInfoType::new("Success".to_string());
        let response = UnlockConnectorResponse::new(UnlockStatusEnumType::Unlocked)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_unlock_connector_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UnlockConnectorResponse::new(UnlockStatusEnumType::OngoingAuthorizedTransaction)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unlock_connector_response_set_methods() {
        let status_info = StatusInfoType::new("Failed".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = UnlockConnectorResponse::new(UnlockStatusEnumType::Unlocked);

        response
            .set_status(UnlockStatusEnumType::UnknownConnector)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &UnlockStatusEnumType::UnknownConnector);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unlock_connector_response_builder_pattern() {
        let status_info = StatusInfoType::new("Info".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UnlockConnectorResponse::new(UnlockStatusEnumType::Unlocked)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unlock_connector_response_all_status_types() {
        let status_types = vec![
            UnlockStatusEnumType::Unlocked,
            UnlockStatusEnumType::UnlockFailed,
            UnlockStatusEnumType::OngoingAuthorizedTransaction,
            UnlockStatusEnumType::UnknownConnector,
        ];

        for status in status_types {
            let response = UnlockConnectorResponse::new(status.clone());
            
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: UnlockConnectorResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_unlock_connector_request_json_round_trip() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UnlockConnectorRequest::new(2, 1)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UnlockConnectorRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_unlock_connector_response_json_round_trip() {
        let status_info = StatusInfoType::new("Details".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UnlockConnectorResponse::new(UnlockStatusEnumType::UnlockFailed)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UnlockConnectorResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_unlock_connector_response_with_all_optional_fields() {
        let status_info = StatusInfoType::new("Complete".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UnlockConnectorResponse::new(UnlockStatusEnumType::Unlocked)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &UnlockStatusEnumType::Unlocked);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_unlock_connector_request_with_custom_data_validation() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UnlockConnectorRequest::new(1, 1)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }
}