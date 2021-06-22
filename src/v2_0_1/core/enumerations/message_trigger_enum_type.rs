#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MessageTriggerEnumType {
    BootNotification,
    LogStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    SignChargingStationCertificate,
    SignV2GCertificate,
    StatusNotification,
    TransactionEvent,
    SignCombinedCertificate,
    PublishFirmwareStatusNotification,
}
