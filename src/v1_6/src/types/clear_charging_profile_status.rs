/// Status returned in response to ClearChargingProfileRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ClearChargingProfileStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// No Charging Profile(s) were found matching the request.
    Unknown,
}
