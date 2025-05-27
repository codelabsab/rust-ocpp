use crate::v2_1::datatypes::{ChargingProfileCriterionType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GetChargingProfileStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetChargingProfiles request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    /// Reference identification that is to be used by the Charging Station in the &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when provided.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported. + Reported charging profiles SHALL match the criteria in field _chargingProfile_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[validate(nested)]
    pub charging_profile: ChargingProfileCriterionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetChargingProfilesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - Reference identification that is to be used by the Charging Station in the &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when provided.
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, charging_profile: ChargingProfileCriterionType) -> Self {
        Self {
            request_id,
            evse_id: None,
            charging_profile,
            custom_data: None,
        }
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - Reference identification that is to be used by the Charging Station in the &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when provided.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported. + Reported charging profiles SHALL match the criteria in field _chargingProfile_.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
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
    pub fn set_charging_profile(&mut self, charging_profile: ChargingProfileCriterionType) -> &mut Self {
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

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// Reference identification that is to be used by the Charging Station in the &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when provided.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported. + Reported charging profiles SHALL match the criteria in field _chargingProfile_.
    pub fn get_evse_id(&self) -> Option<&i32> {
        self.evse_id.as_ref()
    }

    /// Gets a reference to the charging_profile field.
    ///
    /// # Returns
    ///
    /// The charging_profile field
    pub fn get_charging_profile(&self) -> &ChargingProfileCriterionType {
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

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported. + Reported charging profiles SHALL match the criteria in field _chargingProfile_.
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

/// Response body for the GetChargingProfiles response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesResponse {
    pub status: GetChargingProfileStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetChargingProfilesResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GetChargingProfileStatusEnumType) -> Self {
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
    pub fn set_status(&mut self, status: GetChargingProfileStatusEnumType) -> &mut Self {
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
    pub fn get_status(&self) -> &GetChargingProfileStatusEnumType {
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

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    fn create_test_charging_profile_criterion() -> ChargingProfileCriterionType {
        ChargingProfileCriterionType::new()
    }

    // Tests for GetChargingProfilesRequest

    #[test]
    fn test_get_charging_profiles_request_new() {
        let charging_profile = create_test_charging_profile_criterion();
        let request = GetChargingProfilesRequest::new(123, charging_profile.clone());

        assert_eq!(request.request_id, 123);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.charging_profile, charging_profile);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_charging_profiles_request_with_evse_id() {
        let charging_profile = create_test_charging_profile_criterion();
        let request = GetChargingProfilesRequest::new(456, charging_profile.clone())
            .with_evse_id(1);

        assert_eq!(request.request_id, 456);
        assert_eq!(request.evse_id, Some(1));
        assert_eq!(request.charging_profile, charging_profile);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_charging_profiles_request_with_custom_data() {
        let charging_profile = create_test_charging_profile_criterion();
        let custom_data = create_test_custom_data();
        let request = GetChargingProfilesRequest::new(789, charging_profile.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.request_id, 789);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.charging_profile, charging_profile);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_charging_profiles_request_setters() {
        let charging_profile1 = create_test_charging_profile_criterion();
        let charging_profile2 = ChargingProfileCriterionType::new();
        let custom_data = create_test_custom_data();

        let mut request = GetChargingProfilesRequest::new(100, charging_profile1);
        request.set_request_id(200);
        request.set_evse_id(Some(2));
        request.set_charging_profile(charging_profile2.clone());
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.request_id, 200);
        assert_eq!(request.evse_id, Some(2));
        assert_eq!(request.charging_profile, charging_profile2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_charging_profiles_request_getters() {
        let charging_profile = create_test_charging_profile_criterion();
        let custom_data = create_test_custom_data();
        let request = GetChargingProfilesRequest::new(999, charging_profile.clone())
            .with_evse_id(3)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_request_id(), &999);
        assert_eq!(request.get_evse_id(), Some(&3));
        assert_eq!(request.get_charging_profile(), &charging_profile);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_charging_profiles_request_serialization() {
        let charging_profile = create_test_charging_profile_criterion();
        let request = GetChargingProfilesRequest::new(555, charging_profile);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetChargingProfilesRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_charging_profiles_request_validation() {
        let charging_profile = create_test_charging_profile_criterion();
        let request = GetChargingProfilesRequest::new(100, charging_profile);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_charging_profiles_request_validation_negative_request_id() {
        let charging_profile = create_test_charging_profile_criterion();
        let mut request = GetChargingProfilesRequest::new(100, charging_profile);
        request.set_request_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_charging_profiles_request_validation_negative_evse_id() {
        let charging_profile = create_test_charging_profile_criterion();
        let mut request = GetChargingProfilesRequest::new(100, charging_profile);
        request.set_evse_id(Some(-1));

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_charging_profiles_request_json_round_trip() {
        let charging_profile = create_test_charging_profile_criterion();
        let custom_data = create_test_custom_data();
        let request = GetChargingProfilesRequest::new(777, charging_profile)
            .with_evse_id(5)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetChargingProfilesRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetChargingProfilesResponse

    #[test]
    fn test_get_charging_profiles_response_new() {
        let response = GetChargingProfilesResponse::new(GetChargingProfileStatusEnumType::Accepted);

        assert_eq!(response.status, GetChargingProfileStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_charging_profiles_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = GetChargingProfilesResponse::new(GetChargingProfileStatusEnumType::NoProfiles)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, GetChargingProfileStatusEnumType::NoProfiles);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_charging_profiles_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetChargingProfilesResponse::new(GetChargingProfileStatusEnumType::Accepted)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GetChargingProfileStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_charging_profiles_response_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut response = GetChargingProfilesResponse::new(GetChargingProfileStatusEnumType::Accepted);
        response.set_status(GetChargingProfileStatusEnumType::NoProfiles);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, GetChargingProfileStatusEnumType::NoProfiles);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_charging_profiles_response_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetChargingProfilesResponse::new(GetChargingProfileStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &GetChargingProfileStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_charging_profiles_response_serialization() {
        let response = GetChargingProfilesResponse::new(GetChargingProfileStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetChargingProfilesResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_charging_profiles_response_validation() {
        let response = GetChargingProfilesResponse::new(GetChargingProfileStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_charging_profiles_response_all_status_types() {
        let statuses = vec![
            GetChargingProfileStatusEnumType::Accepted,
            GetChargingProfileStatusEnumType::NoProfiles,
        ];

        for status in statuses {
            let response = GetChargingProfilesResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_charging_profiles_response_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetChargingProfilesResponse::new(GetChargingProfileStatusEnumType::NoProfiles)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetChargingProfilesResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}