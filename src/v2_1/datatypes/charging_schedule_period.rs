use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// Charging schedule period structure defines a time period in a charging schedule.
/// It is used in: CompositeScheduleType and in ChargingScheduleType.
/// When used in a NotifyEVChargingScheduleRequest only startPeriod, limit, limit_L2, limit_L3 are relevant.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriodType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Start of the period, in seconds from the start of schedule.
    /// The value of StartPeriod also defines the stop time of the previous period.
    pub start_period: i32,

    /// Optional only when not required by the operationMode, as in CentralSetpoint, ExternalSetpoint,
    /// ExternalLimits, LocalFrequency, LocalLoadBalancing.
    /// Charging rate limit during the schedule period, in the applicable chargingRateUnit.
    /// This SHOULD be a non-negative value; a negative value is only supported for backwards compatibility
    /// with older systems that use a negative value to specify a discharging limit.
    /// When using chargingRateUnit = 'W', this field represents the sum of the power of all phases,
    /// unless values are provided for L2 and L3, in which case this field represents phase L1.
    pub limit: f64,

    /// Charging rate limit on phase L2 in the applicable chargingRateUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l2: Option<f64>,

    /// Charging rate limit on phase L3 in the applicable chargingRateUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l3: Option<f64>,

    /// The number of phases that can be used for charging.
    /// If a number of phases is needed, numberPhases=3 will be assumed unless another number is given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,

    /// Values: 1..3, Used if numberPhases=1 and if the EVSE is capable of switching the phase connected to the EV,
    /// i.e. ACPhaseSwitchingSupported is defined and true. It's not allowed unless both conditions above are true.
    /// If both conditions are true, and phaseToUse is omitted, the Charging Station / EVSE will make the selection on its own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i32>,
}

impl ChargingSchedulePeriodType {
    /// Creates a new `ChargingSchedulePeriodType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start of the period, in seconds from the start of schedule
    /// * `limit` - Charging rate limit during the schedule period
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingSchedulePeriodType` with optional fields set to `None`
    pub fn new(start_period: i32, limit: f64) -> Self {
        Self {
            start_period,
            limit,
            limit_l2: None,
            limit_l3: None,
            number_phases: None,
            phase_to_use: None,
            custom_data: None,
        }
    }

    /// Sets the limit for the second phase.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit for the second phase
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l2(mut self, limit_l2: f64) -> Self {
        self.limit_l2 = Some(limit_l2);
        self
    }

    /// Sets the limit for the third phase.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit for the third phase
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l3(mut self, limit_l3: f64) -> Self {
        self.limit_l3 = Some(limit_l3);
        self
    }

    /// Sets the number of phases.
    ///
    /// # Arguments
    ///
    /// * `number_phases` - The number of phases that can be used for charging
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_number_phases(mut self, number_phases: i32) -> Self {
        self.number_phases = Some(number_phases);
        self
    }

    /// Sets the phase to use.
    ///
    /// # Arguments
    ///
    /// * `phase_to_use` - The phase to use (values: 1..3)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_phase_to_use(mut self, phase_to_use: i32) -> Self {
        self.phase_to_use = Some(phase_to_use);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging schedule period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the start period.
    ///
    /// # Returns
    ///
    /// The start of the period, in seconds from the start of schedule
    pub fn start_period(&self) -> i32 {
        self.start_period
    }

    /// Sets the start period.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start of the period, in seconds from the start of schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_period(&mut self, start_period: i32) -> &mut Self {
        self.start_period = start_period;
        self
    }

    /// Gets the charging rate limit.
    ///
    /// # Returns
    ///
    /// The charging rate limit during the schedule period
    pub fn limit(&self) -> f64 {
        self.limit
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
    pub fn set_limit(&mut self, limit: f64) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Gets the limit for the second phase.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit for the second phase
    pub fn limit_l2(&self) -> Option<f64> {
        self.limit_l2
    }

    /// Sets the limit for the second phase.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit for the second phase, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_l2(&mut self, limit_l2: Option<f64>) -> &mut Self {
        self.limit_l2 = limit_l2;
        self
    }

    /// Gets the limit for the third phase.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit for the third phase
    pub fn limit_l3(&self) -> Option<f64> {
        self.limit_l3
    }

    /// Sets the limit for the third phase.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit for the third phase, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_l3(&mut self, limit_l3: Option<f64>) -> &mut Self {
        self.limit_l3 = limit_l3;
        self
    }

    /// Gets the number of phases.
    ///
    /// # Returns
    ///
    /// An optional number of phases that can be used for charging
    pub fn number_phases(&self) -> Option<i32> {
        self.number_phases
    }

    /// Sets the number of phases.
    ///
    /// # Arguments
    ///
    /// * `number_phases` - The number of phases that can be used for charging, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_number_phases(&mut self, number_phases: Option<i32>) -> &mut Self {
        self.number_phases = number_phases;
        self
    }

    /// Gets the phase to use.
    ///
    /// # Returns
    ///
    /// An optional phase to use
    pub fn phase_to_use(&self) -> Option<i32> {
        self.phase_to_use
    }

    /// Sets the phase to use.
    ///
    /// # Arguments
    ///
    /// * `phase_to_use` - The phase to use (values: 1..3), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_phase_to_use(&mut self, phase_to_use: Option<i32>) -> &mut Self {
        self.phase_to_use = phase_to_use;
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
    /// * `custom_data` - Custom data for this charging schedule period, or None to clear
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
    fn test_new_charging_schedule_period() {
        let period = ChargingSchedulePeriodType::new(0, 16.0);

        assert_eq!(period.start_period(), 0);
        assert_eq!(period.limit(), 16.0);
        assert_eq!(period.limit_l2(), None);
        assert_eq!(period.limit_l3(), None);
        assert_eq!(period.number_phases(), None);
        assert_eq!(period.phase_to_use(), None);
        assert_eq!(period.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let period = ChargingSchedulePeriodType::new(0, 16.0)
            .with_limit_l2(16.0)
            .with_limit_l3(16.0)
            .with_number_phases(3)
            .with_phase_to_use(1)
            .with_custom_data(custom_data.clone());

        assert_eq!(period.start_period(), 0);
        assert_eq!(period.limit(), 16.0);
        assert_eq!(period.limit_l2(), Some(16.0));
        assert_eq!(period.limit_l3(), Some(16.0));
        assert_eq!(period.number_phases(), Some(3));
        assert_eq!(period.phase_to_use(), Some(1));
        assert_eq!(period.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut period = ChargingSchedulePeriodType::new(0, 16.0);

        period
            .set_start_period(3600)
            .set_limit(32.0)
            .set_limit_l2(Some(32.0))
            .set_limit_l3(Some(32.0))
            .set_number_phases(Some(3))
            .set_phase_to_use(Some(2))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(period.start_period(), 3600);
        assert_eq!(period.limit(), 32.0);
        assert_eq!(period.limit_l2(), Some(32.0));
        assert_eq!(period.limit_l3(), Some(32.0));
        assert_eq!(period.number_phases(), Some(3));
        assert_eq!(period.phase_to_use(), Some(2));
        assert_eq!(period.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        period
            .set_limit_l2(None)
            .set_limit_l3(None)
            .set_number_phases(None)
            .set_phase_to_use(None)
            .set_custom_data(None);

        assert_eq!(period.limit_l2(), None);
        assert_eq!(period.limit_l3(), None);
        assert_eq!(period.number_phases(), None);
        assert_eq!(period.phase_to_use(), None);
        assert_eq!(period.custom_data(), None);
    }
}
