/// Reason for stopping a transaction in StopTransactionRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Reason {
    /// The transaction was stopped because of the authorization status in a StartTransaction.conf
    DeAuthorized,
    /// Emergency stop button was used.
    EmergencyStop,
    /// disconnecting of cable, vehicle moved away from inductive charge unit.
    EVDisconnected,
    /// A hard reset command was received.
    HardReset,
    /// Stopped locally on request of the user at the Charge Point. This is a regular termination of a transaction. Examples: presenting an RFID tag, pressing a button to stop.
    Local,
    /// Any other reason.
    Other,
    /// Complete loss of power.
    PowerLoss,
    /// A locally initiated reset/reboot occurred. (for instance watchdog kicked in)
    Reboot,
    /// Stopped remotely on request of the user. This is a regular termination of a transaction. Examples: termination using a smartphone app, exceeding a (non local) prepaid credit.
    Remote,
    /// A soft reset command was received.
    SoftReset,
    /// Central System sent an Unlock Connector command.
    UnlockCommand,
}
