use crate::v2_1::datatypes::{ChargingScheduleUpdateType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::ChargingProfileStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the UpdateDynamicSchedule request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDynamicScheduleRequest {
    /// Id of charging profile to update.
    #[validate(range(min = 0))]
    pub charging_profile_id: i32,

    #[validate(nested)]
    pub schedule_update: ChargingScheduleUpdateType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UpdateDynamicScheduleRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `charging_profile_id` - Id of charging profile to update.
    /// * `schedule_update` - The schedule_update field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(charging_profile_id: i32, schedule_update: ChargingScheduleUpdateType) -> Self {
        Self {
            charging_profile_id,
            schedule_update,
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

    /// Sets the schedule_update field.
    ///
    /// * `schedule_update` - The schedule_update field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_schedule_update(&mut self, schedule_update: ChargingScheduleUpdateType) -> &mut Self {
        self.schedule_update = schedule_update;
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

    /// Gets a reference to the schedule_update field.
    ///
    /// # Returns
    ///
    /// The schedule_update field
    pub fn get_schedule_update(&self) -> &ChargingScheduleUpdateType {
        &self.schedule_update
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

/// Response body for the UpdateDynamicSchedule response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDynamicScheduleResponse {
    pub status: ChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl UpdateDynamicScheduleResponse {
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
    use crate::v2_1::datatypes::{ChargingScheduleUpdateType, CustomDataType, StatusInfoType};
    use crate::v2_1::enumerations::ChargingProfileStatusEnumType;
    use serde_json;
    use validator::Validate;

    fn create_test_schedule_update() -> ChargingScheduleUpdateType {
        ChargingScheduleUpdateType::new()
    }

    // Tests for UpdateDynamicScheduleRequest

    #[test]
    fn test_update_dynamic_schedule_request_new() {
        let charging_profile_id = 123;
        let schedule_update = create_test_schedule_update();
        let request = UpdateDynamicScheduleRequest::new(charging_profile_id, schedule_update.clone());

        assert_eq!(request.get_charging_profile_id(), &charging_profile_id);
        assert_eq!(request.get_schedule_update(), &schedule_update);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_update_dynamic_schedule_request_serialization() {
        let schedule_update = create_test_schedule_update();
        let request = UpdateDynamicScheduleRequest::new(456, schedule_update);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UpdateDynamicScheduleRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_update_dynamic_schedule_request_validation() {
        let schedule_update = create_test_schedule_update();
        let request = UpdateDynamicScheduleRequest::new(789, schedule_update);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_dynamic_schedule_request_with_custom_data() {
        let schedule_update = create_test_schedule_update();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UpdateDynamicScheduleRequest::new(101, schedule_update)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_dynamic_schedule_request_set_methods() {
        let schedule_update = create_test_schedule_update();
        let new_schedule_update = create_test_schedule_update();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = UpdateDynamicScheduleRequest::new(100, schedule_update);

        request
            .set_charging_profile_id(200)
            .set_schedule_update(new_schedule_update.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_charging_profile_id(), &200);
        assert_eq!(request.get_schedule_update(), &new_schedule_update);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_dynamic_schedule_request_builder_pattern() {
        let schedule_update = create_test_schedule_update();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = UpdateDynamicScheduleRequest::new(300, schedule_update)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_dynamic_schedule_request_negative_profile_id_validation() {
        let schedule_update = create_test_schedule_update();
        let mut request = UpdateDynamicScheduleRequest::new(1, schedule_update);
        request.set_charging_profile_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_dynamic_schedule_request_zero_profile_id_validation() {
        let schedule_update = create_test_schedule_update();
        let request = UpdateDynamicScheduleRequest::new(0, schedule_update);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_dynamic_schedule_request_large_profile_id() {
        let schedule_update = create_test_schedule_update();
        let request = UpdateDynamicScheduleRequest::new(999999, schedule_update);

        assert_eq!(request.get_charging_profile_id(), &999999);
        assert!(request.validate().is_ok());
    }

    // Tests for UpdateDynamicScheduleResponse

    #[test]
    fn test_update_dynamic_schedule_response_new() {
        let status = ChargingProfileStatusEnumType::Accepted;
        let response = UpdateDynamicScheduleResponse::new(status.clone());

        assert_eq!(response.get_status(), &status);
        assert_eq!(response.get_status_info(), None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_update_dynamic_schedule_response_serialization() {
        let response = UpdateDynamicScheduleResponse::new(ChargingProfileStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UpdateDynamicScheduleResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_update_dynamic_schedule_response_validation() {
        let response = UpdateDynamicScheduleResponse::new(ChargingProfileStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_update_dynamic_schedule_response_with_status_info() {
        let status_info = StatusInfoType::new("Success".to_string());
        let response = UpdateDynamicScheduleResponse::new(ChargingProfileStatusEnumType::Accepted)
            .with_status_info(status_info.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
    }

    #[test]
    fn test_update_dynamic_schedule_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UpdateDynamicScheduleResponse::new(ChargingProfileStatusEnumType::Rejected)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_dynamic_schedule_response_set_methods() {
        let status_info = StatusInfoType::new("Error".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = UpdateDynamicScheduleResponse::new(ChargingProfileStatusEnumType::Accepted);

        response
            .set_status(ChargingProfileStatusEnumType::Rejected)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_status(), &ChargingProfileStatusEnumType::Rejected);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_dynamic_schedule_response_builder_pattern() {
        let status_info = StatusInfoType::new("Info".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UpdateDynamicScheduleResponse::new(ChargingProfileStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_update_dynamic_schedule_response_all_status_types() {
        let status_types = vec![
            ChargingProfileStatusEnumType::Accepted,
            ChargingProfileStatusEnumType::Rejected,
        ];

        for status in status_types {
            let response = UpdateDynamicScheduleResponse::new(status.clone());
            
            assert_eq!(response.get_status(), &status);
            assert!(response.validate().is_ok());

            let json = serde_json::to_string(&response).expect("Failed to serialize");
            let deserialized: UpdateDynamicScheduleResponse = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(response, deserialized);
        }
    }

    #[test]
    fn test_update_dynamic_schedule_request_json_round_trip() {
        let schedule_update = create_test_schedule_update();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UpdateDynamicScheduleRequest::new(555, schedule_update)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: UpdateDynamicScheduleRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_update_dynamic_schedule_response_json_round_trip() {
        let status_info = StatusInfoType::new("Details".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = UpdateDynamicScheduleResponse::new(ChargingProfileStatusEnumType::Rejected)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: UpdateDynamicScheduleResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_update_dynamic_schedule_response_with_all_optional_fields() {
        let status_info = StatusInfoType::new("Complete".to_string());
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = UpdateDynamicScheduleResponse::new(ChargingProfileStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &ChargingProfileStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_update_dynamic_schedule_request_with_custom_data_validation() {
        let schedule_update = create_test_schedule_update();
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let request = UpdateDynamicScheduleRequest::new(777, schedule_update)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());
    }
}