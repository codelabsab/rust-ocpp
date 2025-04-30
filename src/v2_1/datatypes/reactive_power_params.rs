use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Parameters for reactive power control.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReactivePowerParamsType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Maximum reactive power in VAr.
    pub max_var: f64,

    /// Required. Minimum reactive power in VAr.
    pub min_var: f64,
}

impl ReactivePowerParamsType {
    /// Creates a new `ReactivePowerParamsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `max_var` - Maximum reactive power in VAr
    /// * `min_var` - Minimum reactive power in VAr
    ///
    /// # Returns
    ///
    /// A new instance of `ReactivePowerParamsType` with optional fields set to `None`
    pub fn new(max_var: f64, min_var: f64) -> Self {
        Self {
            max_var,
            min_var,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these reactive power parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the maximum reactive power.
    ///
    /// # Returns
    ///
    /// The maximum reactive power in VAr
    pub fn max_var(&self) -> f64 {
        self.max_var
    }

    /// Sets the maximum reactive power.
    ///
    /// # Arguments
    ///
    /// * `max_var` - Maximum reactive power in VAr
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_var(&mut self, max_var: f64) -> &mut Self {
        self.max_var = max_var;
        self
    }

    /// Gets the minimum reactive power.
    ///
    /// # Returns
    ///
    /// The minimum reactive power in VAr
    pub fn min_var(&self) -> f64 {
        self.min_var
    }

    /// Sets the minimum reactive power.
    ///
    /// # Arguments
    ///
    /// * `min_var` - Minimum reactive power in VAr
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_var(&mut self, min_var: f64) -> &mut Self {
        self.min_var = min_var;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these reactive power parameters, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_reactive_power_params() {
        let max_var = 100.0;
        let min_var = -100.0;
        let params = ReactivePowerParamsType::new(max_var, min_var);

        assert_eq!(params.max_var(), max_var);
        assert_eq!(params.min_var(), min_var);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let max_var = 100.0;
        let min_var = -100.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = ReactivePowerParamsType::new(max_var, min_var)
            .with_custom_data(custom_data.clone());

        assert_eq!(params.max_var(), max_var);
        assert_eq!(params.min_var(), min_var);
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let max_var1 = 100.0;
        let min_var1 = -100.0;
        let max_var2 = 200.0;
        let min_var2 = -200.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = ReactivePowerParamsType::new(max_var1, min_var1);

        params
            .set_max_var(max_var2)
            .set_min_var(min_var2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.max_var(), max_var2);
        assert_eq!(params.min_var(), min_var2);
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params.set_custom_data(None);
        assert_eq!(params.custom_data(), None);
    }
}
