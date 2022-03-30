use crate::rpc::ocpp_error::OCPPError;
use crate::rpc::request::Request;
use rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationRequest;
use rust_ocpp::v2_0_1::messages::heartbeat::HeartbeatRequest;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Payload {
    /// OCPP Call
    Request(usize, String, String, Value),
    /// OCPP Result
    Response(usize, String, Value),
    /// OCPP Error
    Error(usize, String, String, String, Option<Value>),
}

impl Payload {
    pub fn try_to_request(&self) -> Result<(String, Request), Box<dyn Error + Send + Sync>> {
        if let Self::Request(_, id, action, payload) = self {
            let request = match action.as_str() {
                "BootNotification" => {
                    let data: BootNotificationRequest = serde_json::from_value(payload.clone())?;
                    Request::BootNotificationRequest(data)
                }
                "Heartbeat" => {
                    let data: HeartbeatRequest = serde_json::from_value(payload.clone())?;
                    Request::HeartbeatRequest(data)
                }
                _ => return Err(format!("Unknown request type '{}'", action).into()),
            };

            Ok((id.to_string(), request))
        } else {
            Err("The payload was not an response type".into())
        }
    }
    pub fn try_to_response<T: DeserializeOwned>(
        &self,
    ) -> Result<(String, T), Box<dyn Error + Send + Sync>> {
        if let Self::Response(_, id, payload) = self {
            Ok((id.to_string(), serde_json::from_value(payload.clone())?))
        } else {
            Err("The payload was not an response type".into())
        }
    }
    pub fn try_to_error(
        &self,
    ) -> Result<(String, Box<dyn Error + Send + Sync>), Box<dyn Error + Send + Sync>> {
        if let Self::Error(_, id, code, message, description) = self {
            Ok((
                id.to_string(),
                format!(
                    "{}: {}, {}",
                    code,
                    message,
                    description
                        .as_ref()
                        .map(|v| serde_json::to_string(&v).unwrap())
                        .unwrap_or_else(|| "".to_string())
                )
                .into(),
            ))
        } else {
            Err("The payload was not an error type".into())
        }
    }

    pub fn is_response(&self) -> bool {
        matches!(self, Self::Response(_, _, _))
    }

    pub fn is_request(&self) -> bool {
        matches!(self, Self::Request(_, _, _, _))
    }

    pub fn id(&self) -> &str {
        match self {
            Payload::Request(_, id, _, _) => id.as_str(),
            Payload::Response(_, id, _) => id.as_str(),
            Payload::Error(_, id, _, _, _) => id.as_str(),
        }
    }

    pub fn new_error(id: String, error: OCPPError) -> Self {
        Self::Error(
            4,
            id,
            error.code().to_string(),
            error.description().to_string(),
            error.details().to_owned(),
        )
    }
}

impl<I: Serialize> From<(String, I)> for Payload {
    fn from((id, data): (String, I)) -> Self {
        let value = serde_json::to_value(data).unwrap();
        Self::Response(3, id, value)
    }
}
