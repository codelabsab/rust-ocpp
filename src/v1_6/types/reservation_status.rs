/// Status in ReserveNowResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReservationStatus {
    /// Reservation has been made.
    #[default]
    Accepted,
    /// Reservation has not been made, because connectors or specified connector are in a faulted state.
    Faulted,
    /// Reservation has not been made. All connectors or the specified connector are occupied.
    Occupied,
    /// Reservation has not been made. Charge Point is not configured to accept reservations.
    Rejected,
    /// Reservation has not been made, because connectors or specified connector are in an unavailable state.
    Unavailable,
}
