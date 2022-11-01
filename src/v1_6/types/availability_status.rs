/// Status returned in response to ChangeAvailabilityRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum AvailabilityStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// Request has been accepted and will be executed when transaction(s) in progress have finished.
    Scheduled,
}
