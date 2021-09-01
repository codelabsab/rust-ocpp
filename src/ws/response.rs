use futures::stream::SplitSink;
use futures::SinkExt;

use log::error;
use log::info;
use warp::ws::{Message, WebSocket};

pub async fn response_handler(response: Message, tx: &mut SplitSink<WebSocket, Message>) {
    // TODO: add some logging
    info!("Entered response_handler");
    match tx.send(response).await {
        Ok(_) => (),
        Err(e) => {
            error!("websocket error: Could not send response. Error({})", e);
        }
    }
}
