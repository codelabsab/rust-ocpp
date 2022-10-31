/// Requested availability change in ChangeAvailabilityRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum AvailabilityType {
    /// Charge point is not available for charging.
    Inoperative,
    /// Charge point is available for charging.
    #[default]
    Operative,
}
