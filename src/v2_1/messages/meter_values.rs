use crate::v2_1::datatypes::{CustomDataType, MeterValueType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the MeterValues request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    /// This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    #[validate(length(min = 1))]
    #[validate(nested)]
    pub meter_value: Vec<MeterValueType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl MeterValuesRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `evse_id` - This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    /// * `meter_value` - The meter_value field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(evse_id: i32, meter_value: Vec<MeterValueType>) -> Self {
        Self {
            evse_id,
            meter_value,
            custom_data: None,
        }
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Sets the meter_value field.
    ///
    /// * `meter_value` - The meter_value field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_meter_value(&mut self, meter_value: Vec<MeterValueType>) -> &mut Self {
        self.meter_value = meter_value;
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
    /// This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    pub fn get_evse_id(&self) -> &i32 {
        &self.evse_id
    }

    /// Gets a reference to the meter_value field.
    ///
    /// # Returns
    ///
    /// The meter_value field
    pub fn get_meter_value(&self) -> &Vec<MeterValueType> {
        &self.meter_value
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

/// Response body for the MeterValues response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl MeterValuesResponse {
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
    use crate::v2_1::datatypes::SampledValueType;
    use chrono::Utc;
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_sampled_value() -> SampledValueType {
        SampledValueType::new(42.5)
    }

    fn create_test_meter_value() -> MeterValueType {
        let timestamp = Utc::now();
        let sampled_values = vec![create_test_sampled_value()];
        MeterValueType::new(timestamp, sampled_values)
    }

    // Tests for MeterValuesRequest

    #[test]
    fn test_meter_values_request_new() {
        let meter_values = vec![create_test_meter_value()];
        let request = MeterValuesRequest::new(1, meter_values.clone());

        assert_eq!(request.evse_id, 1);
        assert_eq!(request.meter_value, meter_values);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_meter_values_request_with_evse_id_zero() {
        let meter_values = vec![create_test_meter_value()];
        let request = MeterValuesRequest::new(0, meter_values.clone());

        assert_eq!(request.evse_id, 0);
        assert_eq!(request.meter_value, meter_values);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_meter_values_request_with_custom_data() {
        let meter_values = vec![create_test_meter_value()];
        let custom_data = create_test_custom_data();
        let request = MeterValuesRequest::new(2, meter_values.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.evse_id, 2);
        assert_eq!(request.meter_value, meter_values);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_meter_values_request_setters() {
        let meter_values1 = vec![create_test_meter_value()];
        let meter_values2 = vec![create_test_meter_value(), create_test_meter_value()];
        let custom_data = create_test_custom_data();

        let mut request = MeterValuesRequest::new(1, meter_values1);
        request.set_evse_id(3);
        request.set_meter_value(meter_values2.clone());
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.evse_id, 3);
        assert_eq!(request.meter_value, meter_values2);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_meter_values_request_getters() {
        let meter_values = vec![create_test_meter_value()];
        let custom_data = create_test_custom_data();
        let request = MeterValuesRequest::new(4, meter_values.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_evse_id(), &4);
        assert_eq!(request.get_meter_value(), &meter_values);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_meter_values_request_serialization() {
        let meter_values = vec![create_test_meter_value()];
        let request = MeterValuesRequest::new(1, meter_values);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: MeterValuesRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_meter_values_request_validation() {
        let meter_values = vec![create_test_meter_value()];
        let request = MeterValuesRequest::new(1, meter_values);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_meter_values_request_validation_negative_evse_id() {
        let meter_values = vec![create_test_meter_value()];
        let mut request = MeterValuesRequest::new(1, meter_values);
        request.set_evse_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_meter_values_request_validation_empty_meter_values() {
        let mut request = MeterValuesRequest::new(1, vec![create_test_meter_value()]);
        request.set_meter_value(vec![]); // Empty list should fail validation

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_meter_values_request_multiple_meter_values() {
        let meter_values = vec![
            create_test_meter_value(),
            create_test_meter_value(),
            create_test_meter_value(),
        ];
        let request = MeterValuesRequest::new(1, meter_values.clone());

        assert_eq!(request.meter_value.len(), 3);
        assert_eq!(request.meter_value, meter_values);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_meter_values_request_json_round_trip() {
        let meter_values = vec![create_test_meter_value()];
        let custom_data = create_test_custom_data();
        let request = MeterValuesRequest::new(5, meter_values)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: MeterValuesRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for MeterValuesResponse

    #[test]
    fn test_meter_values_response_new() {
        let response = MeterValuesResponse::new();

        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_meter_values_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = MeterValuesResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_meter_values_response_setters() {
        let custom_data = create_test_custom_data();

        let mut response = MeterValuesResponse::new();
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_meter_values_response_getters() {
        let custom_data = create_test_custom_data();
        let response = MeterValuesResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_meter_values_response_serialization() {
        let response = MeterValuesResponse::new();

        let json = serde_json::to_string(&response).unwrap();
        let parsed: MeterValuesResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_meter_values_response_validation() {
        let response = MeterValuesResponse::new();

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_meter_values_response_json_round_trip() {
        let custom_data = create_test_custom_data();
        let response = MeterValuesResponse::new()
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: MeterValuesResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }

    #[test]
    fn test_meter_values_response_clear_custom_data() {
        let custom_data = create_test_custom_data();
        let mut response = MeterValuesResponse::new()
            .with_custom_data(custom_data);

        assert!(response.custom_data.is_some());

        response.set_custom_data(None);
        assert_eq!(response.custom_data, None);
    }
}
