/// Status returned in response to ClearChargingProfileRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ClearChargingProfileStatus {
    /// Request has been accepted and will be executed.
    Accepted,
    /// No Charging Profile(s) were found matching the request.
    Unknown,
}
