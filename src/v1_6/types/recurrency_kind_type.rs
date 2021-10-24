/// Type of recurrence of a charging profile, as used in ChargingProfile.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum RecurrencyKindType {
    ///  The schedule restarts every 24 hours, at the same time as in the startSchedule.
    Daily,
    /// The schedule restarts every 7 days, at the same time and day-of-the-week as in the startSchedule.
    Weekly,
}
