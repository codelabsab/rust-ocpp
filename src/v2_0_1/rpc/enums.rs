use std::fmt;

use super::{call::Call, call_error::CallError, call_result::CallResult};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum CallTypeEnum {
    Call(Call),
    CallResult(CallResult),
    CallError(CallError),
    None,
}

impl fmt::Display for CallTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum AuthorizeEnum {
    AuthorizeRequest,
    AuthorizeResponse,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum BootNotificationEnum {
    BootNotificationRequest,
    BootNotificationResponse,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum CallActionEnum {
    Authorize(AuthorizeEnum),
    BootNotification(BootNotificationEnum),
    // CancelReservation,
    // CertificateSigned,
    // ChangeAvailability,
    // ClearCache,
    // ClearChargingProfile,
    // ClearDisplayMessage,
    // ClearedChargingLimit,
    // ClearVariableMonitoring,
    // CostUpdated,
    // CustomerInformation,
    // Datatransfer,
    // DeleteCertificate,
    // FirmwareStatusNotification,
    // Get15118EVCertificate,
    // GetBaseReport,
    // GetCertificateStatus,
    // GetChargingProfile,
    // GetCompositeSchedule,
    // GetDisplayMessage,
    // GetInstalledCertificateIds,
    // GetLocalListVersion,
    // GetLog,
    // GetMonitoringReport,
    // GetReport,
    // GetTransactionStatus,
    // GetVariables,
    // Heartbeat,
    // InstallCertificate,
    // LogStatusNotification,
    // MeterValues,
    // NotifyChargingLimit,
    // NotifyCustomerInformation,
    // NotifyDisplayMessages,
    // NotifyEVChargingNeeds,
    // NotifyEVChargingSchedule,
    // NotifyEvent,
    // NotifyMonitoringReport,
    // NotifyReport,
    // PublishFirmware,
    // PublishFirmwareStatusNotification,
    // ReportChargingProfiles,
    // RequestStartTransaction,
    // RequestStopTransaction,
    // ReservationStatusUpdate,
    // ReserveNow,
    // Reset,
    // SecurityEventNotification,
    // SendLocalList,
    // SetChargingProfile,
    // SetDisplayMessage,
    // SetMonitoringBase,
    // SetMonitoringLevel,
    // SetNetworkProfile,
    // SetVariableMonitoring,
    // SetVariables,
    // SignCertificate,
    // StatusNotification,
    // TransactionEvent,
    // TriggerMessage,
    // UnlockConnector,
    // UnpublishFirmware,
    // UpdateFirmware,
}

impl fmt::Display for CallActionEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
