/// Unit in which a charging schedule is defined, as used in: GetCompositeSchedule.req and ChargingSchedule
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ChargingRateUnitType {
    /// Watts (power).
    /// This is the TOTAL allowed charging power.
    /// If used for AC Charging, the phase current should be calculated via: Current per phase = Power / (Line Voltage * Number of
    /// Phases). The "Line Voltage" used in the calculation is not the measured voltage, but the set voltage for the area (hence, 230 of
    /// 110 volt). The "Number of Phases" is the numberPhases from the ChargingSchedulePeriod.
    /// It is usually more convenient to use this for DC charging.
    /// Note that if numberPhases in a ChargingSchedulePeriod is absent, 3 SHALL be assumed.
    W,
    /// Amperes (current).
    /// The amount of Ampere per phase, not the sum of all phases.
    /// It is usually more convenient to use this for AC charging.
    #[default]
    A,
}
