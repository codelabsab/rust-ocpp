// #![deny(warnings)]

use std::str::FromStr;

use futures::stream::SplitSink;
use futures::SinkExt;
use futures::StreamExt;

use ocpp::v2_0_1::rpc::call::Call;
use ocpp::v2_0_1::rpc::call::CallActionTypeEnum;
use ocpp::v2_0_1::rpc::call::CallError;
use ocpp::v2_0_1::rpc::call::CallResult;
use ocpp::v2_0_1::rpc::call::CallTypeEnum;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use serde_json::{self, Value};

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

async fn connection_handler(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    while let Some(result) = rx.next().await {
        info!("rx.next(): message recieved");
        let msg = match result {
            Ok(msg) => msg,
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
        2. text is valid json
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

    // Get json or die
    let json = if let Ok(v) = serde_json::Value::from_str(msg) {
        v
    } else {
        warn!("Client did not send valid json: {}", msg);
        error_handler(
            Message::text(format!("Client did not send valid json: {}", msg)),
            tx,
        )
        .await;
        return;
    };

    // Did we get a json array?
    if !json.is_array() {
        error_handler(Message::text(format!("Expected array, got: {}", msg)), tx).await;
        return;
    }

    // Can we cast it to a Call?
    let call = call_type_builder(json).await;
}

async fn call_type_builder(val: Value) -> Result<CallTypeEnum, Message> {
    // Does field message_type_id exist?
    let message_type_id_field = if let Some(v) = val.get(0) {
        v
    } else {
        return Err(Message::text("Could not read field message_type_id"));
    };

    // Is message_type_id a number?
    let message_type_id = if let Some(n) = message_type_id_field.as_i64() {
        n
    } else {
        return Err(Message::text("Field message_type_id is not a number"));
    };

    // Validate that message_type_id is either 2, 3 or 4!
    match message_type_id {
        2 | 3 | 4 => {}
        _ => return Err(Message::text("Invalid message_type_id number")),
    };

    if message_type_id == 2 {
        // It's a Call
        info!("It's a Call");
    } else if message_type_id == 3 {
        // It's a CallResult
        info!("It's a CallResult");
    } else if message_type_id == 4 {
        // It's a CallError
        info!("It's a CallError");
    }

    // CatchAll
    Ok(CallTypeEnum::Call(Call::new(
        1,
        "1".to_string(),
        CallActionTypeEnum::Authorize,
        "lala".to_string(),
    )))
}

async fn response_handler(response: Message, tx: &mut SplitSink<WebSocket, Message>) {
    // TODO: add some logging
    info!("Entered response_handler");
    match tx.send(response).await {
        Ok(_) => (),
        Err(e) => {
            error!("websocket error: Could not send response. Error({})", e);
        }
    }
}

async fn error_handler(error: Message, tx: &mut SplitSink<WebSocket, Message>) {
    info!("Entered error_handler");
    match tx.send(error).await {
        Ok(_) => (),
        Err(e) => {
            error!(
                "websocket error: Could not send error response. Error: ({})",
                e
            );
        }
    }
}
