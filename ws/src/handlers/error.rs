use futures::stream::SplitSink;
use futures::SinkExt;

use log::error;
use log::info;
use warp::ws::{Message, WebSocket};

pub async fn handle_error(error: Message, tx: &mut SplitSink<WebSocket, Message>) {
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
