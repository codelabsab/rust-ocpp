use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
/// A Payload consist of either a Call, a CallResult or a CallError
///
/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
///
/// CallResult: [<MessageTypeId>, "<MessageId>", {<Payload>}]
///
/// CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
pub enum OCPPMessage {
    /// OCPP Call
    Request(usize, String, String, Value),
    /// OCPP Result
    Response(usize, String, Value),
    /// OCPP Error
    Error(usize, String, String, String, Option<Value>),
}
