use serde::{Deserialize, Serialize};

/// Status indicating whether the Charging Station has unlocked the connector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum UnlockStatusEnumType {
    /// Connector has been unlocked.
    #[default]
    #[serde(rename = "Unlocked")]
    Unlocked,

    /// Failed to unlock the connector.
    #[serde(rename = "UnlockFailed")]
    UnlockFailed,

    /// Connector is not unlocked because there is still an ongoing authorized transaction.
    #[serde(rename = "OngoingAuthorizedTransaction")]
    OngoingAuthorizedTransaction,

    /// Connector is unknown.
    #[serde(rename = "UnknownConnector")]
    UnknownConnector,
}
