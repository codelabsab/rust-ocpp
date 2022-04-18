use crate::{
    handlers::message::handle_message,
    rpc::{
        enums::OcppActionEnum,
        messages::{OcppCall, OcppMessageType},
    },
};
use rust_ocpp::v2_0_1::datatypes::charging_station_type::ChargingStationType;
use rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationRequest;
use serde_json::{self, Error};
extern crate pretty_env_logger;
extern crate tokio;
use futures::StreamExt;
use warp::{ws::Message, Filter};

fn mock_handle_connection(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Copy {
    warp::ws().map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|websocket| async {
            let (mut tx, mut rx) = websocket.split();
            while let Some(result) = rx.next().await {
                let msg = match result {
                    Ok(msg) => msg, // We got a real message
                    Err(_) => {
                        break;
                    }
                };

                handle_message(msg, &mut tx).await;
            }
        })
    })
}

#[tokio::test]
async fn ws_bootnotificationrequest_test() {
    // mock a test client
    let mut client = warp::test::ws()
        .handshake(mock_handle_connection())
        .await
        .expect("handshake");

    // Setup our test message that the client will send
    let req = r#"[2,"19223201","BootNotification",{"reason":"PowerUp","chargingStation":{"model":"SingleSocketCharger","vendorName":"VendorX"}}]"#;

    // client sends message
    client.send(Message::text(req)).await;

    // receive sent message or die
    let res = client.recv().await.expect("Failed test");

    // convert to str and json
    let res = res.to_str().unwrap();

    let json_res: serde_json::Value = serde_json::from_str(&res).unwrap();

    // // some test cases to match response to
    let reason = "PowerUp";
    let charging_station = ChargingStationType {
        serial_number: None,
        model: "SingleSocketCharger".to_string(),
        vendor_name: "VendorX".to_string(),
        firmware_version: None,
        modem: None,
    };
    let charging_station_json = serde_json::to_string(&charging_station).unwrap();

    // // cast string to real BootNotificationRequest struct
    println!("{:#?}", res);
    let bnr: Result<OcppMessageType, Error> = serde_json::from_str::<OcppMessageType>(&res);

    match bnr {
        Ok(ocpp_message_type) => match ocpp_message_type {
            OcppMessageType::Call(_, _, _, _) => {
                let call: Result<OcppCall, _> = ocpp_message_type.try_into();
                match call {
                    Ok(ok_call) => {
                        // Do some more testing
                        assert_eq!(ok_call.action, OcppActionEnum::BootNotification);
                        assert_eq!(ok_call.message_type_id, 2);
                        assert_eq!(serde_json::to_string(&ok_call).unwrap(), req);
                    }
                    _ => {
                        panic!("Failed to parse Call")
                    }
                };
            }
            OcppMessageType::CallResult(message_type_id, message_id, payload) => {
                todo!()
            }
            OcppMessageType::CallError(
                message_type_id,
                message_id,
                error_code,
                error_description,
                error_details,
            ) => {
                todo!()
            }
        },
        Err(_) => {
            panic!("Failed to parse Call")
        }
    };
}
