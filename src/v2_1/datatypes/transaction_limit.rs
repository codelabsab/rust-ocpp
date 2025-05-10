use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Transaction limits for a charging session.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLimitType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. Maximum amount of energy in kWh that may be delivered to an EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_limit: Option<f64>,

    /// Optional. Maximum duration in seconds that a transaction may last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<i32>,
}

impl TransactionLimitType {
    /// Creates a new `TransactionLimitType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `TransactionLimitType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
            energy_limit: None,
            time_limit: None,
        }
    }

    /// Gets the energy limit.
    ///
    /// # Returns
    ///
    /// An optional maximum amount of energy in kWh that may be delivered to an EV
    pub fn energy_limit(&self) -> Option<f64> {
        self.energy_limit
    }

    /// Gets the time limit.
    ///
    /// # Returns
    ///
    /// An optional maximum duration in seconds that a transaction may last
    pub fn time_limit(&self) -> Option<i32> {
        self.time_limit
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the energy limit.
    ///
    /// # Arguments
    ///
    /// * `energy_limit` - Maximum amount of energy in kWh that may be delivered to an EV, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_limit(&mut self, energy_limit: Option<f64>) -> &mut Self {
        self.energy_limit = energy_limit;
        self
    }

    /// Sets the time limit.
    ///
    /// # Arguments
    ///
    /// * `time_limit` - Maximum duration in seconds that a transaction may last, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_limit(&mut self, time_limit: Option<i32>) -> &mut Self {
        self.time_limit = time_limit;
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this transaction limit, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the energy limit.
    ///
    /// # Arguments
    ///
    /// * `energy_limit` - Maximum amount of energy in kWh that may be delivered to an EV
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy_limit(mut self, energy_limit: f64) -> Self {
        self.energy_limit = Some(energy_limit);
        self
    }

    /// Sets the time limit.
    ///
    /// # Arguments
    ///
    /// * `time_limit` - Maximum duration in seconds that a transaction may last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_time_limit(mut self, time_limit: i32) -> Self {
        self.time_limit = Some(time_limit);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this transaction limit
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction_limit() {
        let transaction_limit = TransactionLimitType::new();

        assert_eq!(transaction_limit.energy_limit(), None);
        assert_eq!(transaction_limit.time_limit(), None);
        assert_eq!(transaction_limit.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let energy_limit = 50.0;
        let time_limit = 3600;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let transaction_limit = TransactionLimitType::new()
            .with_energy_limit(energy_limit)
            .with_time_limit(time_limit)
            .with_custom_data(custom_data.clone());

        assert_eq!(transaction_limit.energy_limit(), Some(energy_limit));
        assert_eq!(transaction_limit.time_limit(), Some(time_limit));
        assert_eq!(transaction_limit.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let mut transaction_limit = TransactionLimitType::new();
        let custom_data = CustomDataType::new("VendorX".to_string());

        transaction_limit
            .set_energy_limit(Some(50.0))
            .set_time_limit(Some(3600))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(transaction_limit.energy_limit(), Some(50.0));
        assert_eq!(transaction_limit.time_limit(), Some(3600));
        assert_eq!(transaction_limit.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        transaction_limit
            .set_energy_limit(None)
            .set_time_limit(None)
            .set_custom_data(None);

        assert_eq!(transaction_limit.energy_limit(), None);
        assert_eq!(transaction_limit.time_limit(), None);
        assert_eq!(transaction_limit.custom_data(), None);
    }
}
