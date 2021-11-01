//! CostUpdated
use validator::Validate;

/// CostUpdatedRequest, sent by the CSMS to the Charging Station.
///
/// With this request the CSMS can send the current cost of a transaction to a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedRequest {
    /// Current total cost, based on the information known by the CSMS, of the transaction including taxes. In the currency configured with the configuration Variable: [Currency]
    pub total_cost: f64,
    /// Transaction Id of the transaction the current cost are asked for.
    #[validate(length(min = 0, max = 36))]
    pub transaction_id: String,
}

/// CostUpdatedResponse, sent by the Charging Station to the CSMS in response to CostUpdatedRequest.
///
/// No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedResponse {
    // No fields are defined.
}
