use std::{fmt, str::FromStr};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub message_type_id: i64,
    pub message_id: String,
    pub action: CallActionTypeEnum,
    pub payload: String,
}

impl Call {
    pub fn new(
        message_type_id: i64,
        message_id: String,
        action: CallActionTypeEnum,
        payload: String,
    ) -> Self {
        Self {
            message_type_id,
            message_id,
            action,
            payload,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallResult {
    pub message_type_id: i64,
    pub message_id: String,
    pub payload: serde_json::Value,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallError {
    pub message_type_id: i64,
    pub message_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum CallTypeEnum {
    Call,
    CallResult,
    CallError,
}

impl fmt::Display for CallTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl FromStr for CallTypeEnum {
    type Err = ();

    fn from_str(input: &str) -> Result<CallTypeEnum, Self::Err> {
        match input {
            "2" => Ok(CallTypeEnum::Call),
            "3" => Ok(CallTypeEnum::CallResult),
            "4" => Ok(CallTypeEnum::CallError),
            _ => Err(()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum CallActionTypeEnum {
    Authorize,
    BootNotification,
    CancelReservation,
    CertificateSigned,
    ChangeAvailability,
    ClearCache,
    ClearChargingProfile,
    ClearDisplayMessage,
    ClearedChargingLimit,
    ClearVariableMonitoring,
    CostUpdated,
    CustomerInformation,
    Datatransfer,
    DeleteCertificate,
    FirmwareStatusNotification,
    Get15118EVCertificate,
    GetBaseReport,
    GetCertificateStatus,
    GetChargingProfile,
    GetCompositeSchedule,
    GetDisplayMessage,
    GetInstalledCertificateIds,
    GetLocalListVersion,
    GetLog,
    GetMonitoringReport,
    GetReport,
    GetTransactionStatus,
    GetVariables,
    Heartbeat,
    InstallCertificate,
    LogStatusNotification,
    MeterValues,
    NotifyChargingLimit,
    NotifyCustomerInformation,
    NotifyDisplayMessages,
    NotifyEVChargingNeeds,
    NotifyEVChargingSchedule,
    NotifyEvent,
    NotifyMonitoringReport,
    NotifyReport,
    PublishFirmware,
    PublishFirmwareStatusNotification,
    ReportChargingProfiles,
    RequestStartTransaction,
    RequestStopTransaction,
    ReservationStatusUpdate,
    ReserveNow,
    Reset,
    SecurityEventNotification,
    SendLocalList,
    SetChargingProfile,
    SetDisplayMessage,
    SetMonitoringBase,
    SetMonitoringLevel,
    SetNetworkProfile,
    SetVariableMonitoring,
    SetVariables,
    SignCertificate,
    StatusNotification,
    TransactionEvent,
    TriggerMessage,
    UnlockConnector,
    UnpublishFirmware,
    UpdateFirmware,
}

impl fmt::Display for CallActionTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for CallActionTypeEnum {
    type Err = ();

    fn from_str(input: &str) -> Result<CallActionTypeEnum, Self::Err> {
        match input {
            "Authorize" => Ok(CallActionTypeEnum::Authorize),
            "BootNotification" => Ok(CallActionTypeEnum::BootNotification),
            "CancelReservation" => Ok(CallActionTypeEnum::CancelReservation),
            "CertificateSigned" => Ok(CallActionTypeEnum::CertificateSigned),
            "ChangeAvailability" => Ok(CallActionTypeEnum::ChangeAvailability),
            "ClearCache" => Ok(CallActionTypeEnum::ClearCache),
            "ClearChargingProfile" => Ok(CallActionTypeEnum::ClearChargingProfile),
            "ClearDisplayMessage" => Ok(CallActionTypeEnum::ClearDisplayMessage),
            "ClearedChargingLimit" => Ok(CallActionTypeEnum::ClearedChargingLimit),
            "ClearVariableMonitoring" => Ok(CallActionTypeEnum::ClearVariableMonitoring),
            "CostUpdated" => Ok(CallActionTypeEnum::CostUpdated),
            "CustomerInformation" => Ok(CallActionTypeEnum::CustomerInformation),
            "Datatransfer" => Ok(CallActionTypeEnum::Datatransfer),
            "DeleteCertificate" => Ok(CallActionTypeEnum::DeleteCertificate),
            "FirmwareStatusNotification" => Ok(CallActionTypeEnum::FirmwareStatusNotification),
            "Get15118EVCertificate" => Ok(CallActionTypeEnum::Get15118EVCertificate),
            "GetBaseReport" => Ok(CallActionTypeEnum::GetBaseReport),
            "GetCertificateStatus" => Ok(CallActionTypeEnum::GetCertificateStatus),
            "GetChargingProfile" => Ok(CallActionTypeEnum::GetChargingProfile),
            "GetCompositeSchedule" => Ok(CallActionTypeEnum::GetCompositeSchedule),
            "GetDisplayMessage" => Ok(CallActionTypeEnum::GetDisplayMessage),
            "GetInstalledCertificateIds" => Ok(CallActionTypeEnum::GetInstalledCertificateIds),
            "GetLocalListVersion" => Ok(CallActionTypeEnum::GetLocalListVersion),
            "GetLog" => Ok(CallActionTypeEnum::GetLog),
            "GetMonitoringReport" => Ok(CallActionTypeEnum::GetMonitoringReport),
            "GetReport" => Ok(CallActionTypeEnum::GetReport),
            "GetTransactionStatus" => Ok(CallActionTypeEnum::GetTransactionStatus),
            "GetVariables" => Ok(CallActionTypeEnum::GetVariables),
            "Heartbeat" => Ok(CallActionTypeEnum::Heartbeat),
            "InstallCertificate" => Ok(CallActionTypeEnum::InstallCertificate),
            "LogStatusNotification" => Ok(CallActionTypeEnum::LogStatusNotification),
            "MeterValues" => Ok(CallActionTypeEnum::MeterValues),
            "NotifyChargingLimit" => Ok(CallActionTypeEnum::NotifyChargingLimit),
            "NotifyCustomerInformation" => Ok(CallActionTypeEnum::NotifyCustomerInformation),
            "NotifyDisplayMessages" => Ok(CallActionTypeEnum::NotifyDisplayMessages),
            "NotifyEVChargingNeeds" => Ok(CallActionTypeEnum::NotifyEVChargingNeeds),
            "NotifyEVChargingSchedule" => Ok(CallActionTypeEnum::NotifyEVChargingSchedule),
            "NotifyEvent" => Ok(CallActionTypeEnum::NotifyEvent),
            "NotifyMonitoringReport" => Ok(CallActionTypeEnum::NotifyMonitoringReport),
            "NotifyReport" => Ok(CallActionTypeEnum::NotifyReport),
            "PublishFirmware" => Ok(CallActionTypeEnum::PublishFirmware),
            "PublishFirmwareStatusNotification" => {
                Ok(CallActionTypeEnum::PublishFirmwareStatusNotification)
            }
            "ReportChargingProfiles" => Ok(CallActionTypeEnum::ReportChargingProfiles),
            "RequestStartTransaction" => Ok(CallActionTypeEnum::RequestStartTransaction),
            "RequestStopTransaction" => Ok(CallActionTypeEnum::RequestStopTransaction),
            "ReservationStatusUpdate" => Ok(CallActionTypeEnum::ReservationStatusUpdate),
            "ReserveNow" => Ok(CallActionTypeEnum::ReserveNow),
            "Reset" => Ok(CallActionTypeEnum::Reset),
            "SecurityEventNotification" => Ok(CallActionTypeEnum::SecurityEventNotification),
            "SendLocalList" => Ok(CallActionTypeEnum::SendLocalList),
            "SetChargingProfile" => Ok(CallActionTypeEnum::SetChargingProfile),
            "SetDisplayMessage" => Ok(CallActionTypeEnum::SetDisplayMessage),
            "SetMonitoringBase" => Ok(CallActionTypeEnum::SetMonitoringBase),
            "SetMonitoringLevel" => Ok(CallActionTypeEnum::SetMonitoringLevel),
            "SetNetworkProfile" => Ok(CallActionTypeEnum::SetNetworkProfile),
            "SetVariableMonitoring" => Ok(CallActionTypeEnum::SetVariableMonitoring),
            "SetVariables" => Ok(CallActionTypeEnum::SetVariables),
            "SignCertificate" => Ok(CallActionTypeEnum::SignCertificate),
            "StatusNotification" => Ok(CallActionTypeEnum::StatusNotification),
            "TransactionEvent" => Ok(CallActionTypeEnum::TransactionEvent),
            "TriggerMessage" => Ok(CallActionTypeEnum::TriggerMessage),
            "UnlockConnector" => Ok(CallActionTypeEnum::UnlockConnector),
            "UnpublishFirmware" => Ok(CallActionTypeEnum::UnpublishFirmware),
            "UpdateFirmwar" => Ok(CallActionTypeEnum::UpdateFirmware),
            _ => Err(()),
        }
    }
}
