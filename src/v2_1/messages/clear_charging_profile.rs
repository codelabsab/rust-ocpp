use crate::v2_1::datatypes::{ClearChargingProfileType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::ClearChargingProfileStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClearChargingProfile request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    /// The Id of the charging profile to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub charging_profile_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub charging_profile_criteria: Option<ClearChargingProfileType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearChargingProfileRequest {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            charging_profile_id: None,
            charging_profile_criteria: None,
            custom_data: None,
        }
    }

    /// Sets the charging_profile_id field.
    ///
    /// * `charging_profile_id` - The Id of the charging profile to clear.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_profile_id(&mut self, charging_profile_id: Option<i32>) -> &mut Self {
        self.charging_profile_id = charging_profile_id;
        self
    }

    /// Sets the charging_profile_criteria field.
    ///
    /// * `charging_profile_criteria` - The charging_profile_criteria field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_profile_criteria(&mut self, charging_profile_criteria: Option<ClearChargingProfileType>) -> &mut Self {
        self.charging_profile_criteria = charging_profile_criteria;
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

    /// Gets a reference to the charging_profile_id field.
    ///
    /// # Returns
    ///
    /// The Id of the charging profile to clear.
    pub fn get_charging_profile_id(&self) -> Option<&i32> {
        self.charging_profile_id.as_ref()
    }

    /// Gets a reference to the charging_profile_criteria field.
    ///
    /// # Returns
    ///
    /// The charging_profile_criteria field
    pub fn get_charging_profile_criteria(&self) -> Option<&ClearChargingProfileType> {
        self.charging_profile_criteria.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the charging_profile_id field and returns self for builder pattern.
    ///
    /// * `charging_profile_id` - The Id of the charging profile to clear.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_charging_profile_id(mut self, charging_profile_id: i32) -> Self {
        self.charging_profile_id = Some(charging_profile_id);
        self
    }

    /// Sets the charging_profile_criteria field and returns self for builder pattern.
    ///
    /// * `charging_profile_criteria` - The charging_profile_criteria field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_charging_profile_criteria(mut self, charging_profile_criteria: ClearChargingProfileType) -> Self {
        self.charging_profile_criteria = Some(charging_profile_criteria);
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

/// Response body for the ClearChargingProfile response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    pub status: ClearChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearChargingProfileResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ClearChargingProfileStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: ClearChargingProfileStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &ClearChargingProfileStatusEnumType {
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
    use serde_json;
    use validator::Validate;

    #[test]
    fn test_clear_charging_profile_request_new() {
        let request = ClearChargingProfileRequest::new();
        assert_eq!(request.get_charging_profile_id(), None);
        assert_eq!(request.get_charging_profile_criteria(), None);
        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_charging_profile_request_with_id() {
        let request = ClearChargingProfileRequest::new()
            .with_charging_profile_id(123);
        assert_eq!(request.get_charging_profile_id(), Some(&123));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_charging_profile_request_validation_invalid_id() {
        let mut request = ClearChargingProfileRequest::new();
        request.set_charging_profile_id(Some(-1)); // Invalid: must be >= 0
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_clear_charging_profile_request_serialization() {
        let request = ClearChargingProfileRequest::new();
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearChargingProfileRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_clear_charging_profile_response_new() {
        let status = ClearChargingProfileStatusEnumType::Accepted;
        let response = ClearChargingProfileResponse::new(status.clone());
        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_charging_profile_response_all_status_values() {
        let status_values = vec![
            ClearChargingProfileStatusEnumType::Accepted,
            ClearChargingProfileStatusEnumType::Unknown,
        ];

        for status in status_values {
            let response = ClearChargingProfileResponse::new(status.clone());
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_clear_charging_profile_response_serialization() {
        let status = ClearChargingProfileStatusEnumType::Accepted;
        let response = ClearChargingProfileResponse::new(status);
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearChargingProfileResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_clear_charging_profile_request_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = ClearChargingProfileRequest::new()
            .with_custom_data(custom_data.clone());
        assert_eq!(request.get_custom_data(), Some(&custom_data));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_clear_charging_profile_response_with_status_info() {
        let status = ClearChargingProfileStatusEnumType::Unknown;
        let status_info = StatusInfoType::new("NotFound".to_string());
        let response = ClearChargingProfileResponse::new(status)
            .with_status_info(status_info.clone());
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_clear_charging_profile_request_json_round_trip() {
        let request = ClearChargingProfileRequest::new()
            .with_charging_profile_id(456)
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearChargingProfileRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_clear_charging_profile_response_json_round_trip() {
        let response = ClearChargingProfileResponse::new(ClearChargingProfileStatusEnumType::Unknown)
            .with_status_info(StatusInfoType::new("ProfileNotFound".to_string()))
            .with_custom_data(CustomDataType::new("TestVendor".to_string()));

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearChargingProfileResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }
}