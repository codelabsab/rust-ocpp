// #![deny(warnings)]

use std::str::FromStr;

use futures::stream::SplitSink;
use futures::SinkExt;
use futures::StreamExt;

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
        let r = message_parser(msg).await;

        response_handler(Message::text("yes"), &mut tx).await
    }
}

/// Parse incoming data to Message and create the correct Call type
async fn message_parser(msg: Message) -> Result<Vec<Value>, Message> {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        warn!("Client sent non-text message");
        return Err(Message::text(format!("Failed to parse text")));
    };

    // Get json or die
    let v = if let Ok(v) = serde_json::Value::from_str(msg) {
        info!("Got json: {}", v);
        info!("v0: {}", v.get(0).unwrap().is_i64());
        info!("v1: {}", v.get(1).unwrap().is_string());
        info!("v2: {:#?}", v.get(2).unwrap().is_string());
        info!("v3: {}", v.get(3).unwrap().is_object());
        info!("len: {}", v.as_array().unwrap().len());
        v
    } else {
        warn!("Client did not send json");
        return Err(Message::text(format!("Failed to parse json")));
    };

    // validate json is of type array
    let valid_json = if let Some(arr) = v.as_array() {
        arr
    } else {
        return Err(Message::text("Expected json array"));
    };

    // The number of fields should be 3, 4 or 5 depending on if it's
    // a Call, CallResult or CallError
    if !(valid_json.len() >= 3 && valid_json.len() <= 5) {
        return Err(Message::text(format!(
            "Array length {}, expected 3 or 4",
            valid_json.len()
        )));
    };

    info!("valid json length: {}", valid_json.len());

    Ok(valid_json.to_owned())
}

async fn call_type_parser(v: Value) -> Result<CallTypeEnum, Message> {
    info!("Entered call_type_parser");
    let call_type: Result<CallTypeEnum, Message> = match v.get(0) {
        Some(data) => {
            if data.is_number() {
                info!("data.is_number() is true");
                if data.is_i64() {
                    info!("data.is_i64() is true");
                    if data.as_i64().eq(&Some(2)) {
                        info!("data.as_i64().eq(&Some(2)) is true");
                        Ok(CallTypeEnum::Call)
                    } else if data.as_i64().eq(&Some(3)) {
                        info!("data.as_i64().eq(&Some(3)) is true");
                        Ok(CallTypeEnum::CallResult)
                    } else if data.as_i64().eq(&Some(4)) {
                        info!("data.as_i64().eq(&Some(4)) is true");
                        Ok(CallTypeEnum::CallError)
                    } else {
                        error!("data.as_i64().eq(&Some(2, 3 eller 4)) is false");
                        return Err(Message::text("Not a valid Call Type, expected 2, 3 or 4"));
                    }
                } else {
                    error!("data.is_i64() is false");
                    return Err(Message::text("Not a valid i64 number"));
                }
            } else {
                error!("data.is_number() is false");
                return Err(Message::text("Not a number"));
            }
        }
        None => {
            error!("Failed to parse call type");
            return Err(Message::text("Failed to parse call type"));
        }
    };

    match call_type {
        Ok(o) => {
            info!("call_type is Ok");
            Ok(o)
        }
        Err(e) => {
            error!("call_type is Err");
            Err(e)
        }
    }
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
