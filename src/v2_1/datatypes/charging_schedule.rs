use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{charging_schedule_period::ChargingSchedulePeriodType, CustomDataType},
    enumerations::ChargingRateUnitEnumType,
};

/// Charging schedule structure defines a list of charging periods, as used in: NotifyEVChargingScheduleRequest and ChargingProfileType.
/// When used in a NotifyEVChargingScheduleRequest only duration and chargingSchedulePeriod are relevant and chargingRateUnit must be 'W'.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Identifies the ChargingSchedule.
    pub id: i32,

    /// Starting point of an absolute schedule or recurring schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<DateTime<Utc>>,

    /// Duration of the charging schedule in seconds.
    /// If the duration is left empty, the last period will continue indefinitely or until end of the transaction
    /// in case startSchedule is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    /// The unit of measure in which limits and setpoints are expressed.
    pub charging_rate_unit: ChargingRateUnitEnumType,

    /// List of charging periods describing the amount of power or current that can be delivered per time interval.
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}

impl ChargingScheduleType {
    /// Creates a new `ChargingScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifies the ChargingSchedule
    /// * `charging_rate_unit` - The unit of measure in which limits and setpoints are expressed
    /// * `charging_schedule_period` - List of charging periods
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingScheduleType` with optional fields set to `None`
    pub fn new(
        id: i32,
        charging_rate_unit: ChargingRateUnitEnumType,
        charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    ) -> Self {
        Self {
            id,
            charging_rate_unit,
            charging_schedule_period,
            custom_data: None,
            start_schedule: None,
            duration: None,
        }
    }

    /// Sets the start schedule.
    ///
    /// # Arguments
    ///
    /// * `start_schedule` - Starting point of an absolute schedule or recurring schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_start_schedule(mut self, start_schedule: DateTime<Utc>) -> Self {
        self.start_schedule = Some(start_schedule);
        self
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the charging schedule in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ID.
    ///
    /// # Returns
    ///
    /// The ID of the charging schedule
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifies the ChargingSchedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the start schedule.
    ///
    /// # Returns
    ///
    /// An optional reference to the start schedule
    pub fn start_schedule(&self) -> Option<&DateTime<Utc>> {
        self.start_schedule.as_ref()
    }

    /// Sets the start schedule.
    ///
    /// # Arguments
    ///
    /// * `start_schedule` - Starting point of an absolute schedule or recurring schedule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_schedule(&mut self, start_schedule: Option<DateTime<Utc>>) -> &mut Self {
        self.start_schedule = start_schedule;
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// An optional duration of the charging schedule in seconds
    pub fn duration(&self) -> Option<i32> {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the charging schedule in seconds, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: Option<i32>) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Gets the charging rate unit.
    ///
    /// # Returns
    ///
    /// The unit of measure in which limits and setpoints are expressed
    pub fn charging_rate_unit(&self) -> &ChargingRateUnitEnumType {
        &self.charging_rate_unit
    }

    /// Sets the charging rate unit.
    ///
    /// # Arguments
    ///
    /// * `charging_rate_unit` - The unit of measure in which limits and setpoints are expressed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_rate_unit(
        &mut self,
        charging_rate_unit: ChargingRateUnitEnumType,
    ) -> &mut Self {
        self.charging_rate_unit = charging_rate_unit;
        self
    }

    /// Gets the charging schedule periods.
    ///
    /// # Returns
    ///
    /// A reference to the list of charging periods
    pub fn charging_schedule_period(&self) -> &Vec<ChargingSchedulePeriodType> {
        &self.charging_schedule_period
    }

    /// Sets the charging schedule periods.
    ///
    /// # Arguments
    ///
    /// * `charging_schedule_period` - List of charging periods
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_schedule_period(
        &mut self,
        charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    ) -> &mut Self {
        self.charging_schedule_period = charging_schedule_period;
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
    /// * `custom_data` - Custom data for this charging schedule, or None to clear
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

    fn create_test_period() -> ChargingSchedulePeriodType {
        ChargingSchedulePeriodType::new_from_f64(0, 16.0)
    }

    #[test]
    fn test_new_charging_schedule() {
        let period = create_test_period();
        let schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::A, vec![period.clone()]);

        assert_eq!(schedule.id(), 1);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::A);
        assert_eq!(schedule.charging_schedule_period().len(), 1);
        assert_eq!(
            schedule.charging_schedule_period()[0].start_period,
            period.start_period
        );
        assert_eq!(schedule.charging_schedule_period()[0].limit, period.limit);
        assert_eq!(schedule.start_schedule(), None);
        assert_eq!(schedule.duration(), None);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let period = create_test_period();
        let custom_data = CustomDataType::new("VendorX".to_string());
        let start_time = Utc::now();

        let schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::W, vec![period.clone()])
                .with_start_schedule(start_time)
                .with_duration(3600)
                .with_custom_data(custom_data.clone());

        assert_eq!(schedule.id(), 1);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::W);
        assert_eq!(schedule.charging_schedule_period().len(), 1);
        assert_eq!(
            schedule.charging_schedule_period()[0].start_period,
            period.start_period
        );
        assert_eq!(schedule.charging_schedule_period()[0].limit, period.limit);
        assert_eq!(schedule.start_schedule(), Some(&start_time));
        assert_eq!(schedule.duration(), Some(3600));
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let period1 = create_test_period();
        let period2 = ChargingSchedulePeriodType::new_from_f64(3600, 32.0);

        let custom_data = CustomDataType::new("VendorX".to_string());
        let start_time = Utc::now();

        let mut schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::A, vec![period1.clone()]);

        schedule
            .set_id(2)
            .set_charging_rate_unit(ChargingRateUnitEnumType::W)
            .set_charging_schedule_period(vec![period1.clone(), period2.clone()])
            .set_start_schedule(Some(start_time))
            .set_duration(Some(7200))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.id(), 2);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::W);
        assert_eq!(schedule.charging_schedule_period().len(), 2);
        assert_eq!(
            schedule.charging_schedule_period()[0].start_period,
            period1.start_period
        );
        assert_eq!(schedule.charging_schedule_period()[0].limit, period1.limit);
        assert_eq!(
            schedule.charging_schedule_period()[1].start_period,
            period2.start_period
        );
        assert_eq!(schedule.charging_schedule_period()[1].limit, period2.limit);
        assert_eq!(schedule.start_schedule(), Some(&start_time));
        assert_eq!(schedule.duration(), Some(7200));
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule
            .set_start_schedule(None)
            .set_duration(None)
            .set_custom_data(None);

        assert_eq!(schedule.start_schedule(), None);
        assert_eq!(schedule.duration(), None);
        assert_eq!(schedule.custom_data(), None);
    }
}
