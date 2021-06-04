/// Electric Vehicle Supply Equipment
/// EVSEType is used by: Common:ComponentType , TriggerMessageRequest , ChangeAvailabilityRequest , TransactionEventRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
}
