//! GetLocalListVersion

/// GetLocalListVersionRequest, sent by the CSMS to the Charging Station. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {}

/// GetLocalListVersionResponse, sent by the Charging Station to CSMS in response to a GetLocalListVersionRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    /// This contains the current version number of thelocal authorization list in the Charging Station
    pub version_number: i32,
}
