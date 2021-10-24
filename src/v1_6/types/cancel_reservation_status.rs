/// Status in CancelReservationResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum CancelReservationStatus {
    /// Reservation for the identifier has been cancelled.
    Accepted,
    /// Reservation could not be cancelled, because there is no reservation active for the identifier.
    Rejected,
}
