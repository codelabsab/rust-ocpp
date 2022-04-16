use std::fmt;
use std::str::FromStr;
use strum_macros::Display;

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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum AuthorizeKind {
    Request(AuthorizeRequest),
    Response(AuthorizeResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum BootNotificationKind {
    Request(BootNotificationRequest),
    Response(BootNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum CancelReservationKind {
    Request(CancelReservationRequest),
    Response(CancelReservationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum CertificateSignedKind {
    Request(CertificateSignedRequest),
    Response(CertificateSignedResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ChangeAvailabilityKind {
    Request(ChangeAvailabilityRequest),
    Response(ChangeAvailabilityResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearCacheKind {
    Request(ClearCacheRequest),
    Response(ClearCacheResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearChargingProfileKind {
    Request(ClearChargingProfileRequest),
    Response(ClearChargingProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearDisplayMessageKind {
    Request(ClearDisplayMessageRequest),
    Response(ClearDisplayMessageResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearedChargingLimitKind {
    Request(ClearedChargingLimitRequest),
    Response(ClearedChargingLimitResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ClearVariableMonitoringKind {
    Request(ClearVariableMonitoringRequest),
    Response(ClearVariableMonitoringResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum CostUpdatedKind {
    Request(CostUpdatedRequest),
    Response(CostUpdatedResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum CustomerInformationKind {
    Request(CustomerInformationRequest),
    Response(CustomerInformationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum DataTransferKind {
    Request(DataTransferRequest),
    Response(DataTransferResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum DeleteCertificateKind {
    Request(DeleteCertificateRequest),
    Response(DeleteCertificateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum FirmwareStatusNotificationKind {
    Request(FirmwareStatusNotificationRequest),
    Response(FirmwareStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum Get15118EVCertificateKind {
    Request(Get15118EVCertificateRequest),
    Response(Get15118EVCertificateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetBaseReportKind {
    Request(GetBaseReportRequest),
    Response(GetBaseReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetCertificateStatusKind {
    Request(GetCertificateStatusRequest),
    Response(GetCertificateStatusResponse),
}

// Is this name correct? Enum is called Singular Profile vs struct that says Profiles in pluralis
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetChargingProfilesKind {
    Request(GetChargingProfilesRequest),
    Response(GetChargingProfilesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetCompositeScheduleKind {
    Request(GetCompositeScheduleRequest),
    Response(GetCompositeScheduleResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetDisplayMessagesKind {
    Request(GetDisplayMessagesRequest),
    Response(GetDisplayMessagesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetInstalledCertificateIdsKind {
    Request(GetInstalledCertificateIdsRequest),
    Response(Box<GetInstalledCertificateIdsResponse>),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetLocalListVersionKind {
    Request(GetLocalListVersionRequest),
    Response(GetLocalListVersionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetLogKind {
    Request(GetLogRequest),
    Response(GetLogResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetMonitoringReportKind {
    Request(GetMonitoringReportRequest),
    Response(GetMonitoringReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetReportKind {
    Request(GetReportRequest),
    Response(GetReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetTransactionStatusKind {
    Request(GetTransactionStatusRequest),
    Response(GetTransactionStatusResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum GetVariablesKind {
    Request(GetVariablesRequest),
    Response(GetVariablesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum HeartbeatKind {
    Request(HeartbeatRequest),
    Response(HeartbeatResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum InstallCertificateKind {
    Request(InstallCertificateRequest),
    Response(InstallCertificateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum LogStatusNotificationKind {
    Request(LogStatusNotificationRequest),
    Response(LogStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum MeterValuesKind {
    Request(MeterValuesRequest),
    Response(MeterValuesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum NotifyChargingLimitKind {
    Request(NotifyChargingLimitRequest),
    Response(NotifyChargingLimitResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum NotifyCustomerInformationKind {
    Request(NotifyCustomerInformationRequest),
    Response(NotifyCustomerInformationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum NotifyDisplayMessagesKind {
    Request(NotifyDisplayMessagesRequest),
    Response(NotifyDisplayMessagesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum NotifyEVChargingNeedsKind {
    Request(NotifyEVChargingNeedsRequest),
    Response(NotifyEVChargingNeedsResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum NotifyEVChargingScheduleKind {
    Request(NotifyEVChargingScheduleRequest),
    Response(NotifyEVChargingScheduleResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum NotifyEventKind {
    Request(NotifyEventRequest),
    Response(NotifyEventResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum NotifyMonitoringReportKind {
    Request(NotifyMonitoringReportRequest),
    Response(NotifyMonitoringReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum NotifyReportKind {
    Request(NotifyReportRequest),
    Response(NotifyReportResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum PublishFirmwareKind {
    Request(PublishFirmwareRequest),
    Response(PublishFirmwareResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum PublishFirmwareStatusNotificationKind {
    Request(PublishFirmwareStatusNotificationRequest),
    Response(PublishFirmwareStatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ReportChargingProfilesKind {
    Request(ReportChargingProfilesRequest),
    Response(ReportChargingProfilesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum RequestStartTransactionKind {
    Request(RequestStartTransactionRequest),
    Response(RequestStartTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum RequestStopTransactionKind {
    Request(RequestStopTransactionRequest),
    Response(RequestStopTransactionResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ReservationStatusUpdateKind {
    Request(ReservationStatusUpdateRequest),
    Response(ReservationStatusUpdateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ReserveNowKind {
    Request(ReserveNowRequest),
    Response(ReserveNowResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum ResetKind {
    Request(ResetRequest),
    Response(ResetResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SecurityEventNotificationKind {
    Request(SecurityEventNotificationRequest),
    Response(SecurityEventNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SendLocalListKind {
    Request(SendLocalListRequest),
    Response(SendLocalListResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetChargingProfileKind {
    Request(SetChargingProfileRequest),
    Response(SetChargingProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetDisplayMessageKind {
    Request(SetDisplayMessageRequest),
    Response(SetDisplayMessageResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetMonitoringBaseKind {
    Request(SetMonitoringBaseRequest),
    Response(SetMonitoringBaseResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetMonitoringLevelKind {
    Request(SetMonitoringLevelRequest),
    Response(SetMonitoringLevelResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetNetworkProfileKind {
    Request(SetNetworkProfileRequest),
    Response(SetNetworkProfileResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetVariableMonitoringKind {
    Request(SetVariableMonitoringRequest),
    Response(SetVariableMonitoringResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SetVariablesKind {
    Request(SetVariablesRequest),
    Response(SetVariablesResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum SignCertificateKind {
    Request(SignCertificateRequest),
    Response(SignCertificateResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum StatusNotificationKind {
    Request(StatusNotificationRequest),
    Response(StatusNotificationResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum TransactionEventKind {
    Request(TransactionEventRequest),
    Response(TransactionEventResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum TriggerMessageKind {
    Request(TriggerMessageRequest),
    Response(TriggerMessageResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum UnlockConnectorKind {
    Request(UnlockConnectorRequest),
    Response(UnlockConnectorResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum UnpublishFirmwareKind {
    Request(UnpublishFirmwareRequest),
    Response(UnpublishFirmwareResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Display)]
#[serde(untagged)]
pub enum UpdateFirmwareKind {
    Request(UpdateFirmwareRequest),
    Response(UpdateFirmwareResponse),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppActionEnum {
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
    DataTransfer,
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

impl fmt::Display for OcppActionEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for OcppActionEnum {
    type Err = ();

    fn from_str(input: &str) -> Result<OcppActionEnum, Self::Err> {
        match input {
            "Authorize" => Ok(OcppActionEnum::Authorize),
            "BootNotification" => Ok(OcppActionEnum::BootNotification),
            "CancelReservation" => Ok(OcppActionEnum::CancelReservation),
            "CertificateSigned" => Ok(OcppActionEnum::CertificateSigned),
            "ChangeAvailability" => Ok(OcppActionEnum::ChangeAvailability),
            "ClearCache" => Ok(OcppActionEnum::ClearCache),
            "ClearChargingProfile" => Ok(OcppActionEnum::ClearChargingProfile),
            "ClearDisplayMessage" => Ok(OcppActionEnum::ClearDisplayMessage),
            "ClearedChargingLimit" => Ok(OcppActionEnum::ClearedChargingLimit),
            "ClearVariableMonitoring" => Ok(OcppActionEnum::ClearVariableMonitoring),
            "CostUpdated" => Ok(OcppActionEnum::CostUpdated),
            "CustomerInformation" => Ok(OcppActionEnum::CustomerInformation),
            "DataTransfer" => Ok(OcppActionEnum::DataTransfer),
            "DeleteCertificate" => Ok(OcppActionEnum::DeleteCertificate),
            "FirmwareStatusNotification" => Ok(OcppActionEnum::FirmwareStatusNotification),
            "Get15118EVCertificate" => Ok(OcppActionEnum::Get15118EVCertificate),
            "GetBaseReport" => Ok(OcppActionEnum::GetBaseReport),
            "GetCertificateStatus" => Ok(OcppActionEnum::GetCertificateStatus),
            "GetChargingProfile" => Ok(OcppActionEnum::GetChargingProfile),
            "GetCompositeSchedule" => Ok(OcppActionEnum::GetCompositeSchedule),
            "GetDisplayMessage" => Ok(OcppActionEnum::GetDisplayMessage),
            "GetInstalledCertificateIds" => Ok(OcppActionEnum::GetInstalledCertificateIds),
            "GetLocalListVersion" => Ok(OcppActionEnum::GetLocalListVersion),
            "GetLog" => Ok(OcppActionEnum::GetLog),
            "GetMonitoringReport" => Ok(OcppActionEnum::GetMonitoringReport),
            "GetReport" => Ok(OcppActionEnum::GetReport),
            "GetTransactionStatus" => Ok(OcppActionEnum::GetTransactionStatus),
            "GetVariables" => Ok(OcppActionEnum::GetVariables),
            "Heartbeat" => Ok(OcppActionEnum::Heartbeat),
            "InstallCertificate" => Ok(OcppActionEnum::InstallCertificate),
            "LogStatusNotification" => Ok(OcppActionEnum::LogStatusNotification),
            "MeterValues" => Ok(OcppActionEnum::MeterValues),
            "NotifyChargingLimit" => Ok(OcppActionEnum::NotifyChargingLimit),
            "NotifyCustomerInformation" => Ok(OcppActionEnum::NotifyCustomerInformation),
            "NotifyDisplayMessages" => Ok(OcppActionEnum::NotifyDisplayMessages),
            "NotifyEVChargingNeeds" => Ok(OcppActionEnum::NotifyEVChargingNeeds),
            "NotifyEVChargingSchedule" => Ok(OcppActionEnum::NotifyEVChargingSchedule),
            "NotifyEvent" => Ok(OcppActionEnum::NotifyEvent),
            "NotifyMonitoringReport" => Ok(OcppActionEnum::NotifyMonitoringReport),
            "NotifyReport" => Ok(OcppActionEnum::NotifyReport),
            "PublishFirmware" => Ok(OcppActionEnum::PublishFirmware),
            "PublishFirmwareStatusNotification" => {
                Ok(OcppActionEnum::PublishFirmwareStatusNotification)
            }
            "ReportChargingProfiles" => Ok(OcppActionEnum::ReportChargingProfiles),
            "RequestStartTransaction" => Ok(OcppActionEnum::RequestStartTransaction),
            "RequestStopTransaction" => Ok(OcppActionEnum::RequestStopTransaction),
            "ReservationStatusUpdate" => Ok(OcppActionEnum::ReservationStatusUpdate),
            "ReserveNow" => Ok(OcppActionEnum::ReserveNow),
            "Reset" => Ok(OcppActionEnum::Reset),
            "SecurityEventNotification" => Ok(OcppActionEnum::SecurityEventNotification),
            "SendLocalList" => Ok(OcppActionEnum::SendLocalList),
            "SetChargingProfile" => Ok(OcppActionEnum::SetChargingProfile),
            "SetDisplayMessage" => Ok(OcppActionEnum::SetDisplayMessage),
            "SetMonitoringBase" => Ok(OcppActionEnum::SetMonitoringBase),
            "SetMonitoringLevel" => Ok(OcppActionEnum::SetMonitoringLevel),
            "SetNetworkProfile" => Ok(OcppActionEnum::SetNetworkProfile),
            "SetVariableMonitoring" => Ok(OcppActionEnum::SetVariableMonitoring),
            "SetVariables" => Ok(OcppActionEnum::SetVariables),
            "SignCertificate" => Ok(OcppActionEnum::SignCertificate),
            "StatusNotification" => Ok(OcppActionEnum::StatusNotification),
            "TransactionEvent" => Ok(OcppActionEnum::TransactionEvent),
            "TriggerMessage" => Ok(OcppActionEnum::TriggerMessage),
            "UnlockConnector" => Ok(OcppActionEnum::UnlockConnector),
            "UnpublishFirmware" => Ok(OcppActionEnum::UnpublishFirmware),
            "UpdateFirmware" => Ok(OcppActionEnum::UpdateFirmware),
            _ => Err(()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum OcppPayload {
    Authorize(AuthorizeKind),
    BootNotification(BootNotificationKind),
    CancelReservation(CancelReservationKind),
    CertificateSigned(CertificateSignedKind),
    ChangeAvailability(ChangeAvailabilityKind),
    ClearCache(ClearCacheKind),
    ClearChargingProfile(ClearChargingProfileKind),
    ClearDisplayMessage(ClearDisplayMessageKind),
    ClearedChargingLimit(ClearedChargingLimitKind),
    ClearVariableMonitoring(ClearVariableMonitoringKind),
    CostUpdated(CostUpdatedKind),
    CustomerInforfmation(CustomerInformationKind),
    Datatransfer(DataTransferKind),
    DeleteCertificate(DeleteCertificateKind),
    FirmwareStatusNotification(FirmwareStatusNotificationKind),
    Get15118EVCertificate(Get15118EVCertificateKind),
    GetBaseReport(GetBaseReportKind),
    GetCertificateStatus(GetCertificateStatusKind),
    GetChargingProfile(GetChargingProfilesKind),
    GetCompositeSchedule(GetCompositeScheduleKind),
    GetDisplayMessage(GetDisplayMessagesKind),
    GetInstalledCertificateIds(GetInstalledCertificateIdsKind),
    GetLocalListVersion(GetLocalListVersionKind),
    GetLog(GetLogKind),
    GetMonitoringReport(GetMonitoringReportKind),
    GetReport(GetReportKind),
    GetTransactionStatus(GetTransactionStatusKind),
    GetVariables(GetVariablesKind),
    Heartbeat(HeartbeatKind),
    InstallCertificate(InstallCertificateKind),
    LogStatusNotification(LogStatusNotificationKind),
    MeterValues(MeterValuesKind),
    NotifyChargingLimit(NotifyChargingLimitKind),
    NotifyCustomerInformation(NotifyCustomerInformationKind),
    NotifyDisplayMessages(NotifyDisplayMessagesKind),
    NotifyEVChargingNeeds(NotifyEVChargingNeedsKind),
    NotifyEVChargingSchedule(NotifyEVChargingScheduleKind),
    NotifyEvent(NotifyEventKind),
    NotifyMonitoringReport(NotifyMonitoringReportKind),
    NotifyReport(NotifyReportKind),
    PublishFirmware(PublishFirmwareKind),
    PublishFirmwareStatusNotification(PublishFirmwareStatusNotificationKind),
    ReportChargingProfiles(ReportChargingProfilesKind),
    RequestStartTransaction(RequestStartTransactionKind),
    RequestStopTransaction(RequestStopTransactionKind),
    ReservationStatusUpdate(ReservationStatusUpdateKind),
    ReserveNow(ReserveNowKind),
    Reset(ResetKind),
    SecurityEventNotification(SecurityEventNotificationKind),
    SendLocalList(SendLocalListKind),
    SetChargingProfile(SetChargingProfileKind),
    SetDisplayMessage(SetDisplayMessageKind),
    SetMonitoringBase(SetMonitoringBaseKind),
    SetMonitoringLevel(SetMonitoringLevelKind),
    SetNetworkProfile(SetNetworkProfileKind),
    SetVariableMonitoring(SetVariableMonitoringKind),
    SetVariables(SetVariablesKind),
    SignCertificate(SignCertificateKind),
    StatusNotification(StatusNotificationKind),
    TransactionEvent(TransactionEventKind),
    TriggerMessage(TriggerMessageKind),
    UnlockConnector(UnlockConnectorKind),
    UnpublishFirmware(UnpublishFirmwareKind),
    UpdateFirmware(UpdateFirmwareKind),
}
