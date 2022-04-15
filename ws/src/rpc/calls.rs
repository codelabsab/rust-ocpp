use super::enums::OcppPayloadKindEnum;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
/// A Payload consist of either a Call, a CallResult or a CallError
///
/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
///
/// CallResult: [<MessageTypeId>, "<MessageId>", {<Payload>}]
///
/// CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
pub enum OcppMessageType {
    /// OCPP Call
    Call(usize, String, String, Value),
    /// OCPP Result
    CallResult(usize, String, Value),
    /// OCPP Error
    CallError(usize, String, String, String, Value),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OcppPayload {
    payload: OcppPayloadKindEnum,
}

// impl OCPPPayload {
//     fn new(payload: String) -> Result<Self, String> {
//         let res = serde_json::from_str(&payload);
//         match res {
//             Ok(o) => Ok(Self { payload: o }),
//             _ => Err("Failed to parse".to_string()),
//         }
//     }
// }

type MessageTypeId = usize;
type MessageId = usize;

/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OcppCall {
    pub message_type_id: MessageTypeId,
    pub message_id: MessageId,
    pub action: OcppActionEnum,
    pub payload: OcppPayload,
}

impl OcppCall {
    pub fn new(
        message_type_id: usize,
        message_id: String,
        action: String,
        payload: Value,
    ) -> Result<Self, String> {
        // type cast payload
        let payload = if let Ok(o) = serde_json::from_value(payload) {
            o
        } else {
            return Err("Failed to parse payload".to_string());
        };

        // type cast action
        let action = if let Ok(o) = serde_json::from_str(&action) {
            o
        } else {
            return Err("Failed to parse action".to_string());
        };

        // type cast message_id to usize
        let message_id = if let Ok(o) = message_id.parse::<usize>() {
            o
        } else {
            return Err("Failed to parse message_id".to_string());
        };

        // match message_type_id
        match message_type_id {
            2 | 3 | 4 => {}
            _ => return Err("Failed to parse message_type_id".to_string()),
        }

        return Ok(Self {
            message_type_id,
            message_id,
            action,
            payload,
        });
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// CallResult: [<MessageTypeId>, "<MessageId>", {<Payload>}]
pub struct OcppCallResult {
    pub message_type_id: usize,
    pub message_id: String,
    pub payload: serde_json::Value,
}

impl OcppCallResult {
    pub fn new(message_type_id: usize, message_id: String, payload: serde_json::Value) -> Self {
        Self {
            message_type_id,
            message_id,
            payload,
        }
    }
}

use crate::rpc::enums::OcppActionEnum;
use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
pub struct OcppCallError {
    pub message_type_id: usize,
    pub message_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: Option<Value>,
}

impl OcppCallError {
    pub fn new(
        message_type_id: usize,
        message_id: String,
        error_code: String,
        error_description: String,
        error_details: Option<Value>,
    ) -> Self {
        Self {
            message_type_id,
            message_id,
            error_code,
            error_description,
            error_details,
        }
    }
}
