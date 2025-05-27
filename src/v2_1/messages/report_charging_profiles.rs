use crate::v2_1::datatypes::{ChargingProfileType, CustomDataType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ReportChargingProfiles request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesRequest {
    /// Id used to match the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt; message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL contain the same value.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    #[validate(length(max = 20))]
    pub charging_limit_source: String,

    #[validate(length(min = 1))]
    #[validate(nested)]
    pub charging_profile: Vec<ChargingProfileType>,

    /// To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReportChargingProfilesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - Id used to match the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt; message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL contain the same value.
    /// * `charging_limit_source` - Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    /// * `charging_profile` - The charging_profile field
    /// * `evse_id` - The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32, charging_limit_source: String, charging_profile: Vec<ChargingProfileType>, evse_id: i32) -> Self {
        Self {
            request_id,
            charging_limit_source,
            charging_profile,
            tbc: None,
            evse_id,
            custom_data: None,
        }
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - Id used to match the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt; message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL contain the same value.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the charging_limit_source field.
    ///
    /// * `charging_limit_source` - Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_limit_source(&mut self, charging_limit_source: String) -> &mut Self {
        self.charging_limit_source = charging_limit_source;
        self
    }

    /// Sets the charging_profile field.
    ///
    /// * `charging_profile` - The charging_profile field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_profile(&mut self, charging_profile: Vec<ChargingProfileType>) -> &mut Self {
        self.charging_profile = charging_profile;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tbc(&mut self, tbc: Option<bool>) -> &mut Self {
        self.tbc = tbc;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
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

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// Id used to match the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt; message with the resulting ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL contain the same value.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the charging_limit_source field.
    ///
    /// # Returns
    ///
    /// Source that has installed this charging profile. Values defined in Appendix as ChargingLimitSourceEnumStringType.
    pub fn get_charging_limit_source(&self) -> &String {
        &self.charging_limit_source
    }

    /// Gets a reference to the charging_profile field.
    ///
    /// # Returns
    ///
    /// The charging_profile field
    pub fn get_charging_profile(&self) -> &Vec<ChargingProfileType> {
        &self.charging_profile
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    pub fn get_tbc(&self) -> Option<&bool> {
        self.tbc.as_ref()
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// The evse to which the charging profile applies. If evseId = 0, the message contains an overall limit for the Charging Station.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tbc(mut self, tbc: bool) -> Self {
        self.tbc = Some(tbc);
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

/// Response body for the ReportChargingProfiles response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReportChargingProfilesResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
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
    use crate::v2_1::datatypes::{ChargingProfileType, ChargingScheduleType};
    use crate::v2_1::enumerations::{ChargingProfileKindEnumType, ChargingProfilePurposeEnumType, ChargingRateUnitEnumType};
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_charging_profile() -> ChargingProfileType {
        use crate::v2_1::datatypes::ChargingSchedulePeriodType;
        use rust_decimal::prelude::*;

        let period = ChargingSchedulePeriodType::new(0, Decimal::from_str("1000.0").unwrap());
        let schedule = ChargingScheduleType::new(
            1,
            ChargingRateUnitEnumType::W,
            vec![period]
        );
        ChargingProfileType::new(
            1,
            0,
            ChargingProfilePurposeEnumType::TxProfile,
            ChargingProfileKindEnumType::Absolute,
            vec![schedule]
        )
    }

    #[test]
    fn test_report_charging_profiles_request_new() {
        let charging_profiles = vec![create_test_charging_profile()];
        let request = ReportChargingProfilesRequest::new(
            123,
            "EMS".to_string(),
            charging_profiles.clone(),
            1
        );

        assert_eq!(request.request_id, 123);
        assert_eq!(request.charging_limit_source, "EMS");
        assert_eq!(request.charging_profile, charging_profiles);
        assert_eq!(request.evse_id, 1);
        assert!(request.tbc.is_none());
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_report_charging_profiles_request_serialization() {
        let charging_profiles = vec![create_test_charging_profile()];
        let request = ReportChargingProfilesRequest::new(
            456,
            "CSO".to_string(),
            charging_profiles.clone(),
            2
        )
        .with_tbc(true)
        .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ReportChargingProfilesRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.request_id, 456);
        assert_eq!(deserialized.charging_limit_source, "CSO");
        assert_eq!(deserialized.evse_id, 2);
        assert_eq!(deserialized.tbc, Some(true));
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_report_charging_profiles_request_validation() {
        let charging_profiles = vec![create_test_charging_profile()];

        // Valid request
        let valid_request = ReportChargingProfilesRequest::new(
            0,
            "EMS".to_string(),
            charging_profiles.clone(),
            0
        );
        assert!(valid_request.validate().is_ok());

        // Invalid request_id (negative)
        let invalid_request = ReportChargingProfilesRequest {
            request_id: -1,
            charging_limit_source: "EMS".to_string(),
            charging_profile: charging_profiles.clone(),
            tbc: None,
            evse_id: 0,
            custom_data: None,
        };
        assert!(invalid_request.validate().is_err());

        // Invalid charging_limit_source (too long)
        let invalid_source_request = ReportChargingProfilesRequest {
            request_id: 0,
            charging_limit_source: "A".repeat(21),
            charging_profile: charging_profiles.clone(),
            tbc: None,
            evse_id: 0,
            custom_data: None,
        };
        assert!(invalid_source_request.validate().is_err());

        // Invalid charging_profile (empty)
        let empty_profiles_request = ReportChargingProfilesRequest {
            request_id: 0,
            charging_limit_source: "EMS".to_string(),
            charging_profile: vec![],
            tbc: None,
            evse_id: 0,
            custom_data: None,
        };
        assert!(empty_profiles_request.validate().is_err());

        // Invalid evse_id (negative)
        let invalid_evse_request = ReportChargingProfilesRequest {
            request_id: 0,
            charging_limit_source: "EMS".to_string(),
            charging_profile: charging_profiles,
            tbc: None,
            evse_id: -1,
            custom_data: None,
        };
        assert!(invalid_evse_request.validate().is_err());
    }

    #[test]
    fn test_report_charging_profiles_request_builder_pattern() {
        let charging_profiles = vec![create_test_charging_profile()];
        let custom_data = create_test_custom_data();

        let request = ReportChargingProfilesRequest::new(
            789,
            "SO".to_string(),
            charging_profiles.clone(),
            3
        )
        .with_tbc(false)
        .with_custom_data(custom_data.clone());

        assert_eq!(request.request_id, 789);
        assert_eq!(request.charging_limit_source, "SO");
        assert_eq!(request.charging_profile, charging_profiles);
        assert_eq!(request.evse_id, 3);
        assert_eq!(request.tbc, Some(false));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_report_charging_profiles_request_setters_getters() {
        let charging_profiles = vec![create_test_charging_profile()];
        let mut request = ReportChargingProfilesRequest::new(
            100,
            "EMS".to_string(),
            charging_profiles.clone(),
            1
        );
        let custom_data = create_test_custom_data();
        let new_profiles = vec![create_test_charging_profile(), create_test_charging_profile()];

        // Test setters
        request.set_request_id(200);
        request.set_charging_limit_source("Other".to_string());
        request.set_charging_profile(new_profiles.clone());
        request.set_tbc(Some(true));
        request.set_evse_id(5);
        request.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*request.get_request_id(), 200);
        assert_eq!(request.get_charging_limit_source(), "Other");
        assert_eq!(request.get_charging_profile(), &new_profiles);
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(*request.get_evse_id(), 5);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_report_charging_profiles_response_new() {
        let response = ReportChargingProfilesResponse::new();

        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_report_charging_profiles_response_serialization() {
        let response = ReportChargingProfilesResponse::new()
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ReportChargingProfilesResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_report_charging_profiles_response_validation() {
        let valid_response = ReportChargingProfilesResponse::new();
        assert!(valid_response.validate().is_ok());

        let response_with_custom_data = ReportChargingProfilesResponse::new()
            .with_custom_data(create_test_custom_data());
        assert!(response_with_custom_data.validate().is_ok());
    }

    #[test]
    fn test_report_charging_profiles_response_builder_pattern() {
        let custom_data = create_test_custom_data();
        let response = ReportChargingProfilesResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_report_charging_profiles_response_setters_getters() {
        let mut response = ReportChargingProfilesResponse::new();
        let custom_data = create_test_custom_data();

        // Test setter
        response.set_custom_data(Some(custom_data.clone()));

        // Test getter
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_report_charging_profiles_request_edge_cases() {
        let charging_profiles = vec![create_test_charging_profile()];

        // Test minimum valid values
        let min_request = ReportChargingProfilesRequest::new(
            0,
            "".to_string(),
            charging_profiles.clone(),
            0
        );
        assert!(min_request.validate().is_ok());

        // Test maximum valid charging_limit_source length
        let max_source_request = ReportChargingProfilesRequest::new(
            i32::MAX,
            "A".repeat(20),
            charging_profiles.clone(),
            i32::MAX
        );
        assert!(max_source_request.validate().is_ok());

        // Test multiple charging profiles
        let multiple_profiles = vec![
            create_test_charging_profile(),
            create_test_charging_profile(),
            create_test_charging_profile()
        ];
        let multi_request = ReportChargingProfilesRequest::new(
            100,
            "EMS".to_string(),
            multiple_profiles,
            1
        );
        assert!(multi_request.validate().is_ok());
    }

    #[test]
    fn test_report_charging_profiles_json_round_trip() {
        let charging_profiles = vec![create_test_charging_profile()];
        let original_request = ReportChargingProfilesRequest::new(
            12345,
            "CSO".to_string(),
            charging_profiles,
            10
        )
        .with_tbc(true)
        .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_request).expect("Failed to serialize request");
        let parsed_request: ReportChargingProfilesRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");

        assert_eq!(original_request, parsed_request);

        let original_response = ReportChargingProfilesResponse::new()
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_response).expect("Failed to serialize response");
        let parsed_response: ReportChargingProfilesResponse =
            serde_json::from_str(&json).expect("Failed to deserialize response");

        assert_eq!(original_response, parsed_response);
    }

    #[test]
    fn test_report_charging_profiles_request_charging_limit_sources() {
        let charging_profiles = vec![create_test_charging_profile()];

        // Test different valid charging limit sources
        let sources = vec!["EMS", "Other", "SO", "CSO"];
        for source in sources {
            let request = ReportChargingProfilesRequest::new(
                1,
                source.to_string(),
                charging_profiles.clone(),
                1
            );
            assert!(request.validate().is_ok());
            assert_eq!(request.charging_limit_source, source);
        }
    }

    #[test]
    fn test_report_charging_profiles_request_tbc_values() {
        let charging_profiles = vec![create_test_charging_profile()];

        // Test with tbc = true
        let request_true = ReportChargingProfilesRequest::new(
            1,
            "EMS".to_string(),
            charging_profiles.clone(),
            1
        ).with_tbc(true);
        assert_eq!(request_true.tbc, Some(true));

        // Test with tbc = false
        let request_false = ReportChargingProfilesRequest::new(
            1,
            "EMS".to_string(),
            charging_profiles.clone(),
            1
        ).with_tbc(false);
        assert_eq!(request_false.tbc, Some(false));

        // Test without tbc (None)
        let request_none = ReportChargingProfilesRequest::new(
            1,
            "EMS".to_string(),
            charging_profiles,
            1
        );
        assert_eq!(request_none.tbc, None);
    }
}