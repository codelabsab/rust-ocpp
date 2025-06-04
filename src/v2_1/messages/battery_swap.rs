use crate::v2_1::datatypes::{BatteryDataType, CustomDataType, IdTokenType};
use crate::v2_1::enumerations::BatterySwapEventEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the BatterySwap request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapRequest {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub battery_data: Vec<BatteryDataType>,

    pub event_type: BatterySwapEventEnumType,

    #[validate(nested)]
    pub id_token: IdTokenType,

    /// RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    #[validate(range(min = 0))]
    pub request_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BatterySwapRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `battery_data` - The battery_data field
    /// * `event_type` - The event_type field
    /// * `id_token` - The id_token field
    /// * `request_id` - RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(battery_data: Vec<BatteryDataType>, event_type: BatterySwapEventEnumType, id_token: IdTokenType, request_id: i32) -> Self {
        Self {
            battery_data,
            event_type,
            id_token,
            request_id,
            custom_data: None,
        }
    }

    /// Sets the battery_data field.
    ///
    /// * `battery_data` - The battery_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_battery_data(&mut self, battery_data: Vec<BatteryDataType>) -> &mut Self {
        self.battery_data = battery_data;
        self
    }

    /// Sets the event_type field.
    ///
    /// * `event_type` - The event_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_event_type(&mut self, event_type: BatterySwapEventEnumType) -> &mut Self {
        self.event_type = event_type;
        self
    }

    /// Sets the id_token field.
    ///
    /// * `id_token` - The id_token field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_id_token(&mut self, id_token: IdTokenType) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
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

    /// Gets a reference to the battery_data field.
    ///
    /// # Returns
    ///
    /// The battery_data field
    pub fn get_battery_data(&self) -> &Vec<BatteryDataType> {
        &self.battery_data
    }

    /// Gets a reference to the event_type field.
    ///
    /// # Returns
    ///
    /// The event_type field
    pub fn get_event_type(&self) -> &BatterySwapEventEnumType {
        &self.event_type
    }

    /// Gets a reference to the id_token field.
    ///
    /// # Returns
    ///
    /// The id_token field
    pub fn get_id_token(&self) -> &IdTokenType {
        &self.id_token
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
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

/// Response body for the BatterySwap response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BatterySwapResponse {
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
    use chrono::Utc;
    use rust_decimal::Decimal;
    use serde_json;
    use validator::Validate;

    fn create_test_battery_data() -> BatteryDataType {
        BatteryDataType::new(
            1,                       // evse_id
            "BAT123456".to_string(), // serial_number
            Decimal::new(750, 1),    // so_c (75.0%)
            Decimal::new(850, 1),    // so_h (85.0%)
            Utc::now(),              // production_date
        )
    }

    fn create_test_id_token() -> IdTokenType {
        IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string())
    }

    #[test]
    fn test_battery_swap_request_new() {
        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = 123;

        let request = BatterySwapRequest::new(
            battery_data.clone(),
            event_type.clone(),
            id_token.clone(),
            request_id,
        );

        assert_eq!(request.get_battery_data(), &battery_data);
        assert_eq!(request.get_event_type(), &event_type);
        assert_eq!(request.get_id_token(), &id_token);
        assert_eq!(request.get_request_id(), &request_id);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_battery_swap_request_validation() {
        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = 123;

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_battery_swap_request_validation_empty_battery_data() {
        let battery_data = vec![]; // Invalid: must have at least 1 item
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = 123;

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_battery_swap_request_validation_invalid_request_id() {
        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = -1; // Invalid: must be >= 0

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_battery_swap_request_serialization() {
        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = 123;

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: BatterySwapRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_battery_swap_request_with_custom_data() {
        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = 123;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_battery_swap_request_set_methods() {
        let battery_data = vec![create_test_battery_data()];
        let new_battery_data = vec![
            create_test_battery_data(),
            BatteryDataType::new(
                2,
                "BAT789012".to_string(),
                Decimal::new(600, 1), // 60.0%
                Decimal::new(900, 1), // 90.0%
                Utc::now(),
            ),
        ];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let new_event_type = BatterySwapEventEnumType::BatteryOut;
        let id_token = create_test_id_token();
        let new_id_token = IdTokenType::new("ABCD1234567890".to_string(), "RFID".to_string());
        let request_id = 123;
        let new_request_id = 456;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id);

        request
            .set_battery_data(new_battery_data.clone())
            .set_event_type(new_event_type.clone())
            .set_id_token(new_id_token.clone())
            .set_request_id(new_request_id)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_battery_data(), &new_battery_data);
        assert_eq!(request.get_event_type(), &new_event_type);
        assert_eq!(request.get_id_token(), &new_id_token);
        assert_eq!(request.get_request_id(), &new_request_id);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_battery_swap_request_all_event_types() {
        let battery_data = vec![create_test_battery_data()];
        let id_token = create_test_id_token();
        let request_id = 123;

        let event_types = vec![
            BatterySwapEventEnumType::BatteryIn,
            BatterySwapEventEnumType::BatteryOut,
            BatterySwapEventEnumType::BatteryOutTimeout,
        ];

        for event_type in event_types {
            let request = BatterySwapRequest::new(
                battery_data.clone(),
                event_type.clone(),
                id_token.clone(),
                request_id,
            );
            assert_eq!(request.get_event_type(), &event_type);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_battery_swap_request_multiple_batteries() {
        let battery_data = vec![
            create_test_battery_data(),
            BatteryDataType::new(
                2,
                "BAT789012".to_string(),
                Decimal::new(600, 1), // 60.0%
                Decimal::new(900, 1), // 90.0%
                Utc::now(),
            ),
            BatteryDataType::new(
                3,
                "BAT345678".to_string(),
                Decimal::new(800, 1), // 80.0%
                Decimal::new(950, 1), // 95.0%
                Utc::now(),
            ),
        ];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = 123;

        let request = BatterySwapRequest::new(battery_data.clone(), event_type, id_token, request_id);

        assert_eq!(request.get_battery_data().len(), 3);
        assert_eq!(request.get_battery_data(), &battery_data);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_battery_swap_request_edge_cases() {
        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();

        // Test with minimum valid request_id
        let request = BatterySwapRequest::new(
            battery_data.clone(),
            event_type.clone(),
            id_token.clone(),
            0,
        );
        assert_eq!(request.get_request_id(), &0);
        assert!(request.validate().is_ok());

        // Test with large request_id
        let request = BatterySwapRequest::new(
            battery_data,
            event_type,
            id_token,
            i32::MAX,
        );
        assert_eq!(request.get_request_id(), &i32::MAX);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_battery_swap_response_new() {
        let response = BatterySwapResponse::new();

        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_battery_swap_response_validation() {
        let response = BatterySwapResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_battery_swap_response_serialization() {
        let response = BatterySwapResponse::new();

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: BatterySwapResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_battery_swap_response_with_custom_data() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = BatterySwapResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_battery_swap_response_set_methods() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = BatterySwapResponse::new();

        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_battery_swap_request_json_round_trip() {
        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryOut;
        let id_token = create_test_id_token();
        let request_id = 123;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: BatterySwapRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_battery_swap_response_json_round_trip() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let response = BatterySwapResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: BatterySwapResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_battery_swap_request_clear_optional_fields() {
        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = 123;
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id)
            .with_custom_data(custom_data);

        // Verify custom data is set
        assert!(request.get_custom_data().is_some());

        // Clear custom data
        request.set_custom_data(None);
        assert_eq!(request.get_custom_data(), None);
    }

    #[test]
    fn test_battery_swap_response_clear_optional_fields() {
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut response = BatterySwapResponse::new()
            .with_custom_data(custom_data);

        // Verify custom data is set
        assert!(response.get_custom_data().is_some());

        // Clear custom data
        response.set_custom_data(None);
        assert_eq!(response.get_custom_data(), None);
    }

    #[test]
    fn test_battery_swap_request_with_complex_battery_data() {
        use serde_json::json;

        let custom_data = CustomDataType::new("BatteryVendor".to_string())
            .with_property("region".to_string(), json!("EU"))
            .with_property("warranty".to_string(), json!("5 years"));

        let battery_data = vec![
            BatteryDataType::new(
                1,
                "BAT123456".to_string(),
                Decimal::new(750, 1), // 75.0%
                Decimal::new(850, 1), // 85.0%
                Utc::now(),
            )
            .with_vendor_info("Vendor specific battery info".to_string())
            .with_custom_data(custom_data),
        ];

        let event_type = BatterySwapEventEnumType::BatteryIn;
        let id_token = create_test_id_token();
        let request_id = 123;

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id);

        assert!(request.validate().is_ok());

        // Test serialization with complex data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: BatterySwapRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_battery_swap_request_with_complex_id_token() {
        use crate::v2_1::datatypes::AdditionalInfoType;

        let additional_info = vec![
            AdditionalInfoType::new("key1".to_string(), "value1".to_string()),
            AdditionalInfoType::new("key2".to_string(), "value2".to_string()),
        ];

        let id_token = IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string())
            .with_additional_info(additional_info);

        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryIn;
        let request_id = 123;

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id);

        assert!(request.validate().is_ok());

        // Test serialization
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: BatterySwapRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_battery_swap_request_with_complex_custom_data() {
        use serde_json::json;

        let battery_data = vec![create_test_battery_data()];
        let event_type = BatterySwapEventEnumType::BatteryOutTimeout;
        let id_token = create_test_id_token();
        let request_id = 123;

        let custom_data = CustomDataType::new("SwapVendor".to_string())
            .with_property("station_id".to_string(), json!("STATION_001"))
            .with_property("operator".to_string(), json!("John Doe"))
            .with_property("metadata".to_string(), json!({
                "swap_duration": 120,
                "temperature": 25.5,
                "humidity": 60
            }));

        let request = BatterySwapRequest::new(battery_data, event_type, id_token, request_id)
            .with_custom_data(custom_data);

        assert!(request.validate().is_ok());

        // Test serialization with complex custom data
        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: BatterySwapRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }
}