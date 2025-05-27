use crate::v2_1::datatypes::{ClearTariffsResultType, CustomDataType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ClearTariffs request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsRequest {
    /// List of tariff Ids to clear. When absent clears all tariffs at _evseId_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub tariff_ids: Option<Vec<String>>,

    /// When present only clear tariffs matching _tariffIds_ at EVSE _evseId_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearTariffsRequest {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            tariff_ids: None,
            evse_id: None,
            custom_data: None,
        }
    }

    /// Sets the tariff_ids field.
    ///
    /// * `tariff_ids` - List of tariff Ids to clear. When absent clears all tariffs at _evseId_.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tariff_ids(&mut self, tariff_ids: Option<Vec<String>>) -> &mut Self {
        self.tariff_ids = tariff_ids;
        self
    }

    /// Sets the evse_id field.
    ///
    /// * `evse_id` - When present only clear tariffs matching _tariffIds_ at EVSE _evseId_.
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

    /// Gets a reference to the tariff_ids field.
    ///
    /// # Returns
    ///
    /// List of tariff Ids to clear. When absent clears all tariffs at _evseId_.
    pub fn get_tariff_ids(&self) -> Option<&Vec<String>> {
        self.tariff_ids.as_ref()
    }

    /// Gets a reference to the evse_id field.
    ///
    /// # Returns
    ///
    /// When present only clear tariffs matching _tariffIds_ at EVSE _evseId_.
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

    /// Sets the tariff_ids field and returns self for builder pattern.
    ///
    /// * `tariff_ids` - List of tariff Ids to clear. When absent clears all tariffs at _evseId_.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tariff_ids(mut self, tariff_ids: Vec<String>) -> Self {
        self.tariff_ids = Some(tariff_ids);
        self
    }

    /// Sets the evse_id field and returns self for builder pattern.
    ///
    /// * `evse_id` - When present only clear tariffs matching _tariffIds_ at EVSE _evseId_.
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

/// Response body for the ClearTariffs response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsResponse {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub clear_tariffs_result: Vec<ClearTariffsResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearTariffsResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `clear_tariffs_result` - The clear_tariffs_result field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(clear_tariffs_result: Vec<ClearTariffsResultType>) -> Self {
        Self {
            clear_tariffs_result,
            custom_data: None,
        }
    }

    /// Sets the clear_tariffs_result field.
    ///
    /// * `clear_tariffs_result` - The clear_tariffs_result field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_clear_tariffs_result(&mut self, clear_tariffs_result: Vec<ClearTariffsResultType>) -> &mut Self {
        self.clear_tariffs_result = clear_tariffs_result;
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

    /// Gets a reference to the clear_tariffs_result field.
    ///
    /// # Returns
    ///
    /// The clear_tariffs_result field
    pub fn get_clear_tariffs_result(&self) -> &Vec<ClearTariffsResultType> {
        &self.clear_tariffs_result
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

    fn create_test_clear_tariffs_result() -> ClearTariffsResultType {
        ClearTariffsResultType::new(crate::v2_1::datatypes::clear_tariffs_result::TariffClearStatusEnumType::Accepted)
            .with_tariff_id("test-tariff-123".to_string())
    }

    #[test]
    fn test_clear_tariffs_request_new() {
        let request = ClearTariffsRequest::new();

        assert_eq!(request.tariff_ids, None);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_clear_tariffs_request_with_tariff_ids() {
        let tariff_ids = vec!["tariff-1".to_string(), "tariff-2".to_string()];

        let request = ClearTariffsRequest::new()
            .with_tariff_ids(tariff_ids.clone());

        assert_eq!(request.tariff_ids, Some(tariff_ids));
        assert_eq!(request.evse_id, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_clear_tariffs_request_with_evse_id() {
        let evse_id = 1;

        let request = ClearTariffsRequest::new()
            .with_evse_id(evse_id);

        assert_eq!(request.tariff_ids, None);
        assert_eq!(request.evse_id, Some(evse_id));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_clear_tariffs_request_with_custom_data() {
        let custom_data = create_test_custom_data();

        let request = ClearTariffsRequest::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(request.tariff_ids, None);
        assert_eq!(request.evse_id, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_clear_tariffs_request_setters() {
        let mut request = ClearTariffsRequest::new();

        let tariff_ids = vec!["tariff-1".to_string(), "tariff-2".to_string()];
        let evse_id = 2;
        let custom_data = create_test_custom_data();

        request.set_tariff_ids(Some(tariff_ids.clone()))
               .set_evse_id(Some(evse_id))
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.tariff_ids, Some(tariff_ids));
        assert_eq!(request.evse_id, Some(evse_id));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_clear_tariffs_request_getters() {
        let tariff_ids = vec!["tariff-1".to_string(), "tariff-2".to_string()];
        let evse_id = 3;
        let custom_data = create_test_custom_data();

        let request = ClearTariffsRequest::new()
            .with_tariff_ids(tariff_ids.clone())
            .with_evse_id(evse_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_tariff_ids(), Some(&tariff_ids));
        assert_eq!(request.get_evse_id(), Some(&evse_id));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_tariffs_request_serialization() {
        let tariff_ids = vec!["tariff-1".to_string(), "tariff-2".to_string()];
        let evse_id = 1;

        let request = ClearTariffsRequest::new()
            .with_tariff_ids(tariff_ids)
            .with_evse_id(evse_id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ClearTariffsRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_clear_tariffs_request_validation_empty_tariff_ids() {
        let empty_tariff_ids: Vec<String> = vec![];

        let request = ClearTariffsRequest::new()
            .with_tariff_ids(empty_tariff_ids);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_clear_tariffs_request_validation_negative_evse_id() {
        let request = ClearTariffsRequest::new()
            .with_evse_id(-1);

        let validation_result = request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_clear_tariffs_request_validation_valid() {
        let tariff_ids = vec!["tariff-1".to_string()];
        let evse_id = 0; // 0 is valid (min = 0)

        let request = ClearTariffsRequest::new()
            .with_tariff_ids(tariff_ids)
            .with_evse_id(evse_id);

        let validation_result = request.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_clear_tariffs_response_new() {
        let results = vec![create_test_clear_tariffs_result()];

        let response = ClearTariffsResponse::new(results.clone());

        assert_eq!(response.clear_tariffs_result, results);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_clear_tariffs_response_with_custom_data() {
        let results = vec![create_test_clear_tariffs_result()];
        let custom_data = create_test_custom_data();

        let response = ClearTariffsResponse::new(results.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.clear_tariffs_result, results);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_clear_tariffs_response_setters() {
        let mut response = ClearTariffsResponse::new(vec![create_test_clear_tariffs_result()]);

        let new_results = vec![
            create_test_clear_tariffs_result(),
            ClearTariffsResultType::new(crate::v2_1::datatypes::clear_tariffs_result::TariffClearStatusEnumType::Rejected)
                .with_tariff_id("tariff-456".to_string())
        ];
        let custom_data = create_test_custom_data();

        response.set_clear_tariffs_result(new_results.clone())
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.clear_tariffs_result, new_results);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_clear_tariffs_response_getters() {
        let results = vec![create_test_clear_tariffs_result()];
        let custom_data = create_test_custom_data();

        let response = ClearTariffsResponse::new(results.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_clear_tariffs_result(), &results);
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_clear_tariffs_response_serialization() {
        let results = vec![create_test_clear_tariffs_result()];

        let response = ClearTariffsResponse::new(results);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearTariffsResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_clear_tariffs_response_validation_empty_results() {
        let empty_results: Vec<ClearTariffsResultType> = vec![];

        let response = ClearTariffsResponse::new(empty_results);

        let validation_result = response.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_clear_tariffs_response_validation_valid() {
        let results = vec![create_test_clear_tariffs_result()];

        let response = ClearTariffsResponse::new(results);

        let validation_result = response.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_clear_tariffs_request_builder_pattern() {
        let tariff_ids = vec!["tariff-1".to_string(), "tariff-2".to_string()];
        let evse_id = 1;
        let custom_data = create_test_custom_data();

        let request = ClearTariffsRequest::new()
            .with_tariff_ids(tariff_ids.clone())
            .with_evse_id(evse_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.tariff_ids, Some(tariff_ids));
        assert_eq!(request.evse_id, Some(evse_id));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_clear_tariffs_response_builder_pattern() {
        let results = vec![create_test_clear_tariffs_result()];
        let custom_data = create_test_custom_data();

        let response = ClearTariffsResponse::new(results.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.clear_tariffs_result, results);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_clear_tariffs_request_edge_cases() {
        // Test with single tariff ID
        let single_tariff = vec!["single-tariff".to_string()];
        let request = ClearTariffsRequest::new()
            .with_tariff_ids(single_tariff.clone());

        assert_eq!(request.tariff_ids, Some(single_tariff));
        assert!(request.validate().is_ok());

        // Test with zero EVSE ID
        let request_zero_evse = ClearTariffsRequest::new()
            .with_evse_id(0);

        assert_eq!(request_zero_evse.evse_id, Some(0));
        assert!(request_zero_evse.validate().is_ok());

        // Test with large EVSE ID
        let request_large_evse = ClearTariffsRequest::new()
            .with_evse_id(999999);

        assert_eq!(request_large_evse.evse_id, Some(999999));
        assert!(request_large_evse.validate().is_ok());
    }

    #[test]
    fn test_clear_tariffs_response_multiple_results() {
        let results = vec![
            ClearTariffsResultType::new(crate::v2_1::datatypes::clear_tariffs_result::TariffClearStatusEnumType::Accepted)
                .with_tariff_id("tariff-1".to_string()),
            ClearTariffsResultType::new(crate::v2_1::datatypes::clear_tariffs_result::TariffClearStatusEnumType::Rejected)
                .with_tariff_id("tariff-2".to_string()),
            ClearTariffsResultType::new(crate::v2_1::datatypes::clear_tariffs_result::TariffClearStatusEnumType::NoTariff),
        ];

        let response = ClearTariffsResponse::new(results.clone());

        assert_eq!(response.clear_tariffs_result.len(), 3);
        assert_eq!(response.clear_tariffs_result, results);
        assert!(response.validate().is_ok());

        // Test serialization with multiple results
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ClearTariffsResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
    }
}