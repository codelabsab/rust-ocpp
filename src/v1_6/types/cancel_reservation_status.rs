/// Status in CancelReservationResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum CancelReservationStatus {
    /// Reservation for the identifier has been cancelled.
    #[default]
    Accepted,
    /// Reservation could not be cancelled, because there is no reservation active for the identifier.
    Rejected,
}
