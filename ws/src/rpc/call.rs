use std::num::ParseIntError;

use super::enums::ActionEnum;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
pub struct Call {
    pub message_type_id: MessageTypeId,
    pub message_id: MessageId,
    pub action: String,
    pub payload: serde_json::Value,
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
        let action = Action::new(action);
        let payload = Payload::new(payload);
        if let (Ok(message_type_id), Ok(message_id), Ok(action), Ok(payload)) =
            (message_id, message_type_id, action, payload)
        {
            return Self {message_type_id.unwrap(), message_id.unwrap(),action.unwrap(), payload.unwrap()}
        } else {
            return Err("fail".to_string()) 
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]

pub struct MessageTypeId {
    pub id: usize,
}

impl MessageTypeId {
    fn new(id: String) -> Result<MessageTypeId, ParseIntError> {
        let id = id.parse::<usize>();
        match id {
            Ok(o) => Ok(Self { id: o }),
            Err(e) => Err(e),
        }
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

pub struct Action {
    kind: ActionEnum,
}

impl Action {
    fn new(action: String) -> Result<Action, String> {
        let action = ActionEnum::from_str(action);
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    data: ActionEnum,
}

impl Payload {
    fn new(data: String) -> Result<Self, String> {
        let payload: ActionEnum = ActionEnum::Authorize();
        Ok(payload)
    }
}
