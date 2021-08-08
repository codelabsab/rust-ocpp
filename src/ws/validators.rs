use log::info;
use serde_json::Value;
use warp::ws::Message;

/*
    Tries to parse json[1] as a String
*/
pub async fn validate_message_id(json: &Value) -> Result<(), Message> {
    info!("Validating message_id");
    // try to read the message_id_field
    let read_message_id_field = if let Some(s) = json.get(1) {
        s
    } else {
        return Err(Message::text("Could not read message_id field"));
    };

    // try to extract a string from message_id field
    let _message_id = if let Some(s) = read_message_id_field.as_str() {
        s
    } else {
        return Err(Message::text("Could not parse String from message_id"));
    };
    Ok(())
}

/*
    Validates that the 0'th field in a json array is
    parasble as a message_type_id.
    json[0] must be:
    - parsed to i64
    - value is either 2, 3 or 4
*/
pub async fn validate_message_type_id(json: &Value) -> Result<i64, Message> {
    info!("Validating message_type_id");
    // try to read message_type_id field
    let read_message_type_id_field = if let Some(i) = json.get(0) {
        i
    } else {
        return Err(Message::text("Could not read message_type_id field"));
    };

    // try to extract an i64 from message_type_id field
    let message_type_id = if let Some(i) = read_message_type_id_field.as_i64() {
        i
    } else {
        return Err(Message::text("Could not parse number from message_type_id"));
    };

    // Validate that message_type_id is either 2, 3 or 4 or die
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
pub async fn validate_call(val: Value, message_type_id: &i64) -> Result<(), Message> {
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
