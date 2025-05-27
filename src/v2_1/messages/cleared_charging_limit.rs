use crate::v2_1::datatypes::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClearedChargingLimit request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitRequest {
    /// Source of the charging limit. Allowed values defined in Appendix as ChargingLimitSourceEnumStringType.
    #[validate(length(max = 20))]
    pub charging_limit_source: String,

    /// EVSE Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearedChargingLimitRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `charging_limit_source` - Source of the charging limit. Allowed values defined in Appendix as ChargingLimitSourceEnumStringType.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(charging_limit_source: String) -> Self {
        Self {
            charging_limit_source,
            evse_id: None,
            custom_data: None,
        }
    }

    /// Sets the charging_limit_source field.
    ///
    /// * `charging_limit_source` - Source of the charging limit. Allowed values defined in Appendix as ChargingLimitSourceEnumStringType.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_charging_limit_source(&mut self, charging_limit_source: String) -> &mut Self {
        self.charging_limit_source = charging_limit_source;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - EVSE Identifier.
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

    /// Gets a reference to the charging_limit_source field.
    ///
    /// # Returns
    ///
    /// Source of the charging limit. Allowed values defined in Appendix as ChargingLimitSourceEnumStringType.
    pub fn get_charging_limit_source(&self) -> &String {
        &self.charging_limit_source
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// EVSE Identifier.
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
    /// * `evse_id` - EVSE Identifier.
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

/// Response body for the ClearedChargingLimit response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearedChargingLimitResponse {
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
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    #[test]
    fn test_cleared_charging_limit_request_new() {
        let charging_limit_source = "EMS".to_string();

        let request = ClearedChargingLimitRequest::new(charging_limit_source.clone());

        assert_eq!(request.charging_limit_source, charging_limit_source);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_cleared_charging_limit_request_with_evse_id() {
        let charging_limit_source = "EMS".to_string();
        let evse_id = 1;

        let request = ClearedChargingLimitRequest::new(charging_limit_source.clone())
            .with_evse_id(evse_id);

        assert_eq!(request.charging_limit_source, charging_limit_source);
        assert_eq!(request.evse_id, Some(evse_id));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_cleared_charging_limit_request_with_custom_data() {
        let charging_limit_source = "EMS".to_string();
        let custom_data = create_test_custom_data();

        let request = ClearedChargingLimitRequest::new(charging_limit_source.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.charging_limit_source, charging_limit_source);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_cleared_charging_limit_request_setters() {
        let mut request = ClearedChargingLimitRequest::new("EMS".to_string());

        let new_charging_limit_source = "CSO".to_string();
        let evse_id = 2;
        let custom_data = create_test_custom_data();

        request.set_charging_limit_source(new_charging_limit_source.clone())
               .set_evse_id(Some(evse_id))
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.charging_limit_source, new_charging_limit_source);
        assert_eq!(request.evse_id, Some(evse_id));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_cleared_charging_limit_request_getters() {
        let charging_limit_source = "EMS".to_string();
        let evse_id = 3;
        let custom_data = create_test_custom_data();

        let request = ClearedChargingLimitRequest::new(charging_limit_source.clone())
            .with_evse_id(evse_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_charging_limit_source(), &charging_limit_source);
        assert_eq!(request.get_evse_id(), Some(&evse_id));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_cleared_charging_limit_request_serialization() {
        let charging_limit_source = "EMS".to_string();
        let evse_id = 1;

        let request = ClearedChargingLimitRequest::new(charging_limit_source)
            .with_evse_id(evse_id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearedChargingLimitRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_cleared_charging_limit_request_validation_charging_limit_source_too_long() {
        let long_source = "a".repeat(21); // Max is 20

        let request = ClearedChargingLimitRequest::new(long_source);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_cleared_charging_limit_request_validation_negative_evse_id() {
        let request = ClearedChargingLimitRequest::new("EMS".to_string())
            .with_evse_id(-1);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_cleared_charging_limit_request_validation_valid() {
        let charging_limit_source = "EMS".to_string();
        let evse_id = 0; // 0 is valid (min = 0)

        let request = ClearedChargingLimitRequest::new(charging_limit_source)
            .with_evse_id(evse_id);

        let validation_result = request.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_cleared_charging_limit_request_common_charging_limit_sources() {
        let sources = vec![
            "EMS".to_string(),
            "CSO".to_string(),
            "SO".to_string(),
            "Other".to_string(),
        ];

        for source in sources {
            let request = ClearedChargingLimitRequest::new(source.clone());
            assert_eq!(request.charging_limit_source, source);
            assert!(request.validate().is_ok());

            // Test serialization for each source
            let json = serde_json::to_string(&request).expect("Failed to serialize");
            let deserialized: ClearedChargingLimitRequest =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(request, deserialized);
        }
    }

    #[test]
    fn test_cleared_charging_limit_response_new() {
        let response = ClearedChargingLimitResponse::new();

        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_cleared_charging_limit_response_with_custom_data() {
        let custom_data = create_test_custom_data();

        let response = ClearedChargingLimitResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_cleared_charging_limit_response_setters() {
        let mut response = ClearedChargingLimitResponse::new();

        let custom_data = create_test_custom_data();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_cleared_charging_limit_response_getters() {
        let custom_data = create_test_custom_data();

        let response = ClearedChargingLimitResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_cleared_charging_limit_response_serialization() {
        let response = ClearedChargingLimitResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearedChargingLimitResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_cleared_charging_limit_response_validation() {
        let response = ClearedChargingLimitResponse::new();

        let validation_result = response.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_cleared_charging_limit_request_builder_pattern() {
        let charging_limit_source = "EMS".to_string();
        let evse_id = 1;
        let custom_data = create_test_custom_data();

        let request = ClearedChargingLimitRequest::new(charging_limit_source.clone())
            .with_evse_id(evse_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.charging_limit_source, charging_limit_source);
        assert_eq!(request.evse_id, Some(evse_id));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_cleared_charging_limit_response_builder_pattern() {
        let custom_data = create_test_custom_data();

        let response = ClearedChargingLimitResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_cleared_charging_limit_request_edge_cases() {
        // Test with empty charging limit source
        let empty_source = "".to_string();
        let request = ClearedChargingLimitRequest::new(empty_source.clone());

        assert_eq!(request.charging_limit_source, empty_source);
        assert!(request.validate().is_ok());

        // Test with maximum length charging limit source
        let max_source = "a".repeat(20);
        let request_max = ClearedChargingLimitRequest::new(max_source.clone());

        assert_eq!(request_max.charging_limit_source, max_source);
        assert!(request_max.validate().is_ok());

        // Test with zero EVSE ID
        let request_zero_evse = ClearedChargingLimitRequest::new("EMS".to_string())
            .with_evse_id(0);

        assert_eq!(request_zero_evse.evse_id, Some(0));
        assert!(request_zero_evse.validate().is_ok());

        // Test with large EVSE ID
        let request_large_evse = ClearedChargingLimitRequest::new("EMS".to_string())
            .with_evse_id(999999);

        assert_eq!(request_large_evse.evse_id, Some(999999));
        assert!(request_large_evse.validate().is_ok());
    }

    #[test]
    fn test_cleared_charging_limit_response_with_and_without_custom_data() {
        // Test response without custom data
        let response_empty = ClearedChargingLimitResponse::new();
        assert_eq!(response_empty.custom_data, None);

        let json_empty = serde_json::to_string(&response_empty).expect("Failed to serialize");
        let deserialized_empty: ClearedChargingLimitResponse =
            serde_json::from_str(&json_empty).expect("Failed to deserialize");
        assert_eq!(response_empty, deserialized_empty);

        // Test response with custom data
        let custom_data = create_test_custom_data();
        let response_with_data = ClearedChargingLimitResponse::new()
            .with_custom_data(custom_data.clone());
        assert_eq!(response_with_data.custom_data, Some(custom_data));

        let json_with_data = serde_json::to_string(&response_with_data).expect("Failed to serialize");
        let deserialized_with_data: ClearedChargingLimitResponse =
            serde_json::from_str(&json_with_data).expect("Failed to deserialize");
        assert_eq!(response_with_data, deserialized_with_data);
    }
}