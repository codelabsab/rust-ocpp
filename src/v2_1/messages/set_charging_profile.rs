use crate::v2_1::datatypes::{ChargingProfileType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::ChargingProfileStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetChargingProfile request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    /// For TxDefaultProfile an evseId=0 applies the profile to each individual evse. For ChargingStationMaxProfile and ChargingStationExternalConstraints an evseId=0 contains an overal limit for the whole Charging Station.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[validate(nested)]
    pub charging_profile: ChargingProfileType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetChargingProfileRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - For TxDefaultProfile an evseId=0 applies the profile to each individual evse. For ChargingStationMaxProfile and ChargingStationExternalConstraints an evseId=0 contains an overal limit for the whole Charging Station.
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32, charging_profile: ChargingProfileType) -> Self {
        Self {
            evse_id,
            charging_profile,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - For TxDefaultProfile an evseId=0 applies the profile to each individual evse. For ChargingStationMaxProfile and ChargingStationExternalConstraints an evseId=0 contains an overal limit for the whole Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the charging_profile field.
    ///
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_profile(&mut self, charging_profile: ChargingProfileType) -> &mut Self {
        self.charging_profile = charging_profile;
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
    /// For TxDefaultProfile an evseId=0 applies the profile to each individual evse. For ChargingStationMaxProfile and ChargingStationExternalConstraints an evseId=0 contains an overal limit for the whole Charging Station.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the charging_profile field.
    ///
    /// # Returns
    ///
    /// The charging_profile field
    pub fn get_charging_profile(&self) -> &ChargingProfileType {
        &self.charging_profile
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

/// Response body for the SetChargingProfile response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse {
    pub status: ChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetChargingProfileResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ChargingProfileStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: ChargingProfileStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &ChargingProfileStatusEnumType {
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
    use crate::v2_1::datatypes::{ChargingProfileType, ChargingScheduleType, ChargingSchedulePeriodType, CustomDataType, StatusInfoType};
    use crate::v2_1::enumerations::{ChargingProfileKindEnumType, ChargingProfilePurposeEnumType, ChargingProfileStatusEnumType, ChargingRateUnitEnumType};
    use rust_decimal_macros::dec;

    fn create_test_charging_profile() -> ChargingProfileType {
        let period = ChargingSchedulePeriodType::new(0, dec!(16.0));
        let charging_schedule = ChargingScheduleType::new(1, ChargingRateUnitEnumType::A, vec![period]);
        ChargingProfileType::new(1, 1, ChargingProfilePurposeEnumType::TxDefaultProfile, ChargingProfileKindEnumType::Absolute, vec![charging_schedule])
    }

    #[test]
    fn test_set_charging_profile_request_new() {
        let charging_profile = create_test_charging_profile();
        let request = SetChargingProfileRequest::new(1, charging_profile.clone());

        assert_eq!(request.evse_id, 1);
        assert_eq!(request.charging_profile, charging_profile);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_set_charging_profile_request_serialization() {
        let charging_profile = create_test_charging_profile();
        let request = SetChargingProfileRequest::new(2, charging_profile);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetChargingProfileRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"evseId\":2"));
    }

    #[test]
    fn test_set_charging_profile_request_validation() {
        let charging_profile = create_test_charging_profile();

        // Test valid request
        let valid_request = SetChargingProfileRequest::new(0, charging_profile.clone());
        assert!(valid_request.validate().is_ok());

        // Test invalid request with negative evse_id
        let invalid_request = SetChargingProfileRequest::new(-1, charging_profile);
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_set_charging_profile_request_builder_pattern() {
        let charging_profile = create_test_charging_profile();
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SetChargingProfileRequest::new(3, charging_profile.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.evse_id, 3);
        assert_eq!(request.charging_profile, charging_profile);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_charging_profile_request_setters() {
        let charging_profile1 = create_test_charging_profile();
        let charging_profile2 = create_test_charging_profile();
        let mut request = SetChargingProfileRequest::new(1, charging_profile1);
        let custom_data = CustomDataType::new("test_vendor".to_string());

        request.set_evse_id(4)
               .set_charging_profile(charging_profile2.clone())
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.evse_id, 4);
        assert_eq!(request.charging_profile, charging_profile2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_charging_profile_request_getters() {
        let charging_profile = create_test_charging_profile();
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SetChargingProfileRequest::new(5, charging_profile.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*request.get_evse_id(), 5);
        assert_eq!(*request.get_charging_profile(), charging_profile);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_charging_profile_response_new() {
        let response = SetChargingProfileResponse::new(ChargingProfileStatusEnumType::Accepted);
        assert_eq!(response.status, ChargingProfileStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_set_charging_profile_response_serialization() {
        let response = SetChargingProfileResponse::new(ChargingProfileStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetChargingProfileResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(json.contains("\"status\":\"Rejected\""));
    }

    #[test]
    fn test_set_charging_profile_response_builder_pattern() {
        let status_info = StatusInfoType::new("Profile conflict".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SetChargingProfileResponse::new(ChargingProfileStatusEnumType::Rejected)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, ChargingProfileStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_charging_profile_response_setters() {
        let mut response = SetChargingProfileResponse::new(ChargingProfileStatusEnumType::Accepted);
        let status_info = StatusInfoType::new("Updated status".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());

        response.set_status(ChargingProfileStatusEnumType::Rejected)
                .set_status_info(Some(status_info.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, ChargingProfileStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_charging_profile_response_getters() {
        let status_info = StatusInfoType::new("Test status".to_string());
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SetChargingProfileResponse::new(ChargingProfileStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*response.get_status(), ChargingProfileStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_charging_profile_edge_cases() {
        let charging_profile = create_test_charging_profile();

        // Test with evse_id = 0 (should be valid)
        let request = SetChargingProfileRequest::new(0, charging_profile);
        assert!(request.validate().is_ok());
        assert_eq!(request.evse_id, 0);
    }

    #[test]
    fn test_set_charging_profile_response_validation() {
        let response = SetChargingProfileResponse::new(ChargingProfileStatusEnumType::Accepted);
        assert!(response.validate().is_ok());
    }
}