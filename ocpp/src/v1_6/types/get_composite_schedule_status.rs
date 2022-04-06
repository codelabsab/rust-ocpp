/// Status returned in response to GetCompositeScheduleRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum GetCompositeScheduleStatus {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
}
