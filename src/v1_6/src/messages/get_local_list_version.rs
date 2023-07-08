/// This contains the field definition of the GetLocalListVersion.req PDU sent by the Central System to the Charge Point. See also Get Local List Version
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {
    // No fields are defined.
}

/// This contains the field definition of the GetDiagnostics.conf PDU sent by the Charge Point to the Central System in response to a GetDiagnosticsRequest PDU. See also Get Diagnostics
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    /// Required. This contains the current version number of the local authorization list in the Charge Point.
    pub list_version: i64,
}
