use crate::v2_1::datatypes::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the CostUpdated request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedRequest {
    /// Current total cost, based on the information known by the CSMS, of the transaction including taxes. In the currency configured with the configuration Variable: [&lt;&lt;configkey-currency, Currency&gt;&gt;]
    pub total_cost: Decimal,

    /// Transaction Id of the transaction the current cost are asked for.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CostUpdatedRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `total_cost` - Current total cost, based on the information known by the CSMS, of the transaction including taxes. In the currency configured with the configuration Variable: [&lt;&lt;configkey-currency, Currency&gt;&gt;]
    /// * `transaction_id` - Transaction Id of the transaction the current cost are asked for.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(total_cost: Decimal, transaction_id: String) -> Self {
        Self {
            total_cost,
            transaction_id,
            custom_data: None,
        }
    }

    /// Sets the total_cost field.
    ///
    /// * `total_cost` - Current total cost, based on the information known by the CSMS, of the transaction including taxes. In the currency configured with the configuration Variable: [&lt;&lt;configkey-currency, Currency&gt;&gt;]
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_total_cost(&mut self, total_cost: Decimal) -> &mut Self {
        self.total_cost = total_cost;
        self
    }

    /// Sets the transaction_id field.
    ///
    /// * `transaction_id` - Transaction Id of the transaction the current cost are asked for.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_transaction_id(&mut self, transaction_id: String) -> &mut Self {
        self.transaction_id = transaction_id;
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

    /// Gets a reference to the total_cost field.
    ///
    /// # Returns
    ///
    /// Current total cost, based on the information known by the CSMS, of the transaction including taxes. In the currency configured with the configuration Variable: [&lt;&lt;configkey-currency, Currency&gt;&gt;]
    pub fn get_total_cost(&self) -> &Decimal {
        &self.total_cost
    }

    /// Gets a reference to the transaction_id field.
    ///
    /// # Returns
    ///
    /// Transaction Id of the transaction the current cost are asked for.
    pub fn get_transaction_id(&self) -> &String {
        &self.transaction_id
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

/// Response body for the CostUpdated response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CostUpdatedResponse {
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
    use rust_decimal::Decimal;
    use serde_json;
    use validator::Validate;

    #[test]
    fn test_cost_updated_request_new() {
        let total_cost = Decimal::new(1250, 2); // 12.50
        let transaction_id = "TXN123".to_string();
        let request = CostUpdatedRequest::new(total_cost, transaction_id.clone());

        assert_eq!(request.get_total_cost(), &total_cost);
        assert_eq!(request.get_transaction_id(), &transaction_id);
        assert_eq!(request.get_custom_data(), None);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_cost_updated_request_validation_invalid_transaction_id() {
        let total_cost = Decimal::new(1000, 2);
        let transaction_id = "x".repeat(37); // Invalid: max 36 chars
        let request = CostUpdatedRequest::new(total_cost, transaction_id);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_cost_updated_request_serialization() {
        let total_cost = Decimal::new(2500, 2); // 25.00
        let transaction_id = "TXN456".to_string();
        let request = CostUpdatedRequest::new(total_cost, transaction_id);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CostUpdatedRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_cost_updated_request_with_custom_data() {
        let total_cost = Decimal::new(750, 2); // 7.50
        let transaction_id = "TXN789".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = CostUpdatedRequest::new(total_cost, transaction_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_custom_data(), Some(&custom_data));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_cost_updated_request_set_methods() {
        let total_cost = Decimal::new(1000, 2);
        let new_total_cost = Decimal::new(1500, 2);
        let transaction_id = "TXN001".to_string();
        let new_transaction_id = "TXN002".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = CostUpdatedRequest::new(total_cost, transaction_id);

        request
            .set_total_cost(new_total_cost)
            .set_transaction_id(new_transaction_id.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.get_total_cost(), &new_total_cost);
        assert_eq!(request.get_transaction_id(), &new_transaction_id);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_cost_updated_request_edge_cases() {
        // Test with zero cost
        let zero_cost = Decimal::new(0, 2);
        let transaction_id = "TXN_ZERO".to_string();
        let request = CostUpdatedRequest::new(zero_cost, transaction_id);
        assert_eq!(request.get_total_cost(), &zero_cost);
        assert!(request.validate().is_ok());

        // Test with maximum length transaction ID
        let total_cost = Decimal::new(100, 2);
        let max_transaction_id = "x".repeat(36); // Max 36 chars
        let request = CostUpdatedRequest::new(total_cost, max_transaction_id.clone());
        assert_eq!(request.get_transaction_id(), &max_transaction_id);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_cost_updated_request_json_round_trip() {
        let total_cost = Decimal::new(9999, 2); // 99.99
        let transaction_id = "TXN_ROUND_TRIP".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let request = CostUpdatedRequest::new(total_cost, transaction_id)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: CostUpdatedRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_cost_updated_request_decimal_precision() {
        // Test with various decimal precisions
        let costs = vec![
            Decimal::new(1, 0),      // 1
            Decimal::new(10, 1),     // 1.0
            Decimal::new(100, 2),    // 1.00
            Decimal::new(1234, 3),   // 1.234
            Decimal::new(12345, 4),  // 1.2345
        ];

        for (i, cost) in costs.iter().enumerate() {
            let transaction_id = format!("TXN_{}", i);
            let request = CostUpdatedRequest::new(*cost, transaction_id);
            assert_eq!(request.get_total_cost(), cost);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_cost_updated_request_clear_optional_fields() {
        let total_cost = Decimal::new(500, 2);
        let transaction_id = "TXN_CLEAR".to_string();
        let custom_data = CustomDataType::new("TestVendor".to_string());

        let mut request = CostUpdatedRequest::new(total_cost, transaction_id)
            .with_custom_data(custom_data);

        // Verify custom data is set
        assert!(request.get_custom_data().is_some());

        // Clear custom data
        request.set_custom_data(None);
        assert_eq!(request.get_custom_data(), None);
    }
}