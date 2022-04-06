use std::fmt;

use rust_ocpp::v2_0_1::messages::authorize::AuthorizeRequest;
use rust_ocpp::v2_0_1::messages::authorize::AuthorizeResponse;
use rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationRequest;
use rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationResponse;
use rust_ocpp::v2_0_1::messages::cancel_reservation::CancelReservationRequest;
use rust_ocpp::v2_0_1::messages::cancel_reservation::CancelReservationResponse;
use rust_ocpp::v2_0_1::messages::certificate_signed::CertificateSignedRequest;
use rust_ocpp::v2_0_1::messages::certificate_signed::CertificateSignedResponse;
use rust_ocpp::v2_0_1::messages::change_availability::ChangeAvailabilityRequest;
use rust_ocpp::v2_0_1::messages::change_availability::ChangeAvailabilityResponse;
use rust_ocpp::v2_0_1::messages::clear_cache::ClearCacheRequest;
use rust_ocpp::v2_0_1::messages::clear_cache::ClearCacheResponse;
use rust_ocpp::v2_0_1::messages::clear_charging_profile::ClearChargingProfileRequest;
use rust_ocpp::v2_0_1::messages::clear_charging_profile::ClearChargingProfileResponse;
use rust_ocpp::v2_0_1::messages::clear_display_message::ClearDisplayMessageRequest;
use rust_ocpp::v2_0_1::messages::clear_display_message::ClearDisplayMessageResponse;
use rust_ocpp::v2_0_1::messages::clear_variable_monitoring::ClearVariableMonitoringRequest;
use rust_ocpp::v2_0_1::messages::clear_variable_monitoring::ClearVariableMonitoringResponse;
use rust_ocpp::v2_0_1::messages::cleared_charging_limit::ClearedChargingLimitRequest;
use rust_ocpp::v2_0_1::messages::cleared_charging_limit::ClearedChargingLimitResponse;
use rust_ocpp::v2_0_1::messages::cost_updated::CostUpdatedRequest;
use rust_ocpp::v2_0_1::messages::cost_updated::CostUpdatedResponse;
use rust_ocpp::v2_0_1::messages::customer_information::CustomerInformationRequest;
use rust_ocpp::v2_0_1::messages::customer_information::CustomerInformationResponse;
use rust_ocpp::v2_0_1::messages::datatransfer::DataTransferRequest;
use rust_ocpp::v2_0_1::messages::datatransfer::DataTransferResponse;
use rust_ocpp::v2_0_1::messages::delete_certificate::DeleteCertificateRequest;
use rust_ocpp::v2_0_1::messages::delete_certificate::DeleteCertificateResponse;
use rust_ocpp::v2_0_1::messages::firmware_status_notification::FirmwareStatusNotificationRequest;
use rust_ocpp::v2_0_1::messages::firmware_status_notification::FirmwareStatusNotificationResponse;
use rust_ocpp::v2_0_1::messages::get_15118ev_certificate::Get15118EVCertificateRequest;
use rust_ocpp::v2_0_1::messages::get_15118ev_certificate::Get15118EVCertificateResponse;
use rust_ocpp::v2_0_1::messages::get_base_report::GetBaseReportRequest;
use rust_ocpp::v2_0_1::messages::get_base_report::GetBaseReportResponse;
use rust_ocpp::v2_0_1::messages::get_certificate_status::GetCertificateStatusRequest;
use rust_ocpp::v2_0_1::messages::get_certificate_status::GetCertificateStatusResponse;
use rust_ocpp::v2_0_1::messages::get_charging_profiles::GetChargingProfilesRequest;
use rust_ocpp::v2_0_1::messages::get_charging_profiles::GetChargingProfilesResponse;
use rust_ocpp::v2_0_1::messages::get_composite_schedule::GetCompositeScheduleRequest;
use rust_ocpp::v2_0_1::messages::get_composite_schedule::GetCompositeScheduleResponse;
use rust_ocpp::v2_0_1::messages::get_display_message::GetDisplayMessagesRequest;
use rust_ocpp::v2_0_1::messages::get_display_message::GetDisplayMessagesResponse;
use rust_ocpp::v2_0_1::messages::get_installed_certificate_ids::GetInstalledCertificateIdsRequest;
use rust_ocpp::v2_0_1::messages::get_installed_certificate_ids::GetInstalledCertificateIdsResponse;
use rust_ocpp::v2_0_1::messages::get_local_list_version::GetLocalListVersionRequest;
use rust_ocpp::v2_0_1::messages::get_local_list_version::GetLocalListVersionResponse;
use rust_ocpp::v2_0_1::messages::get_log::GetLogRequest;
use rust_ocpp::v2_0_1::messages::get_log::GetLogResponse;
use rust_ocpp::v2_0_1::messages::get_monitoring_report::GetMonitoringReportRequest;
use rust_ocpp::v2_0_1::messages::get_monitoring_report::GetMonitoringReportResponse;
use rust_ocpp::v2_0_1::messages::get_report::GetReportRequest;
use rust_ocpp::v2_0_1::messages::get_report::GetReportResponse;
use rust_ocpp::v2_0_1::messages::get_transaction_status::GetTransactionStatusRequest;
use rust_ocpp::v2_0_1::messages::get_transaction_status::GetTransactionStatusResponse;
use rust_ocpp::v2_0_1::messages::get_variables::GetVariablesRequest;
use rust_ocpp::v2_0_1::messages::get_variables::GetVariablesResponse;
use rust_ocpp::v2_0_1::messages::heartbeat::HeartbeatRequest;
use rust_ocpp::v2_0_1::messages::heartbeat::HeartbeatResponse;
use rust_ocpp::v2_0_1::messages::install_certificate::InstallCertificateRequest;
use rust_ocpp::v2_0_1::messages::install_certificate::InstallCertificateResponse;
use rust_ocpp::v2_0_1::messages::log_status_notification::LogStatusNotificationRequest;
use rust_ocpp::v2_0_1::messages::log_status_notification::LogStatusNotificationResponse;
use rust_ocpp::v2_0_1::messages::meter_values::MeterValuesRequest;
use rust_ocpp::v2_0_1::messages::meter_values::MeterValuesResponse;
use rust_ocpp::v2_0_1::messages::notify_charging_limit::NotifyChargingLimitRequest;
use rust_ocpp::v2_0_1::messages::notify_charging_limit::NotifyChargingLimitResponse;
use rust_ocpp::v2_0_1::messages::notify_customer_information::NotifyCustomerInformationRequest;
use rust_ocpp::v2_0_1::messages::notify_customer_information::NotifyCustomerInformationResponse;
use rust_ocpp::v2_0_1::messages::notify_display_messages::NotifyDisplayMessagesRequest;
use rust_ocpp::v2_0_1::messages::notify_display_messages::NotifyDisplayMessagesResponse;
use rust_ocpp::v2_0_1::messages::notify_ev_charging_needs::NotifyEVChargingNeedsRequest;
use rust_ocpp::v2_0_1::messages::notify_ev_charging_needs::NotifyEVChargingNeedsResponse;
use rust_ocpp::v2_0_1::messages::notify_ev_charging_schedule::NotifyEVChargingScheduleRequest;
use rust_ocpp::v2_0_1::messages::notify_ev_charging_schedule::NotifyEVChargingScheduleResponse;
use rust_ocpp::v2_0_1::messages::notify_event::NotifyEventRequest;
use rust_ocpp::v2_0_1::messages::notify_event::NotifyEventResponse;
use rust_ocpp::v2_0_1::messages::notify_monitoring_report::NotifyMonitoringReportRequest;
use rust_ocpp::v2_0_1::messages::notify_monitoring_report::NotifyMonitoringReportResponse;
use rust_ocpp::v2_0_1::messages::notify_report::NotifyReportRequest;
use rust_ocpp::v2_0_1::messages::notify_report::NotifyReportResponse;
use rust_ocpp::v2_0_1::messages::publish_firmware::PublishFirmwareRequest;
use rust_ocpp::v2_0_1::messages::publish_firmware::PublishFirmwareResponse;
use rust_ocpp::v2_0_1::messages::publish_firmware_status_notification::PublishFirmwareStatusNotificationRequest;
use rust_ocpp::v2_0_1::messages::publish_firmware_status_notification::PublishFirmwareStatusNotificationResponse;
use rust_ocpp::v2_0_1::messages::report_charging_profiles::ReportChargingProfilesRequest;
use rust_ocpp::v2_0_1::messages::report_charging_profiles::ReportChargingProfilesResponse;
use rust_ocpp::v2_0_1::messages::request_start_transaction::RequestStartTransactionRequest;
use rust_ocpp::v2_0_1::messages::request_start_transaction::RequestStartTransactionResponse;
use rust_ocpp::v2_0_1::messages::request_stop_transaction::RequestStopTransactionRequest;
use rust_ocpp::v2_0_1::messages::request_stop_transaction::RequestStopTransactionResponse;
use rust_ocpp::v2_0_1::messages::reservation_status_update::ReservationStatusUpdateRequest;
use rust_ocpp::v2_0_1::messages::reservation_status_update::ReservationStatusUpdateResponse;
use rust_ocpp::v2_0_1::messages::reserve_now::ReserveNowRequest;
use rust_ocpp::v2_0_1::messages::reserve_now::ReserveNowResponse;
use rust_ocpp::v2_0_1::messages::reset::ResetRequest;
use rust_ocpp::v2_0_1::messages::reset::ResetResponse;
use rust_ocpp::v2_0_1::messages::security_event_notification::SecurityEventNotificationRequest;
use rust_ocpp::v2_0_1::messages::security_event_notification::SecurityEventNotificationResponse;
use rust_ocpp::v2_0_1::messages::send_local_list::SendLocalListRequest;
use rust_ocpp::v2_0_1::messages::send_local_list::SendLocalListResponse;
use rust_ocpp::v2_0_1::messages::set_charging_profile::SetChargingProfileRequest;
use rust_ocpp::v2_0_1::messages::set_charging_profile::SetChargingProfileResponse;
use rust_ocpp::v2_0_1::messages::set_display_message::SetDisplayMessageRequest;
use rust_ocpp::v2_0_1::messages::set_display_message::SetDisplayMessageResponse;
use rust_ocpp::v2_0_1::messages::set_monitoring_base::SetMonitoringBaseRequest;
use rust_ocpp::v2_0_1::messages::set_monitoring_base::SetMonitoringBaseResponse;
use rust_ocpp::v2_0_1::messages::set_monitoring_level::SetMonitoringLevelRequest;
use rust_ocpp::v2_0_1::messages::set_monitoring_level::SetMonitoringLevelResponse;
use rust_ocpp::v2_0_1::messages::set_network_profile::SetNetworkProfileRequest;
use rust_ocpp::v2_0_1::messages::set_network_profile::SetNetworkProfileResponse;
use rust_ocpp::v2_0_1::messages::set_variable_monitoring::SetVariableMonitoringRequest;
use rust_ocpp::v2_0_1::messages::set_variable_monitoring::SetVariableMonitoringResponse;
use rust_ocpp::v2_0_1::messages::set_variables::SetVariablesRequest;
use rust_ocpp::v2_0_1::messages::set_variables::SetVariablesResponse;
use rust_ocpp::v2_0_1::messages::sign_certificate::SignCertificateRequest;
use rust_ocpp::v2_0_1::messages::sign_certificate::SignCertificateResponse;
use rust_ocpp::v2_0_1::messages::status_notification::StatusNotificationRequest;
use rust_ocpp::v2_0_1::messages::status_notification::StatusNotificationResponse;
use rust_ocpp::v2_0_1::messages::transaction_event::TransactionEventRequest;
use rust_ocpp::v2_0_1::messages::transaction_event::TransactionEventResponse;
use rust_ocpp::v2_0_1::messages::trigger_message::TriggerMessageRequest;
use rust_ocpp::v2_0_1::messages::trigger_message::TriggerMessageResponse;
use rust_ocpp::v2_0_1::messages::unlock_connector::UnlockConnectorRequest;
use rust_ocpp::v2_0_1::messages::unlock_connector::UnlockConnectorResponse;
use rust_ocpp::v2_0_1::messages::unpublish_firmware::UnpublishFirmwareRequest;
use rust_ocpp::v2_0_1::messages::unpublish_firmware::UnpublishFirmwareResponse;
use rust_ocpp::v2_0_1::messages::update_firmware::UpdateFirmwareRequest;
use rust_ocpp::v2_0_1::messages::update_firmware::UpdateFirmwareResponse;

use super::call::Call;
use super::call_error::CallError;
use super::call_result::CallResult;

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
    GetInstalledCertificateIdsResponse(Box<GetInstalledCertificateIdsResponse>),
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

pub enum CallTypeEnum {
    Call(Call),
    CallResult(CallResult),
    CallError(CallError),
    None,
}
