// #![deny(warnings)]

use std::str::FromStr;

use futures::stream::SplitSink;
use futures::SinkExt;
use futures::StreamExt;

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
        error_handler(Message::text(format!("Expected json, got {}", msg)), tx).await;
        return;
    };

    // Did we get a json array?
    if !json.is_array() {
        error_handler(Message::text(format!("Expected array, got: {}", msg)), tx).await;
        return;
    }

    // validate message_type_id field is an i64 of either 2, 3 or 4
    let message_type_id = validate_message_type_id(&json).await;
    match message_type_id {
        Ok(_) => {}
        Err(e) => {
            error_handler(e, tx).await;
            return;
        }
    };

    // validate message_id field
    let message_id = validate_message_id(&json).await;
    match message_id {
        Ok(_) => {}
        Err(e) => {
            error_handler(e, tx).await;
            return;
        }
    };

    // We have valid json! We have a message_type_id! Let's try to build a Call, CallResult or CallError
    //let call = call_type_builder(json).await;

    //let call = call_type_builder(json, message_type_id, message_id).await;
    response_handler(Message::text("Got a something"), tx).await;
}

// async fn call_type_builder(json: Value) -> Result<CallTypeEnum, Message> {
//     // We expect that uid has been validated already

//     // json[0] has already been validate so we can unwrap
//     let message_type_id = json[0].as_i64().unwrap();

//     match message_type_id {
//         2 => {}
//         3 => {}
//         4 => {}
//         _ => {}
//     }

//     // Catch for now all
//     Ok(CallTypeEnum::Call(Call::new(
//         1,
//         "1".to_string(),
//         CallActionTypeEnum::Authorize,
//         "lala".to_string(),
//     )))
// }

/*
    Builds a Call
    We know that message_type_id is 2 and mu
*/
//async fn build_call(val: Value) -> Result<Call, Message> {}

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
