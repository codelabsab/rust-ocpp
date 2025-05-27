use crate::v2_1::datatypes::{ChargingScheduleUpdateType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::ChargingProfileStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the PullDynamicScheduleUpdate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PullDynamicScheduleUpdateRequest {
    /// Id of charging profile to update.
    #[validate(range(min = 0))]
    pub charging_profile_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PullDynamicScheduleUpdateRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `charging_profile_id` - Id of charging profile to update.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(charging_profile_id: i32) -> Self {
        Self {
            charging_profile_id,
            custom_data: None,
        }
    }

    /// Sets the charging_profile_id field.
    ///
    /// * `charging_profile_id` - Id of charging profile to update.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_profile_id(&mut self, charging_profile_id: i32) -> &mut Self {
        self.charging_profile_id = charging_profile_id;
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
    /// Id of charging profile to update.
    pub fn get_charging_profile_id(&self) -> &i32 {
        &self.charging_profile_id
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

/// Response body for the PullDynamicScheduleUpdate response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PullDynamicScheduleUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub schedule_update: Option<ChargingScheduleUpdateType>,

    pub status: ChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PullDynamicScheduleUpdateResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: ChargingProfileStatusEnumType) -> Self {
        Self {
            schedule_update: None,
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the schedule_update field.
    ///
    /// * `schedule_update` - The schedule_update field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_schedule_update(&mut self, schedule_update: Option<ChargingScheduleUpdateType>) -> &mut Self {
        self.schedule_update = schedule_update;
        self
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

    /// Gets a reference to the schedule_update field.
    ///
    /// # Returns
    ///
    /// The schedule_update field
    pub fn get_schedule_update(&self) -> Option<&ChargingScheduleUpdateType> {
        self.schedule_update.as_ref()
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

    /// Sets the schedule_update field and returns self for builder pattern.
    ///
    /// * `schedule_update` - The schedule_update field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_schedule_update(mut self, schedule_update: ChargingScheduleUpdateType) -> Self {
        self.schedule_update = Some(schedule_update);
        self
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
    use crate::v2_1::datatypes::StatusInfoType;
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    fn create_test_schedule_update() -> ChargingScheduleUpdateType {
        ChargingScheduleUpdateType::new()
    }

    #[test]
    fn test_pull_dynamic_schedule_update_request_new() {
        let request = PullDynamicScheduleUpdateRequest::new(123);

        assert_eq!(request.charging_profile_id, 123);
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_pull_dynamic_schedule_update_request_serialization() {
        let request = PullDynamicScheduleUpdateRequest::new(456)
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: PullDynamicScheduleUpdateRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.charging_profile_id, 456);
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_pull_dynamic_schedule_update_request_validation() {
        let valid_request = PullDynamicScheduleUpdateRequest::new(0);
        assert!(valid_request.validate().is_ok());

        let invalid_request = PullDynamicScheduleUpdateRequest {
            charging_profile_id: -1,
            custom_data: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_pull_dynamic_schedule_update_request_builder_pattern() {
        let custom_data = create_test_custom_data();
        let request = PullDynamicScheduleUpdateRequest::new(789)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.charging_profile_id, 789);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_pull_dynamic_schedule_update_request_setters_getters() {
        let mut request = PullDynamicScheduleUpdateRequest::new(100);
        let custom_data = create_test_custom_data();

        // Test setters
        request.set_charging_profile_id(200);
        request.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*request.get_charging_profile_id(), 200);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_pull_dynamic_schedule_update_response_new() {
        let response = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Accepted);

        assert_eq!(response.status, ChargingProfileStatusEnumType::Accepted);
        assert!(response.schedule_update.is_none());
        assert!(response.status_info.is_none());
        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_pull_dynamic_schedule_update_response_serialization() {
        let response = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Rejected)
            .with_schedule_update(create_test_schedule_update())
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: PullDynamicScheduleUpdateResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.status, ChargingProfileStatusEnumType::Rejected);
        assert!(deserialized.schedule_update.is_some());
        assert!(deserialized.status_info.is_some());
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_pull_dynamic_schedule_update_response_validation() {
        let valid_response = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Accepted);
        assert!(valid_response.validate().is_ok());

        let response_with_all_fields = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Rejected)
            .with_schedule_update(create_test_schedule_update())
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());
        assert!(response_with_all_fields.validate().is_ok());
    }

    #[test]
    fn test_pull_dynamic_schedule_update_response_builder_pattern() {
        let schedule_update = create_test_schedule_update();
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let response = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Accepted)
            .with_schedule_update(schedule_update.clone())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, ChargingProfileStatusEnumType::Accepted);
        assert_eq!(response.schedule_update, Some(schedule_update));
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_pull_dynamic_schedule_update_response_setters_getters() {
        let mut response = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Accepted);
        let schedule_update = create_test_schedule_update();
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        // Test setters
        response.set_status(ChargingProfileStatusEnumType::Rejected);
        response.set_schedule_update(Some(schedule_update.clone()));
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*response.get_status(), ChargingProfileStatusEnumType::Rejected);
        assert_eq!(response.get_schedule_update(), Some(&schedule_update));
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_pull_dynamic_schedule_update_response_enum_variants() {
        let accepted_response = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Accepted);
        assert_eq!(accepted_response.status, ChargingProfileStatusEnumType::Accepted);

        let rejected_response = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Rejected);
        assert_eq!(rejected_response.status, ChargingProfileStatusEnumType::Rejected);
    }

    #[test]
    fn test_pull_dynamic_schedule_update_request_edge_cases() {
        // Test minimum valid charging_profile_id
        let min_request = PullDynamicScheduleUpdateRequest::new(0);
        assert!(min_request.validate().is_ok());

        // Test large charging_profile_id
        let large_request = PullDynamicScheduleUpdateRequest::new(i32::MAX);
        assert!(large_request.validate().is_ok());
    }

    #[test]
    fn test_pull_dynamic_schedule_update_json_round_trip() {
        let original_request = PullDynamicScheduleUpdateRequest::new(12345)
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_request).expect("Failed to serialize request");
        let parsed_request: PullDynamicScheduleUpdateRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");

        assert_eq!(original_request, parsed_request);

        let original_response = PullDynamicScheduleUpdateResponse::new(ChargingProfileStatusEnumType::Accepted)
            .with_schedule_update(create_test_schedule_update())
            .with_status_info(create_test_status_info())
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_response).expect("Failed to serialize response");
        let parsed_response: PullDynamicScheduleUpdateResponse =
            serde_json::from_str(&json).expect("Failed to deserialize response");

        assert_eq!(original_response, parsed_response);
    }
}
