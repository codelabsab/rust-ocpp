use crate::handlers::message::handle_message;
use ocpp::v2_0_1::core::datatypes::charging_station_type::ChargingStationType;
use serde_json;
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
    // test client
    let mut client = warp::test::ws()
        .handshake(mock_handle_connection())
        .await
        .expect("handshake");

    // we need to send something
    let bootnotificationrequest = r#"
    [
        2,
        "19223201",
        "BootNotification",
        {"reason": "PowerUp",
        "chargingStation": {
            "model": "SingleSocketCharger",
            "vendorName": "VendorX"}
        }
    ]"#;

    // send message
    client.send(Message::text(bootnotificationrequest)).await;

    // recieve sent message or die
    let res = client.recv().await.expect("Failed test");

    // convert to str and json
    let res = res.to_str().unwrap();
    let json_res: serde_json::Value = serde_json::from_str(res).unwrap();

    // some test cases to match response to
    let reason = "PowerUp";
    let charging_station = ChargingStationType {
        serial_number: None,
        model: "SingleSocketCharger".to_string(),
        vendor_name: "VendorX".to_string(),
        firmware_version: None,
        modem: None,
    };
    let charging_station_json = serde_json::to_string(&charging_station).unwrap();

    // actual tests
    assert_eq!(reason, json_res["reason"]);
    assert_eq!(
        charging_station_json,
        json_res["chargingStation"].to_string()
    );
}