#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum EventNotificationEnumType {
    HardWiredNotification,
    HardWiredMonitor,
    PreconfiguredMonitor,
    CustomMonitor,
}
