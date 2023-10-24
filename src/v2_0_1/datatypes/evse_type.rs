/// Electric Vehicle Supply Equipment
/// EVSEType is used by: Common:ComponentType , TriggerMessageRequest , ChangeAvailabilityRequest , TransactionEventRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}
