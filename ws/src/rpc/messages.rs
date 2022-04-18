use std::str::FromStr;

use serde_json::Value;

use crate::rpc::enums::{OcppActionEnum, OcppPayload};

type OcppMessageTypeId = usize;
type OcppMessageId = String;
type OcppErrorCode = String;
type OcppErrorDescription = String;
type OcppErrorDetails = String;

/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OcppCall {
    pub message_type_id: OcppMessageTypeId,
    pub message_id: OcppMessageId,
    pub action: OcppActionEnum,
    pub payload: OcppPayload,
}

impl TryFrom<OcppMessageType> for OcppCall {
    type Error = &'static str;

    fn try_from(msg: OcppMessageType) -> Result<Self, Self::Error> {
        match msg {
            OcppMessageType::Call(message_type_id, message_id, action, payload) => {
                let action = if let Ok(o) = OcppActionEnum::from_str(&action) {
                    o
                } else {
                    return Err("failed");
                };
                let payload: OcppPayload =
                    if let Ok(p) = serde_json::from_value::<OcppPayload>(payload) {
                        p
                    } else {
                        return Err("failed");
                    };
                Ok(OcppCall {
                    message_type_id,
                    message_id,
                    action,
                    payload,
                })
            }
            _ => Err("failed"),
        }
    }
}

impl serde::Serialize for OcppCall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (
            &self.message_type_id,
            &self.message_id,
            &self.action.to_string(),
            &self.payload,
        )
            .serialize(serializer)
    }
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
    Call(usize, String, String, Value),
    /// OCPP Result
    CallResult(usize, String, Value),
    /// OCPP Error
    CallError(usize, String, String, String, Value),
}
