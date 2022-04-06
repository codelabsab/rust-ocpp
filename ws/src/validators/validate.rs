use log::info;
use serde_json::Value;
use warp::ws::Message;

/*
    Try to parse the 1th field in the json array. The message id
    is a type of state.

    The OCPP_2_0_1 protocol states the following:

        The message ID

        The message ID serves to identify a request. A message ID for any CALL
        message MUST be different from all message IDs previously used by the
        same sender for any other CALL messages on the same WebSocket
        connection. A message ID for a CALLRESULT or CALLERROR message MUST be
        equal to that of the CALL  message that the CALLRESULT or CALLERROR
        message is a response to.

    This means that CALLRESULT and CALLERRORS can only answer a matching CALL
    request.

    What happens if a pod dies in the after receiving a CALL but before
    answering it? Will there be a retry or do we need to store state somewhere?

*/
pub async fn validate_message_id(json: &Value) -> Result<String, Message> {
    info!("Validating message_id");
    // try to read the message_id_field
    let read_message_id_field = if let Some(s) = json.get(1) {
        s
    } else {
        return Err(Message::text("Could not read message_id field"));
    };

    // try to extract a string from message_id field
    let message_id = if let Some(s) = read_message_id_field.as_str() {
        s
    } else {
        return Err(Message::text("Could not parse String from message_id"));
    };

    // TODO: Validate that the message_id has not been previously used
    //       by this client
    Ok(message_id.to_string())
}

/*
    To identify the type of message one of the following Message Type Numbers
    MUST be used.

    | MessageType | MessageTypeNumber | Description |
    | --- | --- | --- |
    | CALL | 2 | Request message |
    | CALLRESULT | 3 | Response message |
    | CALLERROR | 4 | Error response to a request message |

    When a server receives a message with a Message Type Number not in this
    list, it SHALL ignore the message payload. Each message type may have
    additional required fields.
*/
pub async fn validate_message_type_id(json: &Value) -> Result<i64, Message> {
    info!("Validating message_type_id");
    // check for message_type_id in the 0th field
    let read_message_type_id_field = if let Some(i) = json.get(0) {
        i.as_i64()
    } else {
        return Err(Message::text("Could not read message_type_id field"));
    };

    // message_type_id field should be an i64
    let message_type_id = if let Some(i) = read_message_type_id_field {
        i
    } else {
        return Err(Message::text("Could not parse number from message_type_id"));
    };

    /*
        Validate that message_type_id is 2, 3 or 4
            2 -> Call
            3 -> CallResult
            4 -> CallError
    */
    match message_type_id {
        2 | 3 | 4 => {
            info!("Valid message_type_id, got {}", message_type_id);
        }
        _ => {
            return Err(Message::text(format!(
                "Invalid message_type_id number, got {}",
                message_type_id
            )));
        }
    };
    Ok(message_type_id)
}

/*
    Validates that json is correct for a Call
*/
pub async fn _validate_call(_val: String, message_type_id: &i64) -> Result<(), Message> {
    /*
        Call example json:
            [2,"19223201","BootNotification",{"reason": "PowerUp","chargingStation": {"model": "SingleSocketCharger","vendorName": "VendorX"}}]

        To validate a Call we need to check that
            - v[0] is i64
            - v[1] is a String with a UID of this session
            - v[2] is a valid CallActionEnumType
            - v[3] is Serializable to the CallActionEnumTypes real type
    */
    if message_type_id == &2 {
        info!("It's a Call");
    } else if message_type_id == &3 {
        info!("It's a CallResult")
    } else {
        info!("It's a CallError")
    };

    /*
        CallResult example json:
            [3,"19223201",{"currentTime": "2013-02-01T20:53:32.486Z","interval": 300,"status": "Accepted"}]

        To validate a CallResult we need to check:
            - v[0] is i64
            - v[1] is a String with a UID of this session
            - v[2] is a Serializable Action
    */

    /*
        CallError example json:
            [4,"162376037","NotSupported","SetDisplayMessageRequest not implemented",{}]

        To validate a CallError we need to check:
            - v[0] is i64
            - v[1] is a String with a UID of this session
            - v[2] is a valid String from the RPC Framework Error Codes table
            - v[3] is a  .description() from the RPC Framework Error Codes table
            - v[4] is a JSON object that describes error details in an undefined way. If there are no error details you MUST fill in an empty object {}.
    */

    // Catch all for now
    Ok(())
}
