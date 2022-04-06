use std::str::FromStr;

use futures::stream::SplitSink;
use log::{info, warn};
use warp::ws::{Message, WebSocket};

use crate::{
    handlers::{call::handle_call, error::handle_error, response::handle_response},
    rpc::{call::Call, call_error::CallError, call_result::CallResult},
    validators::validate::{validate_message_id, validate_message_type_id},
};

/*
    The job of the message_handler is to:
    Validate that:
        1. incoming transmission is of type text
        2. text is deserializeable to jsonValue
        3. json data is of type array
        4. validate that the array is of type Call
    Cast to correct Call type
*/
pub async fn handle_message(msg: Message, tx: &mut SplitSink<WebSocket, Message>) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        warn!("Client sent non-text message");
        handle_error(
            Message::text("Failed to parse incoming message".to_string()),
            tx,
        )
        .await;
        return;
    };

    // We got some text, but is it json?
    let json = if let Ok(v) = serde_json::Value::from_str(msg) {
        v
    } else {
        warn!("Client did not send valid json: {}", msg);
        handle_error(Message::text(format!("Expected json, got {}", msg)), tx).await;
        return;
    };

    // Ok, we got some json, but is it an array?
    if !json.is_array() {
        handle_error(Message::text(format!("Expected array, got: {}", msg)), tx).await;
        return;
    }

    /*
        In OCPP_2_01 the message_type_id is encoded in the 0th field in the
        json array. a message_type_id is either:
        - 2: A Call
        - 3: A CallResult
        - 4: A CallError

        The OCPP_2_0_1 description:
            To identify the type of message one of the following Message Type
            Numbers MUST be used.
            | MessageType | MessageTypeNumber | Description |
            | --- | --- | --- |
            | CALL | 2 | Request message |
            | CALLRESULT | 3 | Response message |
            | CALLERROR | 4 | Error response to a request message |
            When a server receives a message with a Message Type Number not in
            this list, it SHALL ignore the message payload. Each message type
            may have additional required fields.
    */
    let message_type_id = match validate_message_type_id(&json).await {
        Ok(o) => o,
        Err(e) => {
            handle_error(e, tx).await;
            return;
        }
    };

    /*
        The message ID

        The message ID serves to identify a request. A message ID for any CALL
        message MUST be different from all message IDs previously used by the
        same sender for any other CALL messages on the same WebSocket
        connection. A message ID for a CALLRESULT or CALLERROR message MUST be
        equal to that of the CALL  message that the CALLRESULT or CALLERROR
        message is a response to.

        TODO: How do we verify that the message id has not been previously used?
    */
    let message_id = match validate_message_id(&json).await {
        Ok(o) => o,
        Err(e) => {
            handle_error(e, tx).await;
            return;
        }
    };
    info!("Got message_id: {}", message_id);

    // try to deseralize json to a Call, CallResult or CallError
    if message_type_id == 2 {
        // It's a Call
        let call: Call = match serde_json::from_str(&msg.to_string()) {
            Ok(o) => o,
            Err(e) => {
                handle_error(Message::text(e.to_string()), tx).await;
                return;
            }
        };
        handle_call(call, tx).await;
    } else if message_type_id == 3 {
        // It's a CallResult
        let call_result: CallResult = match serde_json::from_str(&msg.to_string()) {
            Ok(o) => o,
            Err(_) => {
                handle_error(Message::text("Could not deserialize CallResult"), tx).await;
                return;
            }
        };
        info!("got {:?}", call_result);
        handle_response(Message::text("Responding to CallResult"), tx).await;
    } else if message_type_id == 4 {
        // It's a CallError
        let call_error: CallError = match serde_json::from_str(&msg.to_string()) {
            Ok(o) => o,
            Err(_) => {
                handle_error(Message::text("Could not deserialize CallResultError"), tx).await;
                return;
            }
        };
        info!("got {:?}", call_error);
        handle_response(Message::text("Responding to CallError"), tx).await;
    };
}
