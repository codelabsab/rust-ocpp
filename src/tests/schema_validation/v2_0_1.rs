#[cfg(test)]
mod tests {
    use crate::v2_0_1::datatypes::certificate_hash_data_type::CertificateHashDataType;
    use crate::v2_0_1::datatypes::charging_limit_type::ChargingLimitType;
    use crate::v2_0_1::datatypes::charging_needs_type::ChargingNeedsType;
    use crate::v2_0_1::datatypes::charging_profile_criterion_type::ChargingProfileCriterionType;
    use crate::v2_0_1::datatypes::charging_schedule_period_type::ChargingSchedulePeriodType;
    use crate::v2_0_1::datatypes::charging_schedule_type::ChargingScheduleType;
    use crate::v2_0_1::datatypes::charging_station_type::ChargingStationType;
    use crate::v2_0_1::datatypes::clear_monitoring_result_type::ClearMonitoringResultType;
    use crate::v2_0_1::datatypes::component_type::ComponentType;
    use crate::v2_0_1::datatypes::event_data_type::EventDataType;
    use crate::v2_0_1::datatypes::get_variable_data_type::GetVariableDataType;
    use crate::v2_0_1::datatypes::get_variable_result_type::GetVariableResultType;
    use crate::v2_0_1::datatypes::id_token_info_type::IdTokenInfoType;
    use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
    use crate::v2_0_1::datatypes::log_parameters_type::LogParametersType;
    use crate::v2_0_1::datatypes::meter_value_type::MeterValueType;
    use crate::v2_0_1::datatypes::ocsp_request_data_type::OCSPRequestDataType;
    use crate::v2_0_1::datatypes::sampled_value_type::SampledValueType;
    use crate::v2_0_1::datatypes::variable_type::VariableType;
    use crate::v2_0_1::enumerations::authorization_status_enum_type::AuthorizationStatusEnumType;
    use crate::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;
    use crate::v2_0_1::enumerations::cancel_reservation_status_enum_type::CancelReservationStatusEnumType;
    use crate::v2_0_1::enumerations::certificate_action_enum_type::CertificateActionEnumType;
    use crate::v2_0_1::enumerations::certificate_signed_status_enum_type::CertificateSignedStatusEnumType;
    use crate::v2_0_1::enumerations::change_availability_status_enum_type::ChangeAvailabilityStatusEnumType;
    use crate::v2_0_1::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;
    use crate::v2_0_1::enumerations::charging_rate_unit_enum_type::ChargingRateUnitEnumType;
    use crate::v2_0_1::enumerations::clear_cache_status_enum_type::ClearCacheStatusEnumType;
    use crate::v2_0_1::enumerations::clear_charging_profile_status_enum_type::ClearChargingProfileStatusEnumType;
    use crate::v2_0_1::enumerations::clear_message_status_enum_type::ClearMessageStatusEnumType;
    use crate::v2_0_1::enumerations::clear_monitoring_status_enum_type::ClearMonitoringStatusEnumType;
    use crate::v2_0_1::enumerations::customer_information_status_enum_type::CustomerInformationStatusEnumType;
    use crate::v2_0_1::enumerations::data_transfer_status_enum_type::DataTransferStatusEnumType;
    use crate::v2_0_1::enumerations::delete_certificate_status_enum_type::DeleteCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::energy_transfer_mode_enum_type::EnergyTransferModeEnumType;
    use crate::v2_0_1::enumerations::event_notification_enum_type::EventNotificationEnumType;
    use crate::v2_0_1::enumerations::event_trigger_enum_type::EventTriggerEnumType;
    use crate::v2_0_1::enumerations::firmware_status_enum_type::FirmwareStatusEnumType;
    use crate::v2_0_1::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType;
    use crate::v2_0_1::enumerations::generic_status_enum_type::GenericStatusEnumType;
    use crate::v2_0_1::enumerations::get_certificate_status_enum_type::GetCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::get_charging_profile_status_enum_type::GetChargingProfileStatusEnumType;
    use crate::v2_0_1::enumerations::get_display_messages_status_enum_type::GetDisplayMessagesStatusEnumType;
    use crate::v2_0_1::enumerations::get_variable_status_enum_type::GetVariableStatusEnumType;
    use crate::v2_0_1::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;
    use crate::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;
    use crate::v2_0_1::enumerations::install_certificate_status_enum_type::InstallCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::install_certificate_use_enum_type::InstallCertificateUseEnumType;
    use crate::v2_0_1::enumerations::iso15118ev_certificate_status_enum_type::Iso15118EVCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::log_enum_type::LogEnumType;
    use crate::v2_0_1::enumerations::log_status_enum_type::LogStatusEnumType;
    use crate::v2_0_1::enumerations::notify_ev_charging_needs_status_enum_type::NotifyEVChargingNeedsStatusEnumType;
    use crate::v2_0_1::enumerations::operational_status_enum_type::OperationalStatusEnumType;
    use crate::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType;
    use crate::v2_0_1::enumerations::report_base_enum_type::ReportBaseEnumType;
    use crate::v2_0_1::enumerations::upload_log_status_enum_type::UploadLogStatusEnumType;
    use crate::v2_0_1::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
    use crate::v2_0_1::messages::boot_notification::{
        BootNotificationRequest, BootNotificationResponse,
    };
    use crate::v2_0_1::messages::cancel_reservation::{
        CancelReservationRequest, CancelReservationResponse,
    };
    use crate::v2_0_1::messages::certificate_signed::{
        CertificateSignedRequest, CertificateSignedResponse,
    };
    use crate::v2_0_1::messages::change_availability::{
        ChangeAvailabilityRequest, ChangeAvailabilityResponse,
    };
    use crate::v2_0_1::messages::clear_cache::{ClearCacheRequest, ClearCacheResponse};
    use crate::v2_0_1::messages::clear_charging_profile::{
        ClearChargingProfileRequest, ClearChargingProfileResponse,
    };
    use crate::v2_0_1::messages::clear_display_message::{
        ClearDisplayMessageRequest, ClearDisplayMessageResponse,
    };
    use crate::v2_0_1::messages::clear_variable_monitoring::{
        ClearVariableMonitoringRequest, ClearVariableMonitoringResponse,
    };
    use crate::v2_0_1::messages::cleared_charging_limit::{
        ClearedChargingLimitRequest, ClearedChargingLimitResponse,
    };
    use crate::v2_0_1::messages::cost_updated::{CostUpdatedRequest, CostUpdatedResponse};
    use crate::v2_0_1::messages::customer_information::{
        CustomerInformationRequest, CustomerInformationResponse,
    };
    use crate::v2_0_1::messages::datatransfer::{DataTransferRequest, DataTransferResponse};
    use crate::v2_0_1::messages::delete_certificate::{
        DeleteCertificateRequest, DeleteCertificateResponse,
    };
    use crate::v2_0_1::messages::firmware_status_notification::{
        FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
    };
    use crate::v2_0_1::messages::get_15118ev_certificate::{
        Get15118EVCertificateRequest, Get15118EVCertificateResponse,
    };
    use crate::v2_0_1::messages::get_base_report::{GetBaseReportRequest, GetBaseReportResponse};
    use crate::v2_0_1::messages::get_certificate_status::{
        GetCertificateStatusRequest, GetCertificateStatusResponse,
    };
    use crate::v2_0_1::messages::get_charging_profiles::{
        GetChargingProfilesRequest, GetChargingProfilesResponse,
    };
    use crate::v2_0_1::messages::get_composite_schedule::{
        GetCompositeScheduleRequest, GetCompositeScheduleResponse,
    };
    use crate::v2_0_1::messages::get_display_message::{
        GetDisplayMessagesRequest, GetDisplayMessagesResponse,
    };
    use crate::v2_0_1::messages::get_installed_certificate_ids::{
        GetInstalledCertificateIdsRequest, GetInstalledCertificateIdsResponse,
    };
    use crate::v2_0_1::messages::get_local_list_version::{
        GetLocalListVersionRequest, GetLocalListVersionResponse,
    };
    use crate::v2_0_1::messages::get_log::{GetLogRequest, GetLogResponse};
    use crate::v2_0_1::messages::get_monitoring_report::{
        GetMonitoringReportRequest, GetMonitoringReportResponse,
    };
    use crate::v2_0_1::messages::get_report::{GetReportRequest, GetReportResponse};
    use crate::v2_0_1::messages::get_transaction_status::{
        GetTransactionStatusRequest, GetTransactionStatusResponse,
    };
    use crate::v2_0_1::messages::get_variables::{GetVariablesRequest, GetVariablesResponse};
    use crate::v2_0_1::messages::heartbeat::{HeartbeatRequest, HeartbeatResponse};
    use crate::v2_0_1::messages::install_certificate::{
        InstallCertificateRequest, InstallCertificateResponse,
    };
    use crate::v2_0_1::messages::log_status_notification::{
        LogStatusNotificationRequest, LogStatusNotificationResponse,
    };
    use crate::v2_0_1::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
    use crate::v2_0_1::messages::notify_charging_limit::{
        NotifyChargingLimitRequest, NotifyChargingLimitResponse,
    };
    use crate::v2_0_1::messages::notify_customer_information::{
        NotifyCustomerInformationRequest, NotifyCustomerInformationResponse,
    };
    use crate::v2_0_1::messages::notify_display_messages::{
        NotifyDisplayMessagesRequest, NotifyDisplayMessagesResponse,
    };
    use crate::v2_0_1::messages::notify_ev_charging_needs::{
        NotifyEVChargingNeedsRequest, NotifyEVChargingNeedsResponse,
    };
    use crate::v2_0_1::messages::notify_ev_charging_schedule::{
        NotifyEVChargingScheduleRequest, NotifyEVChargingScheduleResponse,
    };
    use crate::v2_0_1::messages::notify_event::{NotifyEventRequest, NotifyEventResponse};
    use crate::v2_0_1::messages::notify_monitoring_report::{
        NotifyMonitoringReportRequest, NotifyMonitoringReportResponse,
    };
    use crate::v2_0_1::messages::notify_report::{NotifyReportRequest, NotifyReportResponse};
    use chrono::Utc;
    use jsonschema::JSONSchema;
    use crate::v2_0_1::datatypes::charging_profile_type::ChargingProfileType;
    use crate::v2_0_1::datatypes::firmware_type::FirmwareType;
    use crate::v2_0_1::datatypes::message_content_type::MessageContentType;
    use crate::v2_0_1::datatypes::message_info_type::MessageInfoType;
    use crate::v2_0_1::datatypes::network_connection_profile_type::NetworkConnectionProfileType;
    use crate::v2_0_1::datatypes::set_monitoring_data_type::SetMonitoringDataType;
    use crate::v2_0_1::datatypes::set_monitoring_result_type::SetMonitoringResultType;
    use crate::v2_0_1::datatypes::set_variable_data_type::SetVariableDataType;
    use crate::v2_0_1::datatypes::set_variable_result_type::SetVariableResultType;
    use crate::v2_0_1::datatypes::transaction_type::TransactionType;
    use crate::v2_0_1::enumerations::charging_profile_kind_enum_type::ChargingProfileKindEnumType;
    use crate::v2_0_1::enumerations::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;
    use crate::v2_0_1::enumerations::charging_profile_status_enum_type::ChargingProfileStatusEnumType;
    use crate::v2_0_1::enumerations::connector_status_enum_type::ConnectorStatusEnumType;
    use crate::v2_0_1::enumerations::display_message_status_enum_type::DisplayMessageStatusEnumType;
    use crate::v2_0_1::enumerations::message_format_enum_type::MessageFormatEnumType;
    use crate::v2_0_1::enumerations::message_priority_enum_type::MessagePriorityEnumType;
    use crate::v2_0_1::enumerations::message_state_enum_type::MessageStateEnumType;
    use crate::v2_0_1::enumerations::message_trigger_enum_type::MessageTriggerEnumType;
    use crate::v2_0_1::enumerations::monitor_enum_type::MonitorEnumType;
    use crate::v2_0_1::enumerations::monitoring_base_enum_type::MonitoringBaseEnumType;
    use crate::v2_0_1::enumerations::ocpp_interface_enum_type::OCPPInterfaceEnumType;
    use crate::v2_0_1::enumerations::ocpp_transport_enum_type::OCPPTransportEnumType;
    use crate::v2_0_1::enumerations::ocpp_version_enum_type::OCPPVersionEnumType;
    use crate::v2_0_1::enumerations::publish_firmware_status_enum_type::PublishFirmwareStatusEnumType;
    use crate::v2_0_1::enumerations::request_start_stop_status_enum_type::RequestStartStopStatusEnumType;
    use crate::v2_0_1::enumerations::reservation_update_status_enum_type::ReservationUpdateStatusEnumType;
    use crate::v2_0_1::enumerations::reserve_now_status_enum_type::ReserveNowStatusEnumType;
    use crate::v2_0_1::enumerations::reset_enum_type::ResetEnumType;
    use crate::v2_0_1::enumerations::reset_status_enum_type::ResetStatusEnumType;
    use crate::v2_0_1::enumerations::send_local_list_status_enum_type::SendLocalListStatusEnumType;
    use crate::v2_0_1::enumerations::set_monitoring_status_enum_type::SetMonitoringStatusEnumType;
    use crate::v2_0_1::enumerations::set_network_profile_status_enum_type::SetNetworkProfileStatusEnumType;
    use crate::v2_0_1::enumerations::set_variable_status_enum_type::SetVariableStatusEnumType;
    use crate::v2_0_1::enumerations::transaction_event_enum_type::TransactionEventEnumType;
    use crate::v2_0_1::enumerations::trigger_message_status_enum_type::TriggerMessageStatusEnumType;
    use crate::v2_0_1::enumerations::trigger_reason_enum_type::TriggerReasonEnumType;
    use crate::v2_0_1::enumerations::unlock_status_enum_type::UnlockStatusEnumType;
    use crate::v2_0_1::enumerations::unpublish_firmware_status_enum_type::UnpublishFirmwareStatusEnumType;
    use crate::v2_0_1::enumerations::update_enum_type::UpdateEnumType;
    use crate::v2_0_1::enumerations::update_firmware_status_enum_type::UpdateFirmwareStatusEnumType;
    use crate::v2_0_1::messages::publish_firmware::{PublishFirmwareRequest, PublishFirmwareResponse};
    use crate::v2_0_1::messages::publish_firmware_status_notification::{PublishFirmwareStatusNotificationRequest, PublishFirmwareStatusNotificationResponse};
    use crate::v2_0_1::messages::report_charging_profiles::{ReportChargingProfilesRequest, ReportChargingProfilesResponse};
    use crate::v2_0_1::messages::request_start_transaction::{RequestStartTransactionRequest, RequestStartTransactionResponse};
    use crate::v2_0_1::messages::request_stop_transaction::{RequestStopTransactionRequest, RequestStopTransactionResponse};
    use crate::v2_0_1::messages::reservation_status_update::{ReservationStatusUpdateRequest, ReservationStatusUpdateResponse};
    use crate::v2_0_1::messages::reserve_now::{ReserveNowRequest, ReserveNowResponse};
    use crate::v2_0_1::messages::reset::{ResetRequest, ResetResponse};
    use crate::v2_0_1::messages::security_event_notification::{SecurityEventNotificationRequest, SecurityEventNotificationResponse};
    use crate::v2_0_1::messages::send_local_list::{SendLocalListRequest, SendLocalListResponse};
    use crate::v2_0_1::messages::set_charging_profile::{SetChargingProfileRequest, SetChargingProfileResponse};
    use crate::v2_0_1::messages::set_display_message::{SetDisplayMessageRequest, SetDisplayMessageResponse};
    use crate::v2_0_1::messages::set_monitoring_base::{SetMonitoringBaseRequest, SetMonitoringBaseResponse};
    use crate::v2_0_1::messages::set_monitoring_level::{SetMonitoringLevelRequest, SetMonitoringLevelResponse};
    use crate::v2_0_1::messages::set_network_profile::{SetNetworkProfileRequest, SetNetworkProfileResponse};
    use crate::v2_0_1::messages::set_variable_monitoring::{SetVariableMonitoringRequest, SetVariableMonitoringResponse};
    use crate::v2_0_1::messages::set_variables::{SetVariablesRequest, SetVariablesResponse};
    use crate::v2_0_1::messages::sign_certificate::{SignCertificateRequest, SignCertificateResponse};
    use crate::v2_0_1::messages::status_notification::{StatusNotificationRequest, StatusNotificationResponse};
    use crate::v2_0_1::messages::transaction_event::{TransactionEventRequest, TransactionEventResponse};
    use crate::v2_0_1::messages::trigger_message::{TriggerMessageRequest, TriggerMessageResponse};
    use crate::v2_0_1::messages::unlock_connector::{UnlockConnectorRequest, UnlockConnectorResponse};
    use crate::v2_0_1::messages::unpublish_firmware::{UnpublishFirmwareRequest, UnpublishFirmwareResponse};
    use crate::v2_0_1::messages::update_firmware::{UpdateFirmwareRequest, UpdateFirmwareResponse};

    #[test]
    fn validate_authorize_request() {
        let test = AuthorizeRequest {
            certificate: None,
            id_token: IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: None,
            },
            iso_15118_certificate_hash_data: None,
        };

        let schema = include_str!("schemas/v2.0.1/AuthorizeRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_authorize_response() {
        let test = AuthorizeResponse {
            certificate_status: None,
            id_token_info: IdTokenInfoType {
                status: AuthorizationStatusEnumType::Accepted,
                cache_expiry_date_time: None,
                charging_priority: None,
                language1: None,
                evse_id: None,
                language2: None,
                group_id_token: None,
                personal_message: None,
            },
        };

        let schema = include_str!("schemas/v2.0.1/AuthorizeResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_boot_notification_request() {
        let test = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                model: "SingleSocketCharger".to_string(),
                vendor_name: "VendorX".to_string(),
                serial_number: None,
                firmware_version: None,
                modem: None,
            },
        };
        let schema = include_str!("schemas/v2.0.1/BootNotificationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_boot_notification_response() {
        let test = BootNotificationResponse {
            current_time: Utc::now(),
            interval: 10,
            status: RegistrationStatusEnumType::Accepted,
            status_info: None,
        };

        let schema = include_str!("schemas/v2.0.1/BootNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cancel_reservation_request() {
        let test = CancelReservationRequest { reservation_id: 0 };
        let schema = include_str!("schemas/v2.0.1/CancelReservationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cancel_reservation_response() {
        let test = CancelReservationResponse {
            status: CancelReservationStatusEnumType::Accepted,
            status_info: None,
        };

        let schema = include_str!("schemas/v2.0.1/CancelReservationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_certificate_signed_request() {
        let test = CertificateSignedRequest {
            certificate_chain: "asd".to_string(),
            certificate_type: None,
        };
        let schema = include_str!("schemas/v2.0.1/CertificateSignedRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_certificate_signed_response() {
        let test = CertificateSignedResponse {
            status: CertificateSignedStatusEnumType::Accepted,
            status_info: None,
        };

        let schema = include_str!("schemas/v2.0.1/CertificateSignedResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_change_availability_request() {
        let test = ChangeAvailabilityRequest {
            operational_status: OperationalStatusEnumType::Inoperative,
            evse: None,
        };
        let schema = include_str!("schemas/v2.0.1/ChangeAvailabilityRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_change_availability_response() {
        let test = ChangeAvailabilityResponse {
            status: ChangeAvailabilityStatusEnumType::Accepted,
            status_info: None,
        };

        let schema =
            include_str!("schemas/v2.0.1/ChangeAvailabilityResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_cache_request() {
        let test = ClearCacheRequest {};
        let schema = include_str!("schemas/v2.0.1/ClearCacheRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_cache_response() {
        let test = ClearCacheResponse {
            status: ClearCacheStatusEnumType::Accepted,
            status_info: None,
        };

        let schema = include_str!("schemas/v2.0.1/ClearCacheResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_charging_profile_request() {
        let test = ClearChargingProfileRequest {
            charging_profile_id: None,
            charging_profile_criteria: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/ClearChargingProfileRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_charging_profile_response() {
        let test = ClearChargingProfileResponse {
            status: ClearChargingProfileStatusEnumType::Accepted,
            status_info: None,
        };

        let schema =
            include_str!("schemas/v2.0.1/ClearChargingProfileResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_display_message_request() {
        let test = ClearDisplayMessageRequest { id: 0 };
        let schema =
            include_str!("schemas/v2.0.1/ClearDisplayMessageRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_display_message_response() {
        let test = ClearDisplayMessageResponse {
            status: ClearMessageStatusEnumType::Accepted,
            status_info: None,
        };

        let schema =
            include_str!("schemas/v2.0.1/ClearDisplayMessageResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cleared_charging_limit_request() {
        let test = ClearedChargingLimitRequest {
            charging_limit_source: ChargingLimitSourceEnumType::EMS,
            evse: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/ClearedChargingLimitRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cleared_charging_limit_response() {
        let test = ClearedChargingLimitResponse {};

        let schema =
            include_str!("schemas/v2.0.1/ClearedChargingLimitResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_variable_monitoring_request() {
        let test = ClearVariableMonitoringRequest { id: vec![0] };
        let schema =
            include_str!("schemas/v2.0.1/ClearVariableMonitoringRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_variable_monitoring_response() {
        let test = ClearVariableMonitoringResponse {
            clear_monitoring_result: vec![ClearMonitoringResultType {
                status: ClearMonitoringStatusEnumType::Accepted,
                id: 0,
                status_info: None,
            }],
        };
        let schema =
            include_str!("schemas/v2.0.1/ClearVariableMonitoringResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cost_updated_request() {
        let test = CostUpdatedRequest {
            total_cost: 0.0,
            transaction_id: "".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/CostUpdatedRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_cost_updated_response() {
        let test = CostUpdatedResponse {};
        let schema = include_str!("schemas/v2.0.1/CostUpdatedResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_customer_information_request() {
        let test = CustomerInformationRequest {
            request_id: 0,
            report: false,
            clear: false,
            customer_identifier: None,
            id_token: None,
            customer_certificate: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/CustomerInformationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_customer_information_response() {
        let test = CustomerInformationResponse {
            status: CustomerInformationStatusEnumType::Accepted,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/CustomerInformationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_data_transfer_request() {
        let test = DataTransferRequest {
            message_id: None,
            data: "".to_string(),
            vendor_id: "".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/DataTransferRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_data_transfer_response() {
        let test = DataTransferResponse {
            status: DataTransferStatusEnumType::Accepted,
            data: "".to_string(),
            status_info: None,
        };
        let schema = include_str!("schemas/v2.0.1/DataTransferResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_delete_certificate_request() {
        let test = DeleteCertificateRequest {
            certificate_hash_data: CertificateHashDataType {
                hash_algorithm: HashAlgorithmEnumType::SHA256,
                issuer_name_hash: "".to_string(),
                issuer_key_hash: "".to_string(),
                serial_number: "".to_string(),
            },
        };
        let schema = include_str!("schemas/v2.0.1/DeleteCertificateRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_delete_certificate_response() {
        let test = DeleteCertificateResponse {
            status: DeleteCertificateStatusEnumType::Accepted,
            status_info: None,
        };
        let schema = include_str!("schemas/v2.0.1/DeleteCertificateResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_firmware_status_notification_request() {
        let test = FirmwareStatusNotificationRequest {
            status: FirmwareStatusEnumType::Downloaded,
            request_id: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/FirmwareStatusNotificationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_firmware_status_notification_response() {
        let test = FirmwareStatusNotificationResponse {};
        let schema =
            include_str!("schemas/v2.0.1/FirmwareStatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get15118ev_certificate_request() {
        let test = Get15118EVCertificateRequest {
            iso_15118_schema_version: "".to_string(),
            action: CertificateActionEnumType::Install,
            exi_request: "".to_string(),
        };
        let schema =
            include_str!("schemas/v2.0.1/Get15118EVCertificateRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get15118ev_certificate_response() {
        let test = Get15118EVCertificateResponse {
            status: Iso15118EVCertificateStatusEnumType::Accepted,
            exi_response: "".to_string(),
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/Get15118EVCertificateResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_base_report_request() {
        let test = GetBaseReportRequest {
            request_id: 0,
            report_base: ReportBaseEnumType::ConfigurationInventory,
        };
        let schema = include_str!("schemas/v2.0.1/GetBaseReportRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_base_report_response() {
        let test = GetBaseReportResponse {
            status: GenericDeviceModelStatusEnumType::Accepted,
            status_info: None,
        };
        let schema = include_str!("schemas/v2.0.1/GetBaseReportResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_certificate_status_request() {
        let test = GetCertificateStatusRequest {
            ocsp_request_data: OCSPRequestDataType {
                hash_algorithm: HashAlgorithmEnumType::SHA256,
                issuer_name_hash: "".to_string(),
                issuer_key_hash: "".to_string(),
                serial_number: "".to_string(),
                responder_url: "".to_string(),
            },
        };
        let schema =
            include_str!("schemas/v2.0.1/GetCertificateStatusRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_certificate_status_response() {
        let test = GetCertificateStatusResponse {
            status: GetCertificateStatusEnumType::Accepted,
            ocsp_result: None,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetCertificateStatusResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_charging_profiles_request() {
        let test = GetChargingProfilesRequest {
            request_id: 0,
            evse_id: None,
            charging_profile: ChargingProfileCriterionType {
                charging_profile_purpose: None,
                stack_level: None,
                charging_profile_id: None,
                charging_limit_source: None,
            },
        };
        let schema =
            include_str!("schemas/v2.0.1/GetChargingProfilesRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_charging_profiles_response() {
        let test = GetChargingProfilesResponse {
            status: GetChargingProfileStatusEnumType::Accepted,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetChargingProfilesResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_composite_schedule_request() {
        let test = GetCompositeScheduleRequest {
            duration: 0,
            charging_rate_unit: None,
            evse_id: 0,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetCompositeScheduleRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_composite_schedule_response() {
        let test = GetCompositeScheduleResponse {
            status: GenericStatusEnumType::Accepted,
            schedule: None,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetCompositeScheduleResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_display_messages_request() {
        let test = GetDisplayMessagesRequest {
            id: None,
            request_id: 0,
            priority: None,
            state: None,
        };
        let schema = include_str!("schemas/v2.0.1/GetDisplayMessagesRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_display_messages_response() {
        let test = GetDisplayMessagesResponse {
            status: GetDisplayMessagesStatusEnumType::Accepted,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetDisplayMessagesResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_installed_certificate_ids_request() {
        let test = GetInstalledCertificateIdsRequest {
            certificate_type: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetInstalledCertificateIdsRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_installed_certificate_ids_response() {
        let test = GetInstalledCertificateIdsResponse {
            status: GetDisplayMessagesStatusEnumType::Accepted,
            certificate_hash_data_chain: None,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetInstalledCertificateIdsResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_local_list_version_request() {
        let test = GetLocalListVersionRequest {};
        let schema =
            include_str!("schemas/v2.0.1/GetLocalListVersionRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_local_list_version_response() {
        let test = GetLocalListVersionResponse { version_number: 0 };
        let schema =
            include_str!("schemas/v2.0.1/GetLocalListVersionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_log_request() {
        let test = GetLogRequest {
            log_type: LogEnumType::DiagnosticsLog,
            request_id: 0,
            retries: None,
            retry_interval: None,
            log: LogParametersType {
                remote_location: "".to_string(),
                oldest_timestamp: None,
                latest_timestamp: None,
            },
        };
        let schema = include_str!("schemas/v2.0.1/GetLogRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_log_response() {
        let test = GetLogResponse {
            status: LogStatusEnumType::Accepted,
            filename: None,
            status_info: None,
        };
        let schema = include_str!("schemas/v2.0.1/GetLogResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_monitoring_report_request() {
        let test = GetMonitoringReportRequest {
            request_id: 0,
            monitoring_criteria: None,
            component_variable: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetMonitoringReportRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_monitoring_report_response() {
        let test = GetMonitoringReportResponse {
            status: GenericDeviceModelStatusEnumType::Accepted,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetMonitoringReportResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_report_request() {
        let test = GetReportRequest {
            request_id: 0,
            component_criteria: None,
            component_variable: None,
        };
        let schema = include_str!("schemas/v2.0.1/GetReportRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_report_response() {
        let test = GetReportResponse {
            status: GenericDeviceModelStatusEnumType::Accepted,
            status_info: None,
        };
        let schema = include_str!("schemas/v2.0.1/GetReportResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_transaction_status_request() {
        let test = GetTransactionStatusRequest {
            transaction_id: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetTransactionStatusRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_transaction_status_response() {
        let test = GetTransactionStatusResponse {
            ongoing_indicator: None,
            messages_in_queue: false,
        };
        let schema =
            include_str!("schemas/v2.0.1/GetTransactionStatusResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_variables_request() {
        let test = GetVariablesRequest {
            get_variable_data: vec![GetVariableDataType {
                attribute_type: None,
                component: ComponentType {
                    name: "".to_string(),
                    instance: None,
                    evse: None,
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: None,
                },
            }],
        };
        let schema = include_str!("schemas/v2.0.1/GetVariablesRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_variables_response() {
        let test = GetVariablesResponse {
            get_variable_result: vec![GetVariableResultType {
                attribute_status: GetVariableStatusEnumType::Accepted,
                attribute_type: None,
                attribute_value: None,
                component: ComponentType {
                    name: "".to_string(),
                    instance: None,
                    evse: None,
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: None,
                },
                attribute_status_info: None,
            }],
        };
        let schema = include_str!("schemas/v2.0.1/GetVariablesResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_heartbeat_request() {
        let test = HeartbeatRequest {};
        let schema = include_str!("schemas/v2.0.1/HeartbeatRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_heartbeat_response() {
        let test = HeartbeatResponse {
            current_time: Utc::now(),
        };
        let schema = include_str!("schemas/v2.0.1/HeartbeatResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_install_certificate_request() {
        let test = InstallCertificateRequest {
            certificate_type: InstallCertificateUseEnumType::V2GRootCertificate,
            certificate: "".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/InstallCertificateRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_install_certificate_response() {
        let test = InstallCertificateResponse {
            status: InstallCertificateStatusEnumType::Accepted,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/InstallCertificateResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_log_status_notification_request() {
        let test = LogStatusNotificationRequest {
            status: UploadLogStatusEnumType::BadMessage,
            request_id: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/LogStatusNotificationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_log_status_notification_response() {
        let test = LogStatusNotificationResponse {};
        let schema =
            include_str!("schemas/v2.0.1/LogStatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_meter_values_request() {
        let test = MeterValuesRequest {
            evse_id: 0,
            meter_value: vec![MeterValueType {
                timestamp: Utc::now(),
                sampled_value: vec![SampledValueType {
                    value: 0.0,
                    context: None,
                    measurand: None,
                    phase: None,
                    location: None,
                    signed_meter_value: None,
                    unit_of_measure: None,
                }],
            }],
        };
        let schema = include_str!("schemas/v2.0.1/MeterValuesRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_meter_values_response() {
        let test = MeterValuesResponse {};
        let schema = include_str!("schemas/v2.0.1/MeterValuesResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_charging_limit_request() {
        let test = NotifyChargingLimitRequest {
            evse_id: None,
            charging_limit: ChargingLimitType {
                charging_limit_source: ChargingLimitSourceEnumType::EMS,
                is_grid_critical: None,
            },
            charging_schedule: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyChargingLimitRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_charging_limit_response() {
        let test = NotifyChargingLimitResponse {};
        let schema =
            include_str!("schemas/v2.0.1/NotifyChargingLimitResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_customer_information_request() {
        let test = NotifyCustomerInformationRequest {
            data: "".to_string(),
            tbc: None,
            seq_no: 0,
            generated_at: Utc::now(),
            request_id: 0,
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyCustomerInformationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_customer_information_response() {
        let test = NotifyCustomerInformationResponse {};
        let schema =
            include_str!("schemas/v2.0.1/NotifyCustomerInformationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_display_messages_request() {
        let test = NotifyDisplayMessagesRequest {
            request_id: 0,
            tbc: None,
            message_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyDisplayMessagesRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_display_messages_response() {
        let test = NotifyDisplayMessagesResponse {};
        let schema =
            include_str!("schemas/v2.0.1/NotifyDisplayMessagesResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_ev_charging_needs_request() {
        let test = NotifyEVChargingNeedsRequest {
            max_schedule_tuples: None,
            evse_id: 0,
            charging_needs: ChargingNeedsType {
                requested_energy_transfer: EnergyTransferModeEnumType::DC,
                departure_time: None,
                ac_charging_parameters: None,
                dc_charging_parameters: None,
            },
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyEVChargingNeedsRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_ev_charging_needs_response() {
        let test = NotifyEVChargingNeedsResponse {
            status: NotifyEVChargingNeedsStatusEnumType::Accepted,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyEVChargingNeedsResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_ev_charging_schedule_request() {
        let test = NotifyEVChargingScheduleRequest {
            time_base: Utc::now(),
            evse_id: 0,
            charging_schedule: ChargingScheduleType {
                id: 0,
                start_schedule: None,
                duration: None,
                charging_rate_unit: ChargingRateUnitEnumType::W,
                min_charging_rate: None,
                charging_schedule_period: vec![ChargingSchedulePeriodType {
                    start_period: 0,
                    limit: 0.0,
                    number_phases: None,
                    phase_to_use: None,
                }],
                sales_tariff: None,
            },
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyEVChargingScheduleRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_ev_charging_schedule_response() {
        let test = NotifyEVChargingScheduleResponse {
            status: GenericStatusEnumType::Accepted,
            status_info: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyEVChargingScheduleResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_event_request() {
        let test = NotifyEventRequest {
            generated_at: Utc::now(),
            tbc: None,
            seq_no: 0,
            event_data: vec![EventDataType {
                event_id: 0,
                timestamp: Utc::now(),
                trigger: EventTriggerEnumType::Alerting,
                cause: None,
                actual_value: "".to_string(),
                tech_code: None,
                tech_info: None,
                cleared: None,
                transaction_id: None,
                variable_monitoring_id: None,
                event_notification_type: EventNotificationEnumType::HardWiredNotification,
                component: ComponentType {
                    name: "".to_string(),
                    instance: None,
                    evse: None,
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: None,
                },
            }],
        };
        let schema = include_str!("schemas/v2.0.1/NotifyEventRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_event_response() {
        let test = NotifyEventResponse {};
        let schema = include_str!("schemas/v2.0.1/NotifyEventResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_monitoring_report_request() {
        let test = NotifyMonitoringReportRequest {
            request_id: 0,
            tbc: None,
            seq_no: 0,
            generated_at: Utc::now(),
            monitor: None,
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyMonitoringReportRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_monitoring_report_response() {
        let test = NotifyMonitoringReportResponse {};
        let schema =
            include_str!("schemas/v2.0.1/NotifyMonitoringReportResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_report_request() {
        let test = NotifyReportRequest {
            request_id: 0,
            tbc: None,
            seq_no: 0,
            generated_at: Utc::now(),
            report_data: None
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyReportRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_report_response() {
        let test = NotifyReportResponse {
        };
        let schema =
            include_str!("schemas/v2.0.1/NotifyReportResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_publish_firmware_request() {
        let test = PublishFirmwareRequest {
            location: "".to_string(),
            retries: None,
            checksum: "".to_string(),
            request_id: 0,
            retry_interval: None
        };
        let schema =
            include_str!("schemas/v2.0.1/PublishFirmwareRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_publish_firmware_response() {
        let test = PublishFirmwareResponse {
            status: GenericStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/PublishFirmwareResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_publish_firmware_status_notification_request() {
        let test = PublishFirmwareStatusNotificationRequest {
            status: PublishFirmwareStatusEnumType::Idle,
            location: None,
            request_id: None
        };
        let schema =
            include_str!("schemas/v2.0.1/PublishFirmwareStatusNotificationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_publish_firmware_status_notification_response() {
        let test = PublishFirmwareStatusNotificationResponse {
        };
        let schema =
            include_str!("schemas/v2.0.1/PublishFirmwareStatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_report_charging_profiles_request() {
        let test = ReportChargingProfilesRequest {
            request_id: 0,
            charging_limit_source: ChargingLimitSourceEnumType::EMS,
            tbc: None,
            evse_id: 0,
            charging_profile: vec![ChargingProfileType {
                id: 0,
                stack_level: 0,
                charging_profile_purpose: ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                charging_profile_kind: ChargingProfileKindEnumType::Absolute,
                recurrency_kind: None,
                valid_from: None,
                valid_to: None,
                transaction_id: None,
                charging_schedule: vec![ChargingScheduleType {
                    id: 0,
                    start_schedule: None,
                    duration: None,
                    charging_rate_unit: ChargingRateUnitEnumType::W,
                    min_charging_rate: None,
                    charging_schedule_period: vec![ChargingSchedulePeriodType {
                        start_period: 0,
                        limit: 0.0,
                        number_phases: None,
                        phase_to_use: None
                    }],
                    sales_tariff: None
                }]
            }]
        };
        let schema =
            include_str!("schemas/v2.0.1/ReportChargingProfilesRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_report_charging_profiles_response() {
        let test = ReportChargingProfilesResponse {
        };
        let schema =
            include_str!("schemas/v2.0.1/ReportChargingProfilesResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_request_start_transaction_request() {
        let test = RequestStartTransactionRequest {
            evse_id: None,
            remote_start_id: 0,
            id_token: IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: None
            },
            charging_profile: None,
            group_id_token: None
        };
        let schema =
            include_str!("schemas/v2.0.1/RequestStartTransactionRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_request_start_transaction_response() {
        let test = RequestStartTransactionResponse {
            status: RequestStartStopStatusEnumType::Accepted,
            transaction_id: None,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/RequestStartTransactionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_request_stop_transaction_request() {
        let test = RequestStopTransactionRequest {
            transaction_id: "".to_string()
        };
        let schema =
            include_str!("schemas/v2.0.1/RequestStopTransactionRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_request_stop_transaction_response() {
        let test = RequestStopTransactionResponse {
            status: RequestStartStopStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/RequestStopTransactionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reservation_status_update_request() {
        let test = ReservationStatusUpdateRequest {
            reservation_id: 0,
            reservation_update_status: ReservationUpdateStatusEnumType::Expired
        };
        let schema =
            include_str!("schemas/v2.0.1/ReservationStatusUpdateRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reservation_status_update_response() {
        let test = ReservationStatusUpdateResponse {
        };
        let schema =
            include_str!("schemas/v2.0.1/ReservationStatusUpdateResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reserve_now_request() {
        let test = ReserveNowRequest {
            id: 0,
            expiry_date_time: Utc::now(),
            connector_type: None,
            evse_id: None,
            id_token: IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: None
            },
            group_id_token: None
        };
        let schema =
            include_str!("schemas/v2.0.1/ReserveNowRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reserve_now_response() {
        let test = ReserveNowResponse {
            status: ReserveNowStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/ReserveNowResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reset_request() {
        let test = ResetRequest {
            request_type: ResetEnumType::Immediate,
            evse_id: None
        };
        let schema =
            include_str!("schemas/v2.0.1/ResetRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reset_response() {
        let test = ResetResponse {
            status: ResetStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/ResetResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_security_event_notification_request() {
        let test = SecurityEventNotificationRequest {
            kind: "".to_string(),
            timestamp: Utc::now(),
            tech_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SecurityEventNotificationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_security_event_notification_response() {
        let test = SecurityEventNotificationResponse {
        };
        let schema =
            include_str!("schemas/v2.0.1/SecurityEventNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_send_local_list_request() {
        let test = SendLocalListRequest {
            version_number: 0,
            update_type: UpdateEnumType::Differential,
            tech_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SendLocalListRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_send_local_list_response() {
        let test = SendLocalListResponse {
            status: SendLocalListStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SendLocalListResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_charging_profile_request() {
        let test = SetChargingProfileRequest {
            evse_id: 0,
            charging_profile: ChargingProfileType {
                id: 0,
                stack_level: 0,
                charging_profile_purpose: ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                charging_profile_kind: ChargingProfileKindEnumType::Absolute,
                recurrency_kind: None,
                valid_from: None,
                valid_to: None,
                transaction_id: None,
                charging_schedule: vec![ChargingScheduleType {
                    id: 0,
                    start_schedule: None,
                    duration: None,
                    charging_rate_unit: ChargingRateUnitEnumType::W,
                    min_charging_rate: None,
                    charging_schedule_period: vec![ChargingSchedulePeriodType{
                        start_period: 0,
                        limit: 0.0,
                        number_phases: None,
                        phase_to_use: None
                    }],
                    sales_tariff: None
                }]
            }
        };
        let schema =
            include_str!("schemas/v2.0.1/SetChargingProfileRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_charging_profile_response() {
        let test = SetChargingProfileResponse {
            status: ChargingProfileStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SetChargingProfileResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_display_message_request() {
        let test = SetDisplayMessageRequest {
            message: MessageInfoType {
                id: 0,
                priority: MessagePriorityEnumType::AlwaysFront,
                state: MessageStateEnumType::Charging,
                start_date_time: None,
                end_date_time: None,
                transaction_id: None,
                message: MessageContentType {
                    format: MessageFormatEnumType::ASCII,
                    language: None,
                    content: "".to_string()
                },
                display: None
            }
        };
        let schema =
            include_str!("schemas/v2.0.1/SetDisplayMessageRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_display_message_response() {
        let test = SetDisplayMessageResponse {
            status: DisplayMessageStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SetDisplayMessageResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_monitoring_base_request() {
        let test = SetMonitoringBaseRequest {
            monitoring_base: MonitoringBaseEnumType::All
        };
        let schema =
            include_str!("schemas/v2.0.1/SetMonitoringBaseRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_monitoring_base_response() {
        let test = SetMonitoringBaseResponse {
            status: GenericDeviceModelStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SetMonitoringBaseResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_monitoring_level_request() {
        let test = SetMonitoringLevelRequest {
            severity: 0
        };
        let schema =
            include_str!("schemas/v2.0.1/SetMonitoringLevelRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_monitoring_level_response() {
        let test = SetMonitoringLevelResponse {
            status: GenericStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SetMonitoringLevelResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_network_profile_request() {
        let test = SetNetworkProfileRequest {
            configuration_slot: 0,
            connection_data: NetworkConnectionProfileType {
                ocpp_version: OCPPVersionEnumType::OCPP12,
                ocpp_transport: OCPPTransportEnumType::JSON,
                ocpp_csms_url: "".to_string(),
                message_timeout: 0,
                security_profile: 0,
                ocpp_interface: OCPPInterfaceEnumType::Wired0,
                vpn: None,
                apn: None
            }
        };
        let schema =
            include_str!("schemas/v2.0.1/SetNetworkProfileRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_network_profile_response() {
        let test = SetNetworkProfileResponse {
            status: SetNetworkProfileStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SetNetworkProfileResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_variable_monitoring_request() {
        let test = SetVariableMonitoringRequest {
            set_monitoring_data: vec![SetMonitoringDataType {
                id: None,
                transaction: None,
                value: 0.0,
                kind: MonitorEnumType::UpperThreshold,
                severity: 0,
                component: ComponentType {
                    name: "".to_string(),
                    instance: None,
                    evse: None
                },
                variable: VariableType { name: "".to_string(), instance: None }
            }]
        };
        let schema =
            include_str!("schemas/v2.0.1/SetVariableMonitoringRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_variable_monitoring_response() {
        let test = SetVariableMonitoringResponse {
            set_monitoring_result: vec![SetMonitoringResultType {
                id: None,
                status: SetMonitoringStatusEnumType::Accepted,
                kind: MonitorEnumType::UpperThreshold,
                severity: 0,
                component: ComponentType {
                    name: "".to_string(),
                    instance: None,
                    evse: None
                },
                variable: VariableType { name: "".to_string(), instance: None },
                status_info: None
            }]
        };
        let schema =
            include_str!("schemas/v2.0.1/SetVariableMonitoringResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_variables_request() {
        let test = SetVariablesRequest {
            set_variable_data: vec![SetVariableDataType {
                attribute_type: None,
                attribute_value: "".to_string(),
                component: ComponentType {
                    name: "".to_string(),
                    instance: None,
                    evse: None
                },
                variable: VariableType { name: "".to_string(), instance: None }
            }]
        };
        let schema =
            include_str!("schemas/v2.0.1/SetVariablesRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_variables_response() {
        let test = SetVariablesResponse {
            set_variable_result: vec![SetVariableResultType {
                attribute_type: None,
                attribute_status: SetVariableStatusEnumType::Accepted,
                component: ComponentType {
                    name: "".to_string(),
                    instance: None,
                    evse: None
                },
                variable: VariableType { name: "".to_string(), instance: None },
                attribute_status_info: None
            }]
        };
        let schema =
            include_str!("schemas/v2.0.1/SetVariablesResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_sign_certificate_request() {
        let test = SignCertificateRequest {
            csr: "".to_string(),
            certificate_type: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SignCertificateRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_sign_certificate_response() {
        let test = SignCertificateResponse {
            status: GenericStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/SignCertificateResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_status_notification_request() {
        let test = StatusNotificationRequest {
            timestamp: Utc::now(),
            connector_status: ConnectorStatusEnumType::Available,
            evse_id: 0,
            connector_id: 0
        };
        let schema =
            include_str!("schemas/v2.0.1/StatusNotificationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_status_notification_response() {
        let test = StatusNotificationResponse {
        };
        let schema =
            include_str!("schemas/v2.0.1/StatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_transaction_event_request() {
        let test = TransactionEventRequest {
            event_type: TransactionEventEnumType::Ended,
            timestamp: Utc::now(),
            trigger_reason: TriggerReasonEnumType::Authorized,
            seq_no: 0,
            offline: None,
            number_of_phases_used: None,
            cable_max_current: None,
            reservation_id: None,
            transaction_info: TransactionType {
                transaction_id: "".to_string(),
                charging_state: None,
                time_spent_charging: None,
                stopped_reason: None,
                remote_start_id: None
            },
            id_token: None,
            evse: None,
            meter_value: None
        };
        let schema =
            include_str!("schemas/v2.0.1/TransactionEventRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_transaction_event_response() {
        let test = TransactionEventResponse {
            total_cost: None,
            charging_priority: None,
            id_token_info: None,
            updated_personal_message: None
        };
        let schema =
            include_str!("schemas/v2.0.1/TransactionEventResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_trigger_message_request() {
        let test = TriggerMessageRequest {
            requested_message: MessageTriggerEnumType::BootNotification,
            evse: None
        };
        let schema =
            include_str!("schemas/v2.0.1/TriggerMessageRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_trigger_message_response() {
        let test = TriggerMessageResponse {
            status: TriggerMessageStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/TriggerMessageResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_unlock_connector_request() {
        let test = UnlockConnectorRequest {
            evse_id: 0,
            connector_id: 0
        };
        let schema =
            include_str!("schemas/v2.0.1/UnlockConnectorRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_unlock_connector_response() {
        let test = UnlockConnectorResponse {
            status: UnlockStatusEnumType::Unlocked,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/UnlockConnectorResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_unpublish_firmware_request() {
        let test = UnpublishFirmwareRequest {
            checksum: "".to_string()
        };
        let schema =
            include_str!("schemas/v2.0.1/UnpublishFirmwareRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_unpublish_firmware_response() {
        let test = UnpublishFirmwareResponse {
            status: UnpublishFirmwareStatusEnumType::DownloadOngoing
        };
        let schema =
            include_str!("schemas/v2.0.1/UnpublishFirmwareResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_update_firmware_request() {
        let test = UpdateFirmwareRequest {
            retries: None,
            retry_interval: None,
            request_id: 0,
            firmware: FirmwareType {
                location: "".to_string(),
                retrieve_date_time: Utc::now(),
                install_date_time: None,
                signing_certificate: None,
                signature: None
            }
        };
        let schema =
            include_str!("schemas/v2.0.1/UpdateFirmwareRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_update_firmware_response() {
        let test = UpdateFirmwareResponse {
            status: UpdateFirmwareStatusEnumType::Accepted,
            status_info: None
        };
        let schema =
            include_str!("schemas/v2.0.1/UpdateFirmwareResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
}
