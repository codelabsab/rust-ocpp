// #![deny(warnings)]

use std::str::FromStr;

use futures::stream::SplitSink;
use futures::StreamExt;

use ocpp::v2_0_1::rpc::call::Call;
use ocpp::ws::error::error_handler;
use ocpp::ws::response::response_handler;
use ocpp::ws::validators::validate_message_id;
use ocpp::ws::validators::validate_message_type_id;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use serde_json::{self};

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

        How do we veryfy that the message id has not been previously used?
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
        let call: Result<Call, _> = serde_json::from_str(&msg.to_string());
        match call {
            Ok(call) => {
                // TODO: Move to another function here for futher processing
                call_handler(call, tx).await;
            }
            Err(_) => {
                error_handler(Message::text("Could not serialize Call"), tx).await;
                return;
            }
        }
    } else if message_type_id == 3 {
        todo!()
    } else if message_type_id == 4 {
        todo!()
    };

    response_handler(Message::text("Success!"), tx).await;
}

async fn call_handler(call: Call, tx: &mut SplitSink<WebSocket, Message>) {
    /*
    State so far:
        pub struct Call {
            pub message_type_id: i64, // Validated
            pub message_id: String,   // Validated
            pub action: String,       // TODO
            pub payload: String,      // TODO
        }
    */
}
