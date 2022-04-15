// use serde_json::Value;
// use strum_macros::Display;
//
// pub struct OCPPError {
//     code: CallErrorCode,
//     description: String,
//     details: Option<Value>,
// }
//
// impl OCPPError {
//     pub fn new(code: CallErrorCode, description: &str) -> Self {
//         Self {
//             code,
//             description: description.to_string(),
//             details: None,
//         }
//     }
//
//     pub fn new_with_details(code: CallErrorCode, description: &str, details: Value) -> Self {
//         Self {
//             code,
//             description: description.to_string(),
//             details: Some(details),
//         }
//     }
//
//     pub fn code(&self) -> &CallErrorCode {
//         &self.code
//     }
//
//     pub fn description(&self) -> &str {
//         &self.description
//     }
//
//     pub fn details(&self) -> &Option<Value> {
//         &self.details
//     }
// }
//
// #[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Display)]
// pub enum CallErrorCode {
//     FormatViolation,
//     GenericError,
//     InternalError,
//     MessageTypeNotSupported,
//     NotImplemented,
//     NotSupported,
//     OccurrenceConstraintViolation,
//     PropertyConstraintViolation,
//     ProtocolError,
//     RpcFrameworkError,
//     SecurityError,
//     TypeConstraintViolation,
// }
