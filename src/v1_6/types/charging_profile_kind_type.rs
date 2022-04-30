#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ChargingProfileKindType {
    /// Schedule periods are relative to a fixed point in time defined in the schedule.
    Absolute,
    ///  The schedule restarts periodically at the first schedule period.
    Recurring,
    /// Schedule periods are relative to a situation-specific start point (such as the start of a Transaction) that is determined by the charge point.
    Relative,
}
