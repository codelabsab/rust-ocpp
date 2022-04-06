use rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationRequest;
use rust_ocpp::v2_0_1::messages::heartbeat::HeartbeatRequest;

#[derive(Debug)]
pub enum Request {
    BootNotificationRequest(BootNotificationRequest),
    HeartbeatRequest(HeartbeatRequest),
}
