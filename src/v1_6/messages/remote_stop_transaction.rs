use crate::v1_6::types::RemoteStartStopStatus;

/// This contains the field definitions of the RemoteStopTransactionRequest PDU sent to Charge Point by Central System. See also Remote Stop Transaction
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionRequest {
    /// Required. The identifier of the transaction which Charge Point is requested to stop.
    pub transaction_id: i64,
}

/// This contains the field definitions of the RemoteStopTransactionResponse PDU sent from Charge Point to Central System. See also Remote Stop Transaction
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionResponse {
    // Required. Status indicating whether Charge Point accepts the request to stop a transaction.
    pub status: RemoteStartStopStatus,
}
