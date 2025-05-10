use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Updates to a ChargingSchedulePeriodType for dynamic charging profiles.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleUpdateType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Optional only when not required by the _operationMode_, as in CentralSetpoint, ExternalSetpoint, ExternalLimits, LocalFrequency,  LocalLoadBalancing.
    /// Charging rate limit during the schedule period, in the applicable _chargingRateUnit_.
    /// This SHOULD be a non-negative value; a negative value is only supported for backwards compatibility with older systems that use a negative value to specify a discharging limit.
    /// For AC this field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f32>,

    /// *(2.1)* Charging rate limit on phase L2 in the applicable _chargingRateUnit_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l2: Option<f32>,

    /// *(2.1)* Charging rate limit on phase L3 in the applicable _chargingRateUnit_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l3: Option<f32>,
}

impl ChargingScheduleUpdateType {
    /// Creates a new `ChargingScheduleUpdateType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingScheduleUpdateType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
            limit: None,
            limit_l2: None,
            limit_l3: None,
        }
    }

    /// Sets the charging rate limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - Charging rate limit during the schedule period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit(mut self, limit: f32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the charging rate limit on phase L2.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit on phase L2
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l2(mut self, limit_l2: f32) -> Self {
        self.limit_l2 = Some(limit_l2);
        self
    }

    /// Sets the charging rate limit on phase L3.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit on phase L3
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l3(mut self, limit_l3: f32) -> Self {
        self.limit_l3 = Some(limit_l3);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging schedule update
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the charging rate limit.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit
    pub fn limit(&self) -> Option<f32> {
        self.limit
    }

    /// Sets the charging rate limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - Charging rate limit during the schedule period, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit(&mut self, limit: Option<f32>) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Gets the charging rate limit on phase L2.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit on phase L2
    pub fn limit_l2(&self) -> Option<f32> {
        self.limit_l2
    }

    /// Sets the charging rate limit on phase L2.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit on phase L2, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_l2(&mut self, limit_l2: Option<f32>) -> &mut Self {
        self.limit_l2 = limit_l2;
        self
    }

    /// Gets the charging rate limit on phase L3.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit on phase L3
    pub fn limit_l3(&self) -> Option<f32> {
        self.limit_l3
    }

    /// Sets the charging rate limit on phase L3.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit on phase L3, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_l3(&mut self, limit_l3: Option<f32>) -> &mut Self {
        self.limit_l3 = limit_l3;
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
    /// * `custom_data` - Custom data for this charging schedule update, or None to clear
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
    fn test_new_charging_schedule_update() {
        let update = ChargingScheduleUpdateType::new();

        assert_eq!(update.limit(), None);
        assert_eq!(update.limit_l2(), None);
        assert_eq!(update.limit_l3(), None);
        assert_eq!(update.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let update = ChargingScheduleUpdateType::new()
            .with_limit(16.0)
            .with_limit_l2(16.0)
            .with_limit_l3(16.0)
            .with_custom_data(custom_data.clone());

        assert_eq!(update.limit(), Some(16.0));
        assert_eq!(update.limit_l2(), Some(16.0));
        assert_eq!(update.limit_l3(), Some(16.0));
        assert_eq!(update.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut update = ChargingScheduleUpdateType::new();

        update
            .set_limit(Some(32.0))
            .set_limit_l2(Some(32.0))
            .set_limit_l3(Some(32.0))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(update.limit(), Some(32.0));
        assert_eq!(update.limit_l2(), Some(32.0));
        assert_eq!(update.limit_l3(), Some(32.0));
        assert_eq!(update.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        update
            .set_limit(None)
            .set_limit_l2(None)
            .set_limit_l3(None)
            .set_custom_data(None);

        assert_eq!(update.limit(), None);
        assert_eq!(update.limit_l2(), None);
        assert_eq!(update.limit_l3(), None);
        assert_eq!(update.custom_data(), None);
    }
}
