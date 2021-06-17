use std::{fmt, str::FromStr};

use enum_as_inner::EnumAsInner;

use crate::v2_0_1::core::messages::{
    authorize::{AuthorizeRequest, AuthorizeResponse},
    boot_notification::{BootNotificationRequest, BootNotificationResponse},
    cancel_reservation::{CancelReservationRequest, CancelReservationResponse},
    certificate_signed::{CertificateSignedRequest, CertificateSignedResponse},
    change_availability::{ChangeAvailabilityRequest, ChangeAvailabilityResponse},
    clear_cache::{ClearCacheRequest, ClearCacheResponse},
    clear_charging_profile::{ClearChargingProfileRequest, ClearChargingProfileResponse},
    clear_display_message::{ClearDisplayMessageRequest, ClearDisplayMessageResponse},
    clear_variable_monitoring::{ClearVariableMonitoringRequest, ClearVariableMonitoringResponse},
    cleared_charging_limit::{ClearedChargingLimitRequest, ClearedChargingLimitResponse},
    cost_updated::CostUpdatedRequest,
    customer_information::{CustomerInformationRequest, CustomerInformationResponse},
    datatransfer::{DataTransferRequest, DataTransferResponse},
    delete_certificate::{DeleteCertificateRequest, DeleteCertificateResponse},
    firmware_status_notification::FirmwareStatusNotificationRequest,
    get_15118ev_certificate::{Get15118EVCertificateRequest, Get15118EVCertificateResponse},
    get_base_report::{GetBaseReportRequest, GetBaseReportResponse},
    get_certificate_status::{GetCertificateStatusRequest, GetCertificateStatusResponse},
    get_charging_profile::{GetChargingProfilesRequest, GetChargingProfilesResponse},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub message_type_id: i64,
    pub message_id: String,
    pub action: CallActionTypeEnum,
    pub payload: CallPayloadTypeEnum,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallResult {
    pub message_type_id: i64,
    pub message_id: String,
    pub payload: String,
}

impl CallResult {
    pub fn new(message_type_id: i64, message_id: String, payload: String) -> Self {
        Self {
            message_type_id,
            message_id,
            payload,
        }
    }
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

impl CallError {
    pub fn new(
        message_type_id: i64,
        message_id: String,
        error_code: String,
        error_description: String,
        error_details: String,
    ) -> Self {
        Self {
            message_type_id,
            message_id,
            error_code,
            error_description,
            error_details,
        }
    }
}

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
            "UpdateFirmware" => Ok(CallActionTypeEnum::UpdateFirmware),
            _ => Err(()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, EnumAsInner)]
#[serde(untagged)]
pub enum CallPayloadTypeEnum {
    AuthorizeRequest(AuthorizeRequest),
    AuthorizeResponse(AuthorizeResponse),
    BootNotificationRequest(BootNotificationRequest),
    BootNotificationResponse(BootNotificationResponse),
    CancelReservationRequest(CancelReservationRequest),
    CancelReservationResponse(CancelReservationResponse),
    CertificateSignedRequest(CertificateSignedRequest),
    CertificateSignedResponse(CertificateSignedResponse),
    ChangeAvailabilityRequest(ChangeAvailabilityRequest),
    ChangeAvailabilityResponse(ChangeAvailabilityResponse),
    ClearCacheRequest(ClearCacheRequest),
    ClearCacheResponse(ClearCacheResponse),
    ClearChargingProfileRequest(ClearChargingProfileRequest),
    ClearChargingProfileResponse(ClearChargingProfileResponse),
    ClearDisplayMessageRequest(ClearDisplayMessageRequest),
    ClearDisplayMessageResponse(ClearDisplayMessageResponse),
    ClearedChargingLimitRequest(ClearedChargingLimitRequest),
    ClearedChargingLimitResponse(ClearedChargingLimitResponse),
    ClearVariableMonitoringRequest(ClearVariableMonitoringRequest),
    ClearVariableMonitoringReponse(ClearVariableMonitoringResponse),
    CostUpdatedRequest(CostUpdatedRequest),
    CostUpdatedResponse,
    CustomerInformationRequest(CustomerInformationRequest),
    CustomerInformationResponse(CustomerInformationResponse),
    DataTransferRequest(DataTransferRequest),
    DataTransferResponse(DataTransferResponse),
    DeleteCertificateRequest(DeleteCertificateRequest),
    DeleteCertificateResponse(DeleteCertificateResponse),
    FirmwareStatusNotificationRequest(FirmwareStatusNotificationRequest),
    FirmwareStatusNotificationResponse,
    Get15118EVCertificateRequest(Get15118EVCertificateRequest),
    Get15118EVCertificateResponse(Get15118EVCertificateResponse),
    GetBaseReportRequest(GetBaseReportRequest),
    GetBaseReportResponse(GetBaseReportResponse),
    GetCertificateStatusRequest(GetCertificateStatusRequest),
    GetCertificateStatusResponse(GetCertificateStatusResponse),
    GetChargingProfilesRequest(GetChargingProfilesRequest),
    GetChargingProfilesResponse(GetChargingProfilesResponse),
}

impl fmt::Display for CallPayloadTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
