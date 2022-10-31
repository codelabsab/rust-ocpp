use crate::v1_6::types::{MessageTrigger, TriggerMessageStatus};

/// This contains the field definition of the TriggerMessage.req PDU sent by the Central System to the Charge Point. See also Trigger Message
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageRequest {
    /// Required.
    pub requested_message: MessageTrigger,
    /// Optional. Only filled in when request applies to a specific connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<u64>,
}

/// This contains the field definition of the TriggerMessage.req PDU sent by the Central System to the Charge Point. See also Trigger Message
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageResponse {
    /// Required. Indicates whether the Charge Point will send the requested notification or not.
    pub status: TriggerMessageStatus,
}
