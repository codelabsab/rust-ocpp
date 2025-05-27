use serde::{Deserialize, Serialize};

/// The updated reservation status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ReservationUpdateStatusEnumType {
    /// Reservation update has been accepted.
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,

    /// Reservation update has failed.
    #[serde(rename = "Failed")]
    Failed,

    /// Reservation update has been rejected.
    #[serde(rename = "Rejected")]
    Rejected,
}
