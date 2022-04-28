#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum UnlockStatus {
    /// # From OCPP Specification
    /// Connector has successfully been unlocked.
    Unlocked,
    /// # From OCPP Specification
    /// Failed to unlock the connector: The Charge Point has tried to unlock the connector and has
    ///detected that the connector is still locked or the unlock mechanism failed.
    UnlockFailed,
    /// # From OCPP Specification
    /// Charge Point has no connector lock, or ConnectorId is unknown.
    NotSupported,
}
