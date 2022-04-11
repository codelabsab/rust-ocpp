use futures::stream::SplitSink;
use log::info;
use warp::ws::{Message, WebSocket};

use crate::handlers::{error::handle_error, response::handle_response};

use crate::rpc::call::Call;
use crate::rpc::call_error::CallError;
use crate::rpc::call_result::CallResult;
use crate::rpc::enums::ActionEnum;

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
pub async fn handle_call(call: Call, tx: &mut SplitSink<WebSocket, Message>) {
    info!("Got {:?} and {:?}", call, tx);
    let c: Result<ActionEnum, serde_json::Error> = serde_json::from_value(call.payload);
    match c {
        Ok(o) => {
            let res = serde_json::to_string(&o).unwrap();
            handle_response(Message::text(res), tx).await;
        }
        Err(_) => handle_error(Message::text("Could not transform json value to Call"), tx).await,
    };
}
pub async fn handle_callresult(call: CallResult, tx: &mut SplitSink<WebSocket, Message>) {
    info!("Got {:?} and {:?}", call, tx);
    handle_response(Message::text("call result".to_string()), tx).await;
}
pub async fn handle_callerror(call: CallError, tx: &mut SplitSink<WebSocket, Message>) {
    info!("Got {:?} and {:?}", call, tx);
    handle_response(Message::text("call error".to_string()), tx).await;
}
