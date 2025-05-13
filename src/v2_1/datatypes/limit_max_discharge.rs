use serde::{Deserialize, Serialize};
use validator::Validate;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use super::custom_data::CustomDataType;
use super::der_curve::DERCurveType;

/// Limit max discharge settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LimitMaxDischargeType {
    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Only for PowerMonitoring. The value specifies a percentage (0 to 100) of the rated maximum discharge power of EV. The PowerMonitoring curve becomes active when power exceeds this percentage.
    #[serde(with = "rust_decimal::serde::arbitrary_precision", rename = "pctMaxDischargePower")]
    pub pct_max_discharge_power: Decimal,

    #[serde(rename = "startTime")]
    pub start_time: Option<DateTime<Utc>>,

    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "powerMonitoringMustTrip")]
    #[validate(nested)]
    pub power_monitoring_must_trip: Option<DERCurveType>,

    /// Maximum discharge rate in percent of nominal discharge rate.
    pub discharge_limit: f64,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl LimitMaxDischargeType {
    /// Creates a new `LimitMaxDischargeType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `pct_max_discharge_power` - Percentage (0 to 100) of the rated maximum discharge power of EV
    /// * `discharge_limit` - Maximum discharge rate in percent of nominal discharge rate
    ///
    /// # Returns
    ///
    /// A new instance of `LimitMaxDischargeType` with optional fields set to `None`
    pub fn new(priority: i32, pct_max_discharge_power: Decimal, discharge_limit: f64) -> Self {
        Self {
            priority,
            pct_max_discharge_power,
            discharge_limit,
            start_time: None,
            duration: None,
            power_monitoring_must_trip: None,
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

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Time when this setting becomes active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_start_time(mut self, start_time: DateTime<Utc>) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds that this setting is active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_duration(mut self, duration: Decimal) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets the power monitoring must trip curve.
    ///
    /// # Arguments
    ///
    /// * `power_monitoring_must_trip` - Power monitoring must trip curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_power_monitoring_must_trip(mut self, power_monitoring_must_trip: DERCurveType) -> Self {
        self.power_monitoring_must_trip = Some(power_monitoring_must_trip);
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

    /// Gets the percentage of maximum discharge power.
    ///
    /// # Returns
    ///
    /// The percentage (0 to 100) of the rated maximum discharge power of EV
    pub fn pct_max_discharge_power(&self) -> &Decimal {
        &self.pct_max_discharge_power
    }

    /// Sets the percentage of maximum discharge power.
    ///
    /// # Arguments
    ///
    /// * `pct_max_discharge_power` - Percentage (0 to 100) of the rated maximum discharge power of EV
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_pct_max_discharge_power(&mut self, pct_max_discharge_power: Decimal) -> &mut Self {
        self.pct_max_discharge_power = pct_max_discharge_power;
        self
    }

    /// Gets the start time.
    ///
    /// # Returns
    ///
    /// An optional reference to the time when this setting becomes active
    pub fn start_time(&self) -> Option<&DateTime<Utc>> {
        self.start_time.as_ref()
    }

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Time when this setting becomes active, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_time(&mut self, start_time: Option<DateTime<Utc>>) -> &mut Self {
        self.start_time = start_time;
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// An optional reference to the duration in seconds that this setting is active
    pub fn duration(&self) -> Option<&Decimal> {
        self.duration.as_ref()
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds that this setting is active, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: Option<Decimal>) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Gets the power monitoring must trip curve.
    ///
    /// # Returns
    ///
    /// An optional reference to the power monitoring must trip curve
    pub fn power_monitoring_must_trip(&self) -> Option<&DERCurveType> {
        self.power_monitoring_must_trip.as_ref()
    }

    /// Sets the power monitoring must trip curve.
    ///
    /// # Arguments
    ///
    /// * `power_monitoring_must_trip` - Power monitoring must trip curve, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_monitoring_must_trip(&mut self, power_monitoring_must_trip: Option<DERCurveType>) -> &mut Self {
        self.power_monitoring_must_trip = power_monitoring_must_trip;
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
    use rust_decimal::prelude::*;
    use chrono::TimeZone;

    #[test]
    fn test_new_limit_max_discharge() {
        let priority = 1;
        let pct_max_discharge_power = Decimal::from_str("80.0").unwrap();
        let discharge_limit = 75.0;

        let limit = LimitMaxDischargeType::new(priority, pct_max_discharge_power.clone(), discharge_limit);

        assert_eq!(limit.priority(), priority);
        assert_eq!(limit.pct_max_discharge_power(), &pct_max_discharge_power);
        assert_eq!(limit.discharge_limit(), discharge_limit);
        assert_eq!(limit.start_time(), None);
        assert_eq!(limit.duration(), None);
        assert_eq!(limit.power_monitoring_must_trip(), None);
        assert_eq!(limit.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let priority = 1;
        let pct_max_discharge_power = Decimal::from_str("80.0").unwrap();
        let discharge_limit = 75.0;
        let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let duration = Decimal::from_str("3600.0").unwrap();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        // Create a simple DERCurveType for testing
        let curve_points = vec![];
        let power_monitoring_must_trip = DERCurveType::new(
            curve_points,
            1,
            crate::v2_1::enumerations::der_unit::DERUnitEnumType::PctMaxW,
        );

        let limit = LimitMaxDischargeType::new(priority, pct_max_discharge_power.clone(), discharge_limit)
            .with_start_time(start_time.clone())
            .with_duration(duration.clone())
            .with_power_monitoring_must_trip(power_monitoring_must_trip.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(limit.priority(), priority);
        assert_eq!(limit.pct_max_discharge_power(), &pct_max_discharge_power);
        assert_eq!(limit.discharge_limit(), discharge_limit);
        assert_eq!(limit.start_time(), Some(&start_time));
        assert_eq!(limit.duration(), Some(&duration));
        assert_eq!(limit.power_monitoring_must_trip(), Some(&power_monitoring_must_trip));
        assert_eq!(limit.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let pct_max_discharge_power1 = Decimal::from_str("80.0").unwrap();
        let discharge_limit1 = 75.0;
        let priority2 = 2;
        let pct_max_discharge_power2 = Decimal::from_str("90.0").unwrap();
        let discharge_limit2 = 85.0;
        let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let duration = Decimal::from_str("3600.0").unwrap();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        // Create a simple DERCurveType for testing
        let curve_points = vec![];
        let power_monitoring_must_trip = DERCurveType::new(
            curve_points,
            1,
            crate::v2_1::enumerations::der_unit::DERUnitEnumType::PctMaxW,
        );

        let mut limit = LimitMaxDischargeType::new(priority1, pct_max_discharge_power1, discharge_limit1);

        limit
            .set_priority(priority2)
            .set_pct_max_discharge_power(pct_max_discharge_power2.clone())
            .set_discharge_limit(discharge_limit2)
            .set_start_time(Some(start_time.clone()))
            .set_duration(Some(duration.clone()))
            .set_power_monitoring_must_trip(Some(power_monitoring_must_trip.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(limit.priority(), priority2);
        assert_eq!(limit.pct_max_discharge_power(), &pct_max_discharge_power2);
        assert_eq!(limit.discharge_limit(), discharge_limit2);
        assert_eq!(limit.start_time(), Some(&start_time));
        assert_eq!(limit.duration(), Some(&duration));
        assert_eq!(limit.power_monitoring_must_trip(), Some(&power_monitoring_must_trip));
        assert_eq!(limit.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        limit
            .set_start_time(None)
            .set_duration(None)
            .set_power_monitoring_must_trip(None)
            .set_custom_data(None);

        assert_eq!(limit.start_time(), None);
        assert_eq!(limit.duration(), None);
        assert_eq!(limit.power_monitoring_must_trip(), None);
        assert_eq!(limit.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid values
        let priority = 1;
        let pct_max_discharge_power = Decimal::from_str("80.0").unwrap();
        let discharge_limit = 75.0;

        let _limit = LimitMaxDischargeType::new(priority, pct_max_discharge_power, discharge_limit);

        // Test with invalid priority (negative)
        let invalid_priority = -1;
        let _limit_invalid_priority = LimitMaxDischargeType::new(invalid_priority, pct_max_discharge_power.clone(), discharge_limit);
        assert!(invalid_priority < 0);

        // Test with invalid pctMaxDischargePower (over 100)
        let invalid_pct_over = Decimal::from_str("101.0").unwrap();
        let _limit_invalid_pct_over = LimitMaxDischargeType::new(priority, invalid_pct_over.clone(), discharge_limit);
        assert!(invalid_pct_over > Decimal::from(100));

        // Test with invalid pctMaxDischargePower (negative)
        let invalid_pct_negative = Decimal::from_str("-1.0").unwrap();
        let _limit_invalid_pct_negative = LimitMaxDischargeType::new(priority, invalid_pct_negative.clone(), discharge_limit);
        assert!(invalid_pct_negative < Decimal::from(0));
    }

    #[test]
    fn test_serialization_deserialization() {
        let priority = 1;
        let pct_max_discharge_power = Decimal::from_str("80.0").unwrap();
        let discharge_limit = 75.0;
        let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let duration = Decimal::from_str("3600.0").unwrap();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let limit = LimitMaxDischargeType::new(priority, pct_max_discharge_power.clone(), discharge_limit)
            .with_start_time(start_time.clone())
            .with_duration(duration.clone())
            .with_custom_data(custom_data.clone());

        // Serialize to JSON
        let serialized = serde_json::to_string(&limit).unwrap();

        // Deserialize back
        let deserialized: LimitMaxDischargeType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(deserialized.priority(), limit.priority());
        assert_eq!(deserialized.pct_max_discharge_power(), limit.pct_max_discharge_power());
        assert_eq!(deserialized.discharge_limit(), limit.discharge_limit());
        assert_eq!(deserialized.start_time(), limit.start_time());
        assert_eq!(deserialized.duration(), limit.duration());
        assert_eq!(deserialized.custom_data().is_some(), limit.custom_data().is_some());
        if let Some(custom_data) = deserialized.custom_data() {
            assert_eq!(custom_data.vendor_id, "VendorX");
        }

        // Verify the deserialized object is valid
        assert!(deserialized.priority() >= 0);
    }
}
