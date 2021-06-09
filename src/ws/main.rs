// #![deny(warnings)]

use std::str::FromStr;

use futures::stream::SplitSink;
use futures::SinkExt;
use futures::StreamExt;

use ocpp::v2_0_1::rpc::call::Call;
use ocpp::v2_0_1::rpc::call::CallActionTypeEnum;
use ocpp::v2_0_1::rpc::call::CallError;
use ocpp::v2_0_1::rpc::call::CallResult;
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

        let parsed_message = message_parser(msg).await;
        match parsed_message {
            Ok(o) => {
                if o.get(0).is_some() {
                    if o.get(0).unwrap().as_i64().is_some() {
                        if o.get(0).unwrap().as_i64().unwrap().eq(&2) {
                            let call = build_call(o).await;
                        } else if o.get(0).unwrap().as_i64().unwrap().eq(&3) {
                            let call = build_call_result(o).await;
                        } else if o.get(0).unwrap().as_i64().unwrap().eq(&4) {
                            let call = build_call_error(o).await;
                        }
                    }
                }
            }
            Err(e) => error_handler(e, &mut tx).await,
        }

        response_handler(Message::text("yes"), &mut tx).await
    }
}

async fn build_call(v: Vec<Value>) -> Result<Call, Message> {
    let message_type_id = v.get(0).unwrap().as_i64().unwrap();
    let message_id = v.get(1).unwrap().to_string();
    let action = v.get(2).unwrap().to_string();
    let payload = v.get(3).unwrap().to_string();
    let call = Call::new(
        message_type_id,
        message_id,
        CallActionTypeEnum::from_str(&action).unwrap(),
        payload,
    );
    Ok(call)
}
async fn build_call_result(v: Vec<Value>) -> Result<CallResult, Message> {
    let message_type_id = v.get(0).unwrap().as_i64().unwrap();
    let message_id = v.get(1).unwrap().to_string();
    let payload = v.get(2).unwrap().to_string();
    let callresult = CallResult::new(message_type_id, message_id, payload);
    Ok(callresult)
}
async fn build_call_error(v: Vec<Value>) -> Result<CallError, Message> {
    let message_type_id = v.get(0).unwrap().as_i64().unwrap();
    let message_id = v.get(1).unwrap().to_string();
    let error_code = v.get(2).unwrap().to_string();
    let error_description = v.get(3).unwrap().to_string();
    let error_details = v.get(4).unwrap().to_string();
    let callerror = CallError::new(
        message_type_id,
        message_id,
        error_code,
        error_description,
        error_details,
    );
    Ok(callerror)
}

/// Parse incoming data to Message and return a vector
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

    match valid_json.len() {
        // if len is 3, 4 or 5
        3 | 4 | 5 => Ok(valid_json.to_owned()),
        _ => Err(Message::text("Wrong number of fields")),
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
