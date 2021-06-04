#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum EventNotificationEnumType {
    HardWiredNotification,
    HardWiredMonitor,
    PreconfiguredMonitor,
    CustomMonitor,
}
