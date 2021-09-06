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
    get_base_report::{GetBaseReportRequest, GetBaseReportResponse},
    get_certificate_status::{GetCertificateStatusRequest, GetCertificateStatusResponse},
    get_charging_profile::{GetChargingProfilesRequest, GetChargingProfilesResponse},
    get_composite_schedule::{GetCompositeScheduleRequest, GetCompositeScheduleResponse},
    get_display_message::{GetDisplayMessagesRequest, GetDisplayMessagesResponse},
    get_installed_certificate_ids::{
        GetInstalledCertificateIdsRequest, GetInstalledCertificateIdsResponse,
    },
    get_local_list_version::{GetLocalListVersionRequest, GetLocalListVersionResponse},
    get_log::{GetLogRequest, GetLogResponse},
    get_monitoring_report::{GetMonitoringReportRequest, GetMonitoringReportResponse},
    get_report::{GetReportRequest, GetReportResponse},
    get_transaction_status::{GetTransactionStatusRequest, GetTransactionStatusResponse},
    get_variables::{GetVariablesRequest, GetVariablesResponse},
    heartbeat::{HeartbeatRequest, HeartbeatResponse},
    install_certificate::{InstallCertificateRequest, InstallCertificateResponse},
    log_status_notification::{LogStatusNotificationRequest, LogStatusNotificationResponse},
    meter_values::{MeterValuesRequest, MeterValuesResponse},
    notify_charging_limit::{NotifyChargingLimitRequest, NotifyChargingLimitResponse},
    notify_customer_information::{
        NotifyCustomerInformationRequest, NotifyCustomerInformationResponse,
    },
    notify_display_messages::{NotifyDisplayMessagesRequest, NotifyDisplayMessagesResponse},
    notify_ev_charging_needs::{NotifyEVChargingNeedsRequest, NotifyEVChargingNeedsResponse},
    notify_ev_charging_schedule::{
        NotifyEVChargingScheduleRequest, NotifyEVChargingScheduleResponse,
    },
    notify_event::{NotifyEventRequest, NotifyEventResponse},
    notify_monitoring_report::{NotifyMonitoringReportRequest, NotifyMonitoringReportResponse},
    notify_report::{NotifyReportRequest, NotifyReportResponse},
    publish_firmware::{PublishFirmwareRequest, PublishFirmwareResponse},
    publish_firmware_status_notification::{
        PublishFirmwareStatusNotificationRequest, PublishFirmwareStatusNotificationResponse,
    },
    report_charging_profiles::{ReportChargingProfilesRequest, ReportChargingProfilesResponse},
    request_start_transaction::{RequestStartTransactionRequest, RequestStartTransactionResponse},
    request_stop_transaction::{RequestStopTransactionRequest, RequestStopTransactionResponse},
    reservation_status_update::{ReservationStatusUpdateRequest, ReservationStatusUpdateResponse},
    reserve_now::{ReserveNowRequest, ReserveNowResponse},
    reset::{ResetRequest, ResetResponse},
    security_event_notification::{
        SecurityEventNotificationRequest, SecurityEventNotificationResponse,
    },
    send_local_list::{SendLocalListRequest, SendLocalListResponse},
    set_charging_profile::{SetChargingProfileRequest, SetChargingProfileResponse},
    set_display_message::{SetDisplayMessageRequest, SetDisplayMessageResponse},
    set_monitoring_base::{SetMonitoringBaseRequest, SetMonitoringBaseResponse},
    set_monitoring_level::{SetMonitoringLevelRequest, SetMonitoringLevelResponse},
    set_network_profile::{SetNetworkProfileRequest, SetNetworkProfileResponse},
    set_variable_monitoring::{SetVariableMonitoringRequest, SetVariableMonitoringResponse},
    set_variables::{SetVariablesRequest, SetVariablesResponse},
    sign_certificate::{SignCertificateRequest, SignCertificateResponse},
    status_notification::{StatusNotificationRequest, StatusNotificationResponse},
    transaction_event::{TransactionEventRequest, TransactionEventResponse},
    trigger_message::{TriggerMessageRequest, TriggerMessageResponse},
    unlock_connector::{UnlockConnectorRequest, UnlockConnectorResponse},
    unpublish_firmware::{UnpublishFirmwareRequest, UnpublishFirmwareResponse},
    update_firmware::{UpdateFirmwareRequest, UpdateFirmwareResponse},
};

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
pub enum GetBaseReportEnum {
    GetBaseReportRequest(GetBaseReportRequest),
    GetBaseReportResponse(GetBaseReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetCertificateStatusEnum {
    GetCertificateStatusRequest(GetCertificateStatusRequest),
    GetCertificateStatusResponse(GetCertificateStatusResponse),
}

// Is this name correct? Enum is called Singular Profile vs struct that says Profiles in pluralis
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetChargingProfilesEnum {
    GetChargingProfilesRequest(GetChargingProfilesRequest),
    GetChargingProfilesResponse(GetChargingProfilesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetCompositeScheduleEnum {
    GetCompositeScheduleRequest(GetCompositeScheduleRequest),
    GetCompositeScheduleResponse(GetCompositeScheduleResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetDisplayMessagesEnum {
    GetDisplayMessagesRequest(GetDisplayMessagesRequest),
    GetDisplayMessagesResponse(GetDisplayMessagesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetInstalledCertificateIdsEnum {
    GetInstalledCertificateIdsRequest(GetInstalledCertificateIdsRequest),
    GetInstalledCertificateIdsResponse(GetInstalledCertificateIdsResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetLocalListVersionEnum {
    GetLocalListVersionRequest(GetLocalListVersionRequest),
    GetLocalListVersionResponse(GetLocalListVersionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetLogEnum {
    GetLogRequest(GetLogRequest),
    GetLogResponse(GetLogResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetMonitoringReportEnum {
    GetMonitoringReportRequest(GetMonitoringReportRequest),
    GetMonitoringReportResponse(GetMonitoringReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetReportEnum {
    GetReportRequest(GetReportRequest),
    GetReportResponse(GetReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetTransactionStatusEnum {
    GetTransactionStatusRequest(GetTransactionStatusRequest),
    GetTransactionStatusResponse(GetTransactionStatusResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GetVariablesEnum {
    GetVariablesRequest(GetVariablesRequest),
    GetVariablesResponse(GetVariablesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum HeartbeatEnum {
    HeartbeatRequest(HeartbeatRequest),
    HeartbeatResponse(HeartbeatResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum InstallCertificateEnum {
    InstallCertificateRequest(InstallCertificateRequest),
    InstallCertificateResponse(InstallCertificateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum LogStatusNotificationEnum {
    LogStatusNotificationRequest(LogStatusNotificationRequest),
    LogStatusNotificationResponse(LogStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MeterValuesEnum {
    MeterValuesRequest(MeterValuesRequest),
    MeterValuesResponse(MeterValuesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NotifyChargingLimitEnum {
    NotifyChargingLimitRequest(NotifyChargingLimitRequest),
    NotifyChargingLimitResponse(NotifyChargingLimitResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NotifyCustomerInformationEnum {
    NotifyCustomerInformationRequest(NotifyCustomerInformationRequest),
    NotifyCustomerInformationResponse(NotifyCustomerInformationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NotifyDisplayMessagesEnum {
    NotifyDisplayMessagesRequest(NotifyDisplayMessagesRequest),
    NotifyDisplayMessagesResponse(NotifyDisplayMessagesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NotifyEVChargingNeedsEnum {
    NotifyEVChargingNeedsRequest(NotifyEVChargingNeedsRequest),
    NotifyEVChargingNeedsResponse(NotifyEVChargingNeedsResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NotifyEVChargingScheduleEnum {
    NotifyEVChargingScheduleRequest(NotifyEVChargingScheduleRequest),
    NotifyEVChargingScheduleResponse(NotifyEVChargingScheduleResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NotifyEventEnum {
    NotifyEventRequest(NotifyEventRequest),
    NotifyEventResponse(NotifyEventResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NotifyMonitoringReportEnum {
    NotifyMonitoringReportRequest(NotifyMonitoringReportRequest),
    NotifyMonitoringReportResponse(NotifyMonitoringReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum NotifyReportEnum {
    NotifyReportRequest(NotifyReportRequest),
    NotifyReportResponse(NotifyReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PublishFirmwareEnum {
    PublishFirmwareRequest(PublishFirmwareRequest),
    PublishFirmwareResponse(PublishFirmwareResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PublishFirmwareStatusNotificationEnum {
    PublishFirmwareStatusNotificationRequest(PublishFirmwareStatusNotificationRequest),
    PublishFirmwareStatusNotificationResponse(PublishFirmwareStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ReportChargingProfilesEnum {
    ReportChargingProfilesRequest(ReportChargingProfilesRequest),
    ReportChargingProfilesResponse(ReportChargingProfilesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum RequestStartTransactionEnum {
    RequestStartTransactionRequest(RequestStartTransactionRequest),
    RequestStartTransactionResponse(RequestStartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum RequestStopTransactionEnum {
    RequestStopTransactionRequest(RequestStopTransactionRequest),
    RequestStopTransactionResponse(RequestStopTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ReservationStatusUpdateEnum {
    ReservationStatusUpdateRequest(ReservationStatusUpdateRequest),
    ReservationStatusUpdateResponse(ReservationStatusUpdateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ReserveNowEnum {
    ReserveNowRequest(ReserveNowRequest),
    ReserveNowResponse(ReserveNowResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ResetEnum {
    ResetRequest(ResetRequest),
    ResetResponse(ResetResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SecurityEventNotificationEnum {
    SecurityEventNotificationRequest(SecurityEventNotificationRequest),
    SecurityEventNotificationResponse(SecurityEventNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SendLocalListEnum {
    SendLocalListRequest(SendLocalListRequest),
    SendLocalListResponse(SendLocalListResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SetChargingProfileEnum {
    SetChargingProfileRequest(SetChargingProfileRequest),
    SetChargingProfileResponse(SetChargingProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SetDisplayMessageEnum {
    SetDisplayMessageRequest(SetDisplayMessageRequest),
    SetDisplayMessageResponse(SetDisplayMessageResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SetMonitoringBaseEnum {
    SetMonitoringBaseRequest(SetMonitoringBaseRequest),
    SetMonitoringBaseResponse(SetMonitoringBaseResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SetMonitoringLevelEnum {
    SetMonitoringLevelRequest(SetMonitoringLevelRequest),
    SetMonitoringLevelResponse(SetMonitoringLevelResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SetNetworkProfileEnum {
    SetNetworkProfileRequest(SetNetworkProfileRequest),
    SetNetworkProfileResponse(SetNetworkProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SetVariableMonitoringEnum {
    SetVariableMonitoringRequest(SetVariableMonitoringRequest),
    SetVariableMonitoringResponse(SetVariableMonitoringResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SetVariablesEnum {
    SetVariablesRequest(SetVariablesRequest),
    SetVariablesResponse(SetVariablesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SignCertificateEnum {
    SignCertificateRequest(SignCertificateRequest),
    SignCertificateResponse(SignCertificateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum StatusNotificationEnum {
    StatusNotificationRequest(StatusNotificationRequest),
    StatusNotificationResponse(StatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TransactionEventEnum {
    TransactionEventRequest(TransactionEventRequest),
    TransactionEventResponse(TransactionEventResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TriggerMessageEnum {
    TriggerMessageRequest(TriggerMessageRequest),
    TriggerMessageResponse(TriggerMessageResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum UnlockConnectorEnum {
    UnlockConnectorRequest(UnlockConnectorRequest),
    UnlockConnectorResponse(UnlockConnectorResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum UnpublishFirmwareEnum {
    UnpublishFirmwareRequest(UnpublishFirmwareRequest),
    UnpublishFirmwareResponse(UnpublishFirmwareResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum UpdateFirmwareEnum {
    UpdateFirmwareRequest(UpdateFirmwareRequest),
    UpdateFirmwareResponse(UpdateFirmwareResponse),
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
    GetBaseReport(GetBaseReportEnum),
    GetCertificateStatus(GetCertificateStatusEnum),
    GetChargingProfile(GetChargingProfilesEnum),
    GetCompositeSchedule(GetCompositeScheduleEnum),
    GetDisplayMessage(GetDisplayMessagesEnum),
    GetInstalledCertificateIds(GetInstalledCertificateIdsEnum),
    GetLocalListVersion(GetLocalListVersionEnum),
    GetLog(GetLogEnum),
    GetMonitoringReport(GetMonitoringReportEnum),
    GetReport(GetReportEnum),
    GetTransactionStatus(GetTransactionStatusEnum),
    GetVariables(GetVariablesEnum),
    Heartbeat(HeartbeatEnum),
    InstallCertificate(InstallCertificateEnum),
    LogStatusNotification(LogStatusNotificationEnum),
    MeterValues(MeterValuesEnum),
    NotifyChargingLimit(NotifyChargingLimitEnum),
    NotifyCustomerInformation(NotifyCustomerInformationEnum),
    NotifyDisplayMessages(NotifyDisplayMessagesEnum),
    NotifyEVChargingNeeds(NotifyEVChargingNeedsEnum),
    NotifyEVChargingSchedule(NotifyEVChargingScheduleEnum),
    NotifyEvent(NotifyEventEnum),
    NotifyMonitoringReport(NotifyMonitoringReportEnum),
    NotifyReport(NotifyReportEnum),
    PublishFirmware(PublishFirmwareEnum),
    PublishFirmwareStatusNotification(PublishFirmwareStatusNotificationEnum),
    ReportChargingProfiles(ReportChargingProfilesEnum),
    RequestStartTransaction(RequestStartTransactionEnum),
    RequestStopTransaction(RequestStopTransactionEnum),
    ReservationStatusUpdate(ReservationStatusUpdateEnum),
    ReserveNow(ReserveNowEnum),
    Reset(ResetEnum),
    SecurityEventNotification(SecurityEventNotificationEnum),
    SendLocalList(SendLocalListEnum),
    SetChargingProfile(SetChargingProfileEnum),
    SetDisplayMessage(SetDisplayMessageEnum),
    SetMonitoringBase(SetMonitoringBaseEnum),
    SetMonitoringLevel(SetMonitoringLevelEnum),
    SetNetworkProfile(SetNetworkProfileEnum),
    SetVariableMonitoring(SetVariableMonitoringEnum),
    SetVariables(SetVariablesEnum),
    SignCertificate(SignCertificateEnum),
    StatusNotification(StatusNotificationEnum),
    TransactionEvent(TransactionEventEnum),
    TriggerMessage(TriggerMessageEnum),
    UnlockConnector(UnlockConnectorEnum),
    UnpublishFirmware(UnpublishFirmwareEnum),
    UpdateFirmware(UpdateFirmwareEnum),
}

impl fmt::Display for CallActionEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
