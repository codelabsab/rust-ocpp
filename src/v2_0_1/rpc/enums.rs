use std::fmt;

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
    cost_updated::{CostUpdatedRequest, CostUpdatedResponse},
    customer_information::{CustomerInformationRequest, CustomerInformationResponse},
    datatransfer::{DataTransferRequest, DataTransferResponse},
    delete_certificate::{DeleteCertificateRequest, DeleteCertificateResponse},
    firmware_status_notification::{
        FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
    },
    get_15118ev_certificate::{Get15118EVCertificateRequest, Get15118EVCertificateResponse},
};

use super::{call::Call, call_error::CallError, call_result::CallResult};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AuthorizeEnum {
    AuthorizeRequest(AuthorizeRequest),
    AuthorizeResponse(AuthorizeResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BootNotificationEnum {
    BootNotificationRequest(BootNotificationRequest),
    BootNotificationResponse(BootNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CancelReservationEnum {
    CancelReservationRequest(CancelReservationRequest),
    CancelReservationResponse(CancelReservationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CertificateSignedEnum {
    CertificateSignedRequest(CertificateSignedRequest),
    CertificateSignedResponse(CertificateSignedResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChangeAvailabilityEnum {
    ChangeAvailabilityRequest(ChangeAvailabilityRequest),
    ChangeAvailabilityResponse(ChangeAvailabilityResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ClearCacheEnum {
    ClearCacheRequest(ClearCacheRequest),
    ClearCacheResponse(ClearCacheResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ClearChargingProfileEnum {
    ClearChargingProfileRequest(ClearChargingProfileRequest),
    ClearChargingProfileResponse(ClearChargingProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ClearDisplayMessageEnum {
    ClearDisplayMessageRequest(ClearDisplayMessageRequest),
    ClearDisplayMessageResponse(ClearDisplayMessageResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ClearedChargingLimitEnum {
    ClearedChargingLimitRequest(ClearedChargingLimitRequest),
    ClearedChargingLimitResponse(ClearedChargingLimitResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ClearVariableMonitoringEnum {
    ClearVariableMonitoringRequest(ClearVariableMonitoringRequest),
    ClearVariableMonitoringResponse(ClearVariableMonitoringResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CostUpdatedEnum {
    CostUpdatedRequest(CostUpdatedRequest),
    CostUpdatedResponse(CostUpdatedResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CustomerInformationEnum {
    CustomerInformationRequest(CustomerInformationRequest),
    CustomerInformationResponse(CustomerInformationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DataTransferEnum {
    DatatransferRequest(DataTransferRequest),
    DatatransferResponse(DataTransferResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DeleteCertificateEnum {
    DeleteCertificateRequest(DeleteCertificateRequest),
    DeleteCertificateResponse(DeleteCertificateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum FirmwareStatusNotificationEnum {
    FirmwareStatusNotificationRequest(FirmwareStatusNotificationRequest),
    FirmwareStatusNotificationResponse(FirmwareStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Get15118EVCertificateEnum {
    Get15118EVCertificateRequest(Get15118EVCertificateRequest),
    Get15118EVCertificateResponse(Get15118EVCertificateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CallActionEnum {
    Authorize(AuthorizeEnum),
    BootNotification(BootNotificationEnum),
    CancelReservation(CancelReservationEnum),
    CertificateSigned(CertificateSignedEnum),
    ChangeAvailability(ChangeAvailabilityEnum),
    ClearCache(ClearCacheEnum),
    ClearChargingProfile(ClearChargingProfileEnum),
    ClearDisplayMessage(ClearDisplayMessageEnum),
    ClearedChargingLimit(ClearedChargingLimitEnum),
    ClearVariableMonitoring(ClearVariableMonitoringEnum),
    CostUpdated(CostUpdatedEnum),
    CustomerInformation(CustomerInformationEnum),
    Datatransfer(DataTransferEnum),
    DeleteCertificate(DeleteCertificateEnum),
    FirmwareStatusNotification(FirmwareStatusNotificationEnum),
    Get15118EVCertificate(Get15118EVCertificateEnum),
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
