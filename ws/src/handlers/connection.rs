use futures::StreamExt;
use log::{error, info};
use warp::ws::WebSocket;

use crate::handlers::message::handle_message;

extern crate pretty_env_logger;
/*
    All connection begin at the handle_connection, this is the first function
    that runs after an http upgrade to websockets.
*/

pub async fn handle_connection(ws: WebSocket) {
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

        handle_message(msg, &mut tx).await;
    }
}
