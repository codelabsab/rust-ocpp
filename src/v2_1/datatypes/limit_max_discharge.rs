use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Limit max discharge settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LimitMaxDischargeType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Maximum discharge rate in percent of nominal discharge rate.
    pub discharge_limit: f64,
}

impl LimitMaxDischargeType {
    /// Creates a new `LimitMaxDischargeType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `discharge_limit` - Maximum discharge rate in percent of nominal discharge rate
    ///
    /// # Returns
    ///
    /// A new instance of `LimitMaxDischargeType` with optional fields set to `None`
    pub fn new(priority: i32, discharge_limit: f64) -> Self {
        Self {
            priority,
            discharge_limit,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this LimitMaxDischarge
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the priority.
    ///
    /// # Returns
    ///
    /// The priority of setting (0=highest)
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Sets the priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_priority(&mut self, priority: i32) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Gets the discharge limit.
    ///
    /// # Returns
    ///
    /// The maximum discharge rate in percent of nominal discharge rate
    pub fn discharge_limit(&self) -> f64 {
        self.discharge_limit
    }

    /// Sets the discharge limit.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit` - Maximum discharge rate in percent of nominal discharge rate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_discharge_limit(&mut self, discharge_limit: f64) -> &mut Self {
        self.discharge_limit = discharge_limit;
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
    /// * `custom_data` - Custom data for this LimitMaxDischarge, or None to clear
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
    fn test_new_limit_max_discharge() {
        let priority = 1;
        let discharge_limit = 75.0;

        let limit = LimitMaxDischargeType::new(priority, discharge_limit);

        assert_eq!(limit.priority(), priority);
        assert_eq!(limit.discharge_limit(), discharge_limit);
        assert_eq!(limit.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let priority = 1;
        let discharge_limit = 75.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let limit = LimitMaxDischargeType::new(priority, discharge_limit)
            .with_custom_data(custom_data.clone());

        assert_eq!(limit.priority(), priority);
        assert_eq!(limit.discharge_limit(), discharge_limit);
        assert_eq!(limit.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let discharge_limit1 = 75.0;
        let priority2 = 2;
        let discharge_limit2 = 85.0;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut limit = LimitMaxDischargeType::new(priority1, discharge_limit1);

        limit
            .set_priority(priority2)
            .set_discharge_limit(discharge_limit2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(limit.priority(), priority2);
        assert_eq!(limit.discharge_limit(), discharge_limit2);
        assert_eq!(limit.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        limit.set_custom_data(None);
        assert_eq!(limit.custom_data(), None);
    }
}
