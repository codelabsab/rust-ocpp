/// This contains the field definition of the GetLocalListVersionRequest PDU sent by the CSMS to the Charging Station. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {}

/// This contains the field definition of the GetLocalListVersionResponse PDU sent by the Charging Station to CSMS in response to a GetLocalListVersionRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    pub version_number: i64,
}
