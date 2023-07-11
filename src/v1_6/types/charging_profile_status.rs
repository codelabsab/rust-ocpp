/// Status returned in response to SetChargingProfileRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ChargingProfileStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// Charge Point indicates that the request is not supported.
    NotSupported,
}
