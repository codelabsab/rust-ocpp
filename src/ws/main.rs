// #![deny(warnings)]

use std::str::FromStr;

use futures::stream::SplitSink;
use futures::StreamExt;

use ocpp::v2_0_1::rpc::call::Call;
use ocpp::v2_0_1::rpc::call_error::CallError;
use ocpp::v2_0_1::rpc::call_result::CallResult;
use ocpp::v2_0_1::rpc::enums::CallActionEnum;
use ocpp::ws::error::error_handler;
use ocpp::ws::response::response_handler;
use ocpp::ws::validators::validate_message_id;
use ocpp::ws::validators::validate_message_type_id;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use serde_json;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| connection_handler(socket)));

    warp::serve(routes).run(([127, 0, 0, 1], 3040)).await;
}

/*
    All connection begin at the connection handler, this is the first function
    that runs after an http upgrade to websockets.
*/
async fn connection_handler(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    while let Some(result) = rx.next().await {
        info!("rx.next(): message recieved");
        let msg = match result {
            Ok(msg) => msg, // We got a real message
            Err(e) => {
                error!("websocket error({})", e);
                break;
            }
        };

        message_handler(msg, &mut tx).await;
    }
}

/*
    The job of the message_handler is to:
    Validate that:
        1. incoming transmission is of type text
        2. text is deserializeable to jsonValue
        3. json data is of type array
        4. validate that the array is of type Call
    Cast to correct Call type
*/
async fn message_handler(msg: Message, tx: &mut SplitSink<WebSocket, Message>) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        warn!("Client sent non-text message");
        error_handler(
            Message::text(format!("Failed to parse incoming message")),
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
        error_handler(Message::text(format!("Expected json, got {}", msg)), tx).await;
        return;
    };

    // Ok, we got some json, but is it an array?
    if !json.is_array() {
        error_handler(Message::text(format!("Expected array, got: {}", msg)), tx).await;
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
            error_handler(e, tx).await;
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

        TODO: How do we veryfy that the message id has not been previously used?
    */
    let message_id = match validate_message_id(&json).await {
        Ok(o) => o,
        Err(e) => {
            error_handler(e, tx).await;
            return;
        }
    };

    // try to deseralize json to a Call, CallResult or CallError
    if message_type_id == 2 {
        // It's a Call
        let call: Call = match serde_json::from_str(&msg.to_string()) {
            Ok(o) => o,
            Err(e) => {
                error_handler(Message::text(e.to_string()), tx).await;
                return;
            }
        };
        call_handler(call, tx).await;
    } else if message_type_id == 3 {
        // It's a CallResult
        let call_result: CallResult = match serde_json::from_str(&msg.to_string()) {
            Ok(o) => o,
            Err(_) => {
                error_handler(Message::text("Could not deserialize CallResult"), tx).await;
                return;
            }
        };
        call_result_handler(call_result, tx).await;
    } else if message_type_id == 4 {
        // It's a CallError
        let call_error: CallError = match serde_json::from_str(&msg.to_string()) {
            Ok(o) => o,
            Err(_) => {
                error_handler(Message::text("Could not deserialize CallResultError"), tx).await;
                return;
            }
        };
        call_error_handler(call_error, tx).await;
    };

    response_handler(Message::text("Done, exiting"), tx).await;
}

async fn call_handler(call: Call, tx: &mut SplitSink<WebSocket, Message>) {
    /*
    State so far:
        pub struct Call {
            pub message_type_id: i64, // OK by validate_message_type_id() in caller
            pub message_id: String,   // OK by validate_message_id() in caller
            pub action: String,       // TODO:
                                      // Check which action it is and map it to
                                         an enum of that type. Example:
                                         BootNotification should map to a
                                         BootNotificationEnum that contains
                                         BootNotificationRequest and
                                         BootNotificationResponse

            pub payload: String,      // TODO: Try to cast the payload to
                                               the corresponding enum from
                                               "action"
        }
    */
    let call_type: CallActionEnum = match serde_json::from_value(call.payload) {
        Ok(o) => o,
        Err(e) => {
            error_handler(Message::text(e.to_string()), tx).await;
            return;
        }
    };
    info!("Got a {:?}", call_type);
    response_handler(Message::text(format!("Got {}", call_type)), tx).await;
}

async fn call_result_handler(call: CallResult, tx: &mut SplitSink<WebSocket, Message>) {
    todo!()
}

async fn call_error_handler(call: CallError, tx: &mut SplitSink<WebSocket, Message>) {
    todo!()
}
