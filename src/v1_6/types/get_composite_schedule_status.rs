/// Status returned in response to GetCompositeScheduleRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum GetCompositeScheduleStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
}
