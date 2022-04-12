use std::{num::ParseIntError, str::FromStr};

use super::enums::{ActionEnum, PayloadKindEnum};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageTypeId {
    pub id: usize,
}

impl MessageTypeId {
    fn new(id: String) -> Result<MessageTypeId, String> {
        let id = id.parse::<usize>();
        let id = match id {
            Ok(o) => match o {
                2 | 3 | 4 => return Ok(Self { id: o }),
                _ => return Err("Failed to parse Call type".to_string()),
            },
            Err(e) => return Err("Failed to parse int from Call type".to_string()),
        };
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageId {
    id: usize,
}

impl MessageId {
    fn new(id: String) -> Result<MessageId, ParseIntError> {
        let id = id.parse::<usize>();
        // should probably validate that it's an increment of the previous message here
        match id {
            Ok(o) => Ok(Self { id: o }),
            Err(e) => Err(e),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    data: PayloadKindEnum,
}

impl Payload {
    fn new(payload: String) -> Result<Self, String> {
        let res = serde_json::from_str(&payload);
        match res {
            Ok(o) => Ok(Self { data: o }),
            _ => Err("Failed to parse".to_string()),
        }
    }
}

/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub message_type_id: MessageTypeId,
    pub message_id: MessageId,
    pub action: ActionEnum,
    pub payload: Payload,
}

impl Call {
    pub fn new(
        message_type_id: String,
        message_id: String,
        action: String,
        payload: String,
    ) -> Result<Call, String> {
        let message_type_id = MessageTypeId::new(message_type_id);
        let message_id = MessageId::new(message_id);
        let action = ActionEnum::from_str(&action);
        let payload = Payload::new(payload); // should not unwrap() here
        if let (Ok(message_id), Ok(message_type_id), Ok(action), Ok(payload)) =
            (message_id, message_type_id, action, payload)
        {
            return Ok(Self {
                message_type_id,
                message_id,
                action,
                payload,
            });
        } else {
            return Err("Failed to parse Call".to_string());
        }
    }
}
