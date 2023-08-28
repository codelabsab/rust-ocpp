#[cfg(feature = "std")]
use validator::Validate;

use crate::v1_6::types::{ChargingProfile, RemoteStartStopStatus};

/// This contains the field definitions of the RemoteStartTransactionRequest PDU sent to Charge Point by Central System. See also Remote Start Transaction
#[cfg_attr(feature="std", derive(Validate))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransactionRequest<'a, const CHARGING_SCHEDULE_SIZE: usize = { crate::CHARGING_SCHEDULE_SIZE }> {
    /// Optional. Number of the connector on which to start the transaction. connectorId SHALL be > 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<u64>,
    /// Required. The identifier that Charge Point must use to start a transaction.
    #[cfg_attr(feature="std", validate(length(min = 1, max = 20)))]
    pub id_tag: &'a str, // IdToken, should this be a type?
    /// Optional. Charging Profile to be used by the Charge Point for the requested transaction. ChargingProfilePurpose MUST be set to TxProfile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfile<CHARGING_SCHEDULE_SIZE>>,
}

/// This contains the field definitions of the RemoteStartTransaction.conf PDU sent from Charge Point to Central System. See also Remote Start Transaction
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransactionResponse {
    // Required. Status indicating whether Charge Point accepts the request to start a transaction.
    pub status: RemoteStartStopStatus,
}
