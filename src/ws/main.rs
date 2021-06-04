// #![deny(warnings)]

use std::str::FromStr;

use futures::stream::SplitSink;
use futures::SinkExt;
use futures::StreamExt;

use ocpp::v2_0_1::rpc::call::CallTypeEnum;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use serde_json::{self, Value};

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
        println!("connection_handler looping");
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error({})", e);
                break;
            }
        };
        let r = message_parser(msg).await;
        println!("message_parser completed");
        match r {
            Ok(o) => {
                println!("response_handler is Ok");
                response_handler(o, &mut tx).await
            }
            Err(e) => {
                println!("response_handler is Err");
                error_handler(e, &mut tx).await
            }
        }
    }
}

/// Parse incoming data to Message and create the correct Call type
async fn message_parser(msg: Message) -> Result<Message, Message> {
    // Skip any non-Text messages...
    println!("Entered message_parser");
    let err = Message::text(format!("Failed to parse"));
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return Err(err);
    };

    let v = serde_json::Value::from_str(msg);

    // get json or die
    if v.is_err() {
        return Err(Message::text("Failed to parse json"));
    }

    // unwrap
    println!("unwrapping value");
    let v = v.unwrap();

    println!("Calling parse_call_type");
    let parse_call_type = call_type_parser(v).await;

    match parse_call_type {
        Ok(o) => Ok(Message::text(o.to_string())),
        Err(e) => Err(e),
    }
}

async fn call_type_parser(v: Value) -> Result<CallTypeEnum, Message> {
    println!("Entered call_type_parser");
    let call_type: Result<CallTypeEnum, Message> = match v.get(0) {
        Some(data) => {
            if data.is_number() {
                println!("data.is_number() is true");
                if data.is_i64() {
                    println!("data.is_i64() is true");
                    if data.as_i64().eq(&Some(2)) {
                        println!("data.as_i64().eq(&Some(2)) is true");
                        Ok(CallTypeEnum::Call)
                    } else if data.as_i64().eq(&Some(3)) {
                        println!("data.as_i64().eq(&Some(3)) is true");
                        Ok(CallTypeEnum::CallResult)
                    } else if data.as_i64().eq(&Some(4)) {
                        println!("data.as_i64().eq(&Some(4)) is true");
                        Ok(CallTypeEnum::CallError)
                    } else {
                        println!("data.as_i64().eq(&Some(2, 3 eller 4)) is false");
                        return Err(Message::text("Not a valid Call Type, expected 2, 3 or 4"));
                    }
                } else {
                    println!("data.is_i64() is false");
                    return Err(Message::text("Not a valid i64 number"));
                }
            } else {
                println!("data.is_number() is false");
                return Err(Message::text("Not a number"));
            }
        }
        None => {
            println!("Failed to parse call type");
            return Err(Message::text("Failed to parse call type"));
        }
    };

    match call_type {
        Ok(o) => {
            println!("call_type is Ok");
            Ok(o)
        }
        Err(e) => {
            println!("call_type is Err");
            Err(e)
        }
    }
}

async fn response_handler(response: Message, tx: &mut SplitSink<WebSocket, Message>) {
    // TODO: add some logging
    println!("Entered response_handler");
    match tx.send(response).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("websocket error: Could not send response. Error({})", e);
        }
    }
}

async fn error_handler(error: Message, tx: &mut SplitSink<WebSocket, Message>) {
    println!("Entered error_handler");
    match tx.send(error).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!(
                "websocket error: Could not send error response. Error: ({})",
                e
            );
        }
    }
}
