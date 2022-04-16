use crate::rpc::enums::{OcppActionEnum, OcppPayload};

type OcppMessageTypeId = usize;
type OcppMessageId = String;
type OcppErrorCode = String;
type OcppErrorDescription = String;
type OcppErrorDetails = String;

/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OcppCall {
    pub message_type_id: OcppMessageTypeId,
    pub message_id: OcppMessageId,
    pub action: OcppActionEnum,
    pub payload: OcppPayload,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
/// CallResult: [<MessageTypeId>, "<MessageId>", {<Payload>}]
pub struct OcppCallResult {
    pub message_type_id: OcppMessageTypeId,
    pub message_id: OcppMessageId,
    pub payload: OcppPayload,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
/// CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
pub struct OcppCallError {
    pub message_type_id: OcppMessageTypeId,
    pub message_id: OcppMessageId,
    pub error_code: OcppErrorCode,
    pub error_description: OcppErrorDescription,
    pub error_details: Option<OcppErrorDetails>,
}

/// A Payload consist of either a Call, a CallResult or a CallError
///
/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
/// CallResult: [<MessageTypeId>, "<MessageId>", {<Payload>}]
/// CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppMessageType {
    /// OCPP Call
    Call(OcppCall),
    /// OCPP Result
    CallResult(OcppCallResult),
    /// OCPP Error
    CallError(OcppCallError),
}
