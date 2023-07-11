#[cfg(test)]
mod tests {
    use crate::datatypes::ac_charging_parameters_type::ACChargingParametersType;
    use crate::datatypes::additional_info_type::AdditionalInfoType;
    use crate::datatypes::apn_type::APNType;
    use crate::datatypes::authorization_data::AuthorizationData;
    use crate::datatypes::certificate_hash_data_chain_type::CertificateHashDataChainType;
    use crate::datatypes::certificate_hash_data_type::CertificateHashDataType;
    use crate::datatypes::charging_limit_type::ChargingLimitType;
    use crate::datatypes::charging_needs_type::ChargingNeedsType;
    use crate::datatypes::charging_profile_criterion_type::ChargingProfileCriterionType;
    use crate::datatypes::charging_profile_type::ChargingProfileType;
    use crate::datatypes::charging_schedule_period_type::ChargingSchedulePeriodType;
    use crate::datatypes::charging_schedule_type::ChargingScheduleType;
    use crate::datatypes::charging_station_type::ChargingStationType;
    use crate::datatypes::clear_charging_profile_type::ClearChargingProfileType;
    use crate::datatypes::clear_monitoring_result_type::ClearMonitoringResultType;
    use crate::datatypes::component_type::ComponentType;
    use crate::datatypes::component_variable_type::ComponentVariableType;
    use crate::datatypes::composite_schedule_type::CompositeScheduleType;
    use crate::datatypes::consumption_cost_type::ConsumptionCostType;
    use crate::datatypes::cost_type::CostType;
    use crate::datatypes::dc_charging_parameters_type::DCChargingParametersType;
    use crate::datatypes::event_data_type::EventDataType;
    use crate::datatypes::evse_type::EVSEType;
    use crate::datatypes::firmware_type::FirmwareType;
    use crate::datatypes::get_variable_data_type::GetVariableDataType;
    use crate::datatypes::get_variable_result_type::GetVariableResultType;
    use crate::datatypes::id_token_info_type::IdTokenInfoType;
    use crate::datatypes::id_token_type::IdTokenType;
    use crate::datatypes::log_parameters_type::LogParametersType;
    use crate::datatypes::message_content_type::MessageContentType;
    use crate::datatypes::message_info_type::MessageInfoType;
    use crate::datatypes::meter_value_type::MeterValueType;
    use crate::datatypes::modem_type::ModemType;
    use crate::datatypes::monitoring_data_type::MonitoringDataType;
    use crate::datatypes::network_connection_profile_type::NetworkConnectionProfileType;
    use crate::datatypes::ocsp_request_data_type::OCSPRequestDataType;
    use crate::datatypes::relative_time_interval_type::RelativeTimeIntervalType;
    use crate::datatypes::report_data_type::ReportDataType;
    use crate::datatypes::sales_tariff_entry_type::SalesTariffEntryType;
    use crate::datatypes::sales_tariff_type::SalesTariffType;
    use crate::datatypes::sampled_value_type::SampledValueType;
    use crate::datatypes::set_monitoring_data_type::SetMonitoringDataType;
    use crate::datatypes::set_monitoring_result_type::SetMonitoringResultType;
    use crate::datatypes::set_variable_data_type::SetVariableDataType;
    use crate::datatypes::set_variable_result_type::SetVariableResultType;
    use crate::datatypes::signed_meter_value_type::SignedMeterValueType;
    use crate::datatypes::status_info_type::StatusInfoType;
    use crate::datatypes::transaction_type::TransactionType;
    use crate::datatypes::unit_of_measure_type::UnitOfMeasureType;
    use crate::datatypes::variable_attribute_type::VariableAttributeType;
    use crate::datatypes::variable_characteristics_type::VariableCharacteristicsType;
    use crate::datatypes::variable_monitoring_type::VariableMonitoringType;
    use crate::datatypes::variable_type::VariableType;
    use crate::datatypes::vpn_type::VPNType;
    use crate::enumerations::apn_authentication_enum_type::APNAuthenticationEnumType;
    use crate::enumerations::attribute_enum_type::AttributeEnumType;
    use crate::enumerations::authorization_status_enum_type::AuthorizationStatusEnumType;
    use crate::enumerations::authorize_certificate_status_enum_type::AuthorizeCertificateStatusEnumType;
    use crate::enumerations::boot_reason_enum_type::BootReasonEnumType;
    use crate::enumerations::cancel_reservation_status_enum_type::CancelReservationStatusEnumType;
    use crate::enumerations::certificate_action_enum_type::CertificateActionEnumType;
    use crate::enumerations::certificate_signed_status_enum_type::CertificateSignedStatusEnumType;
    use crate::enumerations::certificate_signing_use_enum_type::CertificateSigningUseEnumType;
    use crate::enumerations::change_availability_status_enum_type::ChangeAvailabilityStatusEnumType;
    use crate::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;
    use crate::enumerations::charging_profile_kind_enum_type::ChargingProfileKindEnumType;
    use crate::enumerations::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;
    use crate::enumerations::charging_profile_status_enum_type::ChargingProfileStatusEnumType;
    use crate::enumerations::charging_rate_unit_enum_type::ChargingRateUnitEnumType;
    use crate::enumerations::charging_state_enum_type::ChargingStateEnumType;
    use crate::enumerations::clear_cache_status_enum_type::ClearCacheStatusEnumType;
    use crate::enumerations::clear_charging_profile_status_enum_type::ClearChargingProfileStatusEnumType;
    use crate::enumerations::clear_message_status_enum_type::ClearMessageStatusEnumType;
    use crate::enumerations::clear_monitoring_status_enum_type::ClearMonitoringStatusEnumType;
    use crate::enumerations::component_criterion_enum_type::ComponentCriterionEnumType;
    use crate::enumerations::connector_enum_type::ConnectorEnumType;
    use crate::enumerations::connector_status_enum_type::ConnectorStatusEnumType;
    use crate::enumerations::cost_kind_enum_type::CostKindEnumType;
    use crate::enumerations::customer_information_status_enum_type::CustomerInformationStatusEnumType;
    use crate::enumerations::data_enum_type::DataEnumType;
    use crate::enumerations::data_transfer_status_enum_type::DataTransferStatusEnumType;
    use crate::enumerations::delete_certificate_status_enum_type::DeleteCertificateStatusEnumType;
    use crate::enumerations::display_message_status_enum_type::DisplayMessageStatusEnumType;
    use crate::enumerations::energy_transfer_mode_enum_type::EnergyTransferModeEnumType;
    use crate::enumerations::event_notification_enum_type::EventNotificationEnumType;
    use crate::enumerations::event_trigger_enum_type::EventTriggerEnumType;
    use crate::enumerations::firmware_status_enum_type::FirmwareStatusEnumType;
    use crate::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType;
    use crate::enumerations::generic_status_enum_type::GenericStatusEnumType;
    use crate::enumerations::get_certificate_id_use_enum_type::GetCertificateIdUseEnumType;
    use crate::enumerations::get_certificate_status_enum_type::GetCertificateStatusEnumType;
    use crate::enumerations::get_charging_profile_status_enum_type::GetChargingProfileStatusEnumType;
    use crate::enumerations::get_display_messages_status_enum_type::GetDisplayMessagesStatusEnumType;
    use crate::enumerations::get_variable_status_enum_type::GetVariableStatusEnumType;
    use crate::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;
    use crate::enumerations::id_token_enum_type::IdTokenEnumType;
    use crate::enumerations::install_certificate_status_enum_type::InstallCertificateStatusEnumType;
    use crate::enumerations::install_certificate_use_enum_type::InstallCertificateUseEnumType;
    use crate::enumerations::iso15118ev_certificate_status_enum_type::Iso15118EVCertificateStatusEnumType;
    use crate::enumerations::location_enum_type::LocationEnumType;
    use crate::enumerations::log_enum_type::LogEnumType;
    use crate::enumerations::log_status_enum_type::LogStatusEnumType;
    use crate::enumerations::measurand_enum_type::MeasurandEnumType;
    use crate::enumerations::message_format_enum_type::MessageFormatEnumType;
    use crate::enumerations::message_priority_enum_type::MessagePriorityEnumType;
    use crate::enumerations::message_state_enum_type::MessageStateEnumType;
    use crate::enumerations::message_trigger_enum_type::MessageTriggerEnumType;
    use crate::enumerations::monitor_enum_type::MonitorEnumType;
    use crate::enumerations::monitoring_base_enum_type::MonitoringBaseEnumType;
    use crate::enumerations::monitoring_criterion_enum_type::MonitoringCriterionEnumType;
    use crate::enumerations::mutability_enum_type::MutabilityEnumType;
    use crate::enumerations::notify_ev_charging_needs_status_enum_type::NotifyEVChargingNeedsStatusEnumType;
    use crate::enumerations::ocpp_interface_enum_type::OCPPInterfaceEnumType;
    use crate::enumerations::ocpp_transport_enum_type::OCPPTransportEnumType;
    use crate::enumerations::ocpp_version_enum_type::OCPPVersionEnumType;
    use crate::enumerations::operational_status_enum_type::OperationalStatusEnumType;
    use crate::enumerations::phase_enum_type::PhaseEnumType;
    use crate::enumerations::publish_firmware_status_enum_type::PublishFirmwareStatusEnumType;
    use crate::enumerations::reading_context_enum_type::ReadingContextEnumType;
    use crate::enumerations::reason_enum_type::ReasonEnumType;
    use crate::enumerations::recurrency_kind_enum_type::RecurrencyKindEnumType;
    use crate::enumerations::registration_status_enum_type::RegistrationStatusEnumType;
    use crate::enumerations::report_base_enum_type::ReportBaseEnumType;
    use crate::enumerations::request_start_stop_status_enum_type::RequestStartStopStatusEnumType;
    use crate::enumerations::reservation_update_status_enum_type::ReservationUpdateStatusEnumType;
    use crate::enumerations::reserve_now_status_enum_type::ReserveNowStatusEnumType;
    use crate::enumerations::reset_enum_type::ResetEnumType;
    use crate::enumerations::reset_status_enum_type::ResetStatusEnumType;
    use crate::enumerations::send_local_list_status_enum_type::SendLocalListStatusEnumType;
    use crate::enumerations::set_monitoring_status_enum_type::SetMonitoringStatusEnumType;
    use crate::enumerations::set_network_profile_status_enum_type::SetNetworkProfileStatusEnumType;
    use crate::enumerations::set_variable_status_enum_type::SetVariableStatusEnumType;
    use crate::enumerations::transaction_event_enum_type::TransactionEventEnumType;
    use crate::enumerations::trigger_message_status_enum_type::TriggerMessageStatusEnumType;
    use crate::enumerations::trigger_reason_enum_type::TriggerReasonEnumType;
    use crate::enumerations::unlock_status_enum_type::UnlockStatusEnumType;
    use crate::enumerations::unpublish_firmware_status_enum_type::UnpublishFirmwareStatusEnumType;
    use crate::enumerations::update_enum_type::UpdateEnumType;
    use crate::enumerations::update_firmware_status_enum_type::UpdateFirmwareStatusEnumType;
    use crate::enumerations::upload_log_status_enum_type::UploadLogStatusEnumType;
    use crate::enumerations::vpn_enum_type::VPNEnumType;
    use crate::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
    use crate::messages::boot_notification::{BootNotificationRequest, BootNotificationResponse};
    use crate::messages::cancel_reservation::{
        CancelReservationRequest, CancelReservationResponse,
    };
    use crate::messages::certificate_signed::{
        CertificateSignedRequest, CertificateSignedResponse,
    };
    use crate::messages::change_availability::{
        ChangeAvailabilityRequest, ChangeAvailabilityResponse,
    };
    use crate::messages::clear_cache::{ClearCacheRequest, ClearCacheResponse};
    use crate::messages::clear_charging_profile::{
        ClearChargingProfileRequest, ClearChargingProfileResponse,
    };
    use crate::messages::clear_display_message::{
        ClearDisplayMessageRequest, ClearDisplayMessageResponse,
    };
    use crate::messages::clear_variable_monitoring::{
        ClearVariableMonitoringRequest, ClearVariableMonitoringResponse,
    };
    use crate::messages::cleared_charging_limit::{
        ClearedChargingLimitRequest, ClearedChargingLimitResponse,
    };
    use crate::messages::cost_updated::{CostUpdatedRequest, CostUpdatedResponse};
    use crate::messages::customer_information::{
        CustomerInformationRequest, CustomerInformationResponse,
    };
    use crate::messages::datatransfer::{DataTransferRequest, DataTransferResponse};
    use crate::messages::delete_certificate::{
        DeleteCertificateRequest, DeleteCertificateResponse,
    };
    use crate::messages::firmware_status_notification::{
        FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
    };
    use crate::messages::get_15118ev_certificate::{
        Get15118EVCertificateRequest, Get15118EVCertificateResponse,
    };
    use crate::messages::get_base_report::{GetBaseReportRequest, GetBaseReportResponse};
    use crate::messages::get_certificate_status::{
        GetCertificateStatusRequest, GetCertificateStatusResponse,
    };
    use crate::messages::get_charging_profiles::{
        GetChargingProfilesRequest, GetChargingProfilesResponse,
    };
    use crate::messages::get_composite_schedule::{
        GetCompositeScheduleRequest, GetCompositeScheduleResponse,
    };
    use crate::messages::get_display_message::{
        GetDisplayMessagesRequest, GetDisplayMessagesResponse,
    };
    use crate::messages::get_installed_certificate_ids::{
        GetInstalledCertificateIdsRequest, GetInstalledCertificateIdsResponse,
    };
    use crate::messages::get_local_list_version::{
        GetLocalListVersionRequest, GetLocalListVersionResponse,
    };
    use crate::messages::get_log::{GetLogRequest, GetLogResponse};
    use crate::messages::get_monitoring_report::{
        GetMonitoringReportRequest, GetMonitoringReportResponse,
    };
    use crate::messages::get_report::{GetReportRequest, GetReportResponse};
    use crate::messages::get_transaction_status::{
        GetTransactionStatusRequest, GetTransactionStatusResponse,
    };
    use crate::messages::get_variables::{GetVariablesRequest, GetVariablesResponse};
    use crate::messages::heartbeat::{HeartbeatRequest, HeartbeatResponse};
    use crate::messages::install_certificate::{
        InstallCertificateRequest, InstallCertificateResponse,
    };
    use crate::messages::log_status_notification::{
        LogStatusNotificationRequest, LogStatusNotificationResponse,
    };
    use crate::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
    use crate::messages::notify_charging_limit::{
        NotifyChargingLimitRequest, NotifyChargingLimitResponse,
    };
    use crate::messages::notify_customer_information::{
        NotifyCustomerInformationRequest, NotifyCustomerInformationResponse,
    };
    use crate::messages::notify_display_messages::{
        NotifyDisplayMessagesRequest, NotifyDisplayMessagesResponse,
    };
    use crate::messages::notify_ev_charging_needs::{
        NotifyEVChargingNeedsRequest, NotifyEVChargingNeedsResponse,
    };
    use crate::messages::notify_ev_charging_schedule::{
        NotifyEVChargingScheduleRequest, NotifyEVChargingScheduleResponse,
    };
    use crate::messages::notify_event::{NotifyEventRequest, NotifyEventResponse};
    use crate::messages::notify_monitoring_report::{
        NotifyMonitoringReportRequest, NotifyMonitoringReportResponse,
    };
    use crate::messages::notify_report::{NotifyReportRequest, NotifyReportResponse};
    use crate::messages::publish_firmware::{PublishFirmwareRequest, PublishFirmwareResponse};
    use crate::messages::publish_firmware_status_notification::{
        PublishFirmwareStatusNotificationRequest, PublishFirmwareStatusNotificationResponse,
    };
    use crate::messages::report_charging_profiles::{
        ReportChargingProfilesRequest, ReportChargingProfilesResponse,
    };
    use crate::messages::request_start_transaction::{
        RequestStartTransactionRequest, RequestStartTransactionResponse,
    };
    use crate::messages::request_stop_transaction::{
        RequestStopTransactionRequest, RequestStopTransactionResponse,
    };
    use crate::messages::reservation_status_update::{
        ReservationStatusUpdateRequest, ReservationStatusUpdateResponse,
    };
    use crate::messages::reserve_now::{ReserveNowRequest, ReserveNowResponse};
    use crate::messages::reset::{ResetRequest, ResetResponse};
    use crate::messages::security_event_notification::{
        SecurityEventNotificationRequest, SecurityEventNotificationResponse,
    };
    use crate::messages::send_local_list::{SendLocalListRequest, SendLocalListResponse};
    use crate::messages::set_charging_profile::{
        SetChargingProfileRequest, SetChargingProfileResponse,
    };
    use crate::messages::set_display_message::{
        SetDisplayMessageRequest, SetDisplayMessageResponse,
    };
    use crate::messages::set_monitoring_base::{
        SetMonitoringBaseRequest, SetMonitoringBaseResponse,
    };
    use crate::messages::set_monitoring_level::{
        SetMonitoringLevelRequest, SetMonitoringLevelResponse,
    };
    use crate::messages::set_network_profile::{
        SetNetworkProfileRequest, SetNetworkProfileResponse,
    };
    use crate::messages::set_variable_monitoring::{
        SetVariableMonitoringRequest, SetVariableMonitoringResponse,
    };
    use crate::messages::set_variables::{SetVariablesRequest, SetVariablesResponse};
    use crate::messages::sign_certificate::{SignCertificateRequest, SignCertificateResponse};
    use crate::messages::status_notification::{
        StatusNotificationRequest, StatusNotificationResponse,
    };
    use crate::messages::transaction_event::{TransactionEventRequest, TransactionEventResponse};
    use crate::messages::trigger_message::{TriggerMessageRequest, TriggerMessageResponse};
    use crate::messages::unlock_connector::{UnlockConnectorRequest, UnlockConnectorResponse};
    use crate::messages::unpublish_firmware::{
        UnpublishFirmwareRequest, UnpublishFirmwareResponse,
    };
    use crate::messages::update_firmware::{UpdateFirmwareRequest, UpdateFirmwareResponse};
    use chrono::Utc;
    use jsonschema::JSONSchema;

    #[test]
    fn validate_authorize_request() {
        let test = AuthorizeRequest {
            certificate: Some("".to_string()),
            id_token: IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    additional_id_token: "more than 5 characters".to_string(),
                    kind: "".to_string(),
                }]),
            },
            iso_15118_certificate_hash_data: Some(vec![OCSPRequestDataType {
                hash_algorithm: HashAlgorithmEnumType::SHA256,
                issuer_name_hash: "".to_string(),
                issuer_key_hash: "".to_string(),
                serial_number: "".to_string(),
                responder_url: "".to_string(),
            }]),
        };

        let schema = include_str!("json_schemas/AuthorizeRequest.json");
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
            certificate_status: Some(AuthorizeCertificateStatusEnumType::Accepted),
            id_token_info: IdTokenInfoType {
                status: AuthorizationStatusEnumType::Accepted,
                cache_expiry_date_time: Some(Utc::now()),
                charging_priority: Some(1),
                language1: Some("English".to_string()),
                evse_id: Some(vec![1]),
                language2: Some("Chinese".to_string()),
                group_id_token: Some(IdTokenType {
                    id_token: "".to_string(),
                    kind: IdTokenEnumType::Central,
                    additional_info: Some(vec![AdditionalInfoType {
                        additional_id_token: "".to_string(),
                        kind: "".to_string(),
                    }]),
                }),
                personal_message: Some(MessageContentType {
                    format: MessageFormatEnumType::ASCII,
                    language: Some("Swedish".to_string()),
                    content: "".to_string(),
                }),
            },
        };

        let schema = include_str!("json_schemas/AuthorizeResponse.json");
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
                serial_number: Some("serial_number".to_string()),
                firmware_version: Some("firmware_version".to_string()),
                modem: Some(ModemType {
                    iccid: Some("iccid".to_string()),
                    imsi: Some("imsi".to_string()),
                }),
            },
        };
        let schema = include_str!("json_schemas/BootNotificationRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("json_schemas/BootNotificationResponse.json");
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
        let schema = include_str!("json_schemas/CancelReservationRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("json_schemas/CancelReservationResponse.json");
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
            certificate_chain: "certificate_chain".to_string(),
            certificate_type: Some(CertificateSigningUseEnumType::ChargingStationCertificate),
        };
        let schema = include_str!("json_schemas/CertificateSignedRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("json_schemas/CertificateSignedResponse.json");
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
            evse: Some(EVSEType {
                id: 0,
                connector_id: Some(1),
            }),
        };
        let schema = include_str!("json_schemas/ChangeAvailabilityRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("json_schemas/ChangeAvailabilityResponse.json");
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
        let schema = include_str!("json_schemas/ClearCacheRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("json_schemas/ClearCacheResponse.json");
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
            charging_profile_id: Some(1),
            charging_profile_criteria: Some(ClearChargingProfileType {
                evse_id: Some(1),
                charging_profile_purpose: Some(
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                ),
                stack_level: Some(1),
            }),
        };
        let schema = include_str!("json_schemas/ClearChargingProfileRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("json_schemas/ClearChargingProfileResponse.json");
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
        let schema = include_str!("json_schemas/ClearDisplayMessageRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("json_schemas/ClearDisplayMessageResponse.json");
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
            evse_id: Some(1),
        };
        let schema = include_str!("json_schemas/ClearedChargingLimitRequest.json");
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

        let schema = include_str!("json_schemas/ClearedChargingLimitResponse.json");
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
        let schema = include_str!("json_schemas/ClearVariableMonitoringRequest.json");
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
                status_info: Some(StatusInfoType {
                    reason_code: "".to_string(),
                    additional_info: Some("".to_string()),
                }),
            }],
        };
        let schema = include_str!("json_schemas/ClearVariableMonitoringResponse.json");
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
        let schema = include_str!("json_schemas/CostUpdatedRequest.json");
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
        let schema = include_str!("json_schemas/CostUpdatedResponse.json");
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
            customer_identifier: Some("customer_identifier".to_string()),
            id_token: Some(IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    additional_id_token: "additional_id_token".to_string(),
                    kind: "type".to_string(),
                }]),
            }),
            customer_certificate: Some(CertificateHashDataType {
                hash_algorithm: HashAlgorithmEnumType::SHA256,
                issuer_name_hash: "issuer_name".to_string(),
                issuer_key_hash: "issuer_key_hash".to_string(),
                serial_number: "serial_number".to_string(),
            }),
        };
        let schema = include_str!("json_schemas/CustomerInformationRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/CustomerInformationResponse.json");
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
            message_id: Some("message_id".to_string()),
            data: "data".to_string(),
            vendor_id: "vendor_id".to_string(),
        };
        let schema = include_str!("json_schemas/DataTransferRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/DataTransferResponse.json");
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
        let schema = include_str!("json_schemas/DeleteCertificateRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/DeleteCertificateResponse.json");
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
            request_id: Some(1),
        };
        let schema = include_str!("json_schemas/FirmwareStatusNotificationRequest.json");
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
        let schema = include_str!("json_schemas/FirmwareStatusNotificationResponse.json");
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
        let schema = include_str!("json_schemas/Get15118EVCertificateRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/Get15118EVCertificateResponse.json");
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
        let schema = include_str!("json_schemas/GetBaseReportRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetBaseReportResponse.json");
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
        let schema = include_str!("json_schemas/GetCertificateStatusRequest.json");
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
            ocsp_result: Some("ocsp_result".to_string()),
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetCertificateStatusResponse.json");
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
            evse_id: Some(1),
            charging_profile: ChargingProfileCriterionType {
                charging_profile_purpose: Some(
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                ),
                stack_level: Some(1),
                charging_profile_id: Some(vec![1]),
                charging_limit_source: Some(vec![ChargingLimitSourceEnumType::CSO]),
            },
        };
        let schema = include_str!("json_schemas/GetChargingProfilesRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetChargingProfilesResponse.json");
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
            charging_rate_unit: Some(ChargingRateUnitEnumType::W),
            evse_id: 0,
        };
        let schema = include_str!("json_schemas/GetCompositeScheduleRequest.json");
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
            schedule: Some(CompositeScheduleType {
                evse_id: 0,
                duration: 0,
                schedule_start: Utc::now(),
                charging_rate_unit: ChargingRateUnitEnumType::W,
                charging_schedule_period: vec![ChargingSchedulePeriodType {
                    start_period: 0,
                    limit: 0.0,
                    number_phases: Some(1),
                    phase_to_use: Some(1),
                }],
            }),
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetCompositeScheduleResponse.json");
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
            id: Some(vec![1]),
            request_id: 0,
            priority: Some(MessagePriorityEnumType::AlwaysFront),
            state: Some(MessageStateEnumType::Charging),
        };
        let schema = include_str!("json_schemas/GetDisplayMessagesRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetDisplayMessagesResponse.json");
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
            certificate_type: Some(vec![GetCertificateIdUseEnumType::CSMSRootCertificate]),
        };
        let schema = include_str!("json_schemas/GetInstalledCertificateIdsRequest.json");
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
            certificate_hash_data_chain: Some(vec![CertificateHashDataChainType {
                certificate_type: GetCertificateIdUseEnumType::V2GRootCertificate,
                certificate_hash_data: CertificateHashDataType {
                    hash_algorithm: HashAlgorithmEnumType::SHA256,
                    issuer_name_hash: "issuer_name_hash".to_string(),
                    issuer_key_hash: "issuer_key_hash".to_string(),
                    serial_number: "serial_number".to_string(),
                },
                child_certificate_hash_data: Some(vec![CertificateHashDataType {
                    hash_algorithm: HashAlgorithmEnumType::SHA256,
                    issuer_name_hash: "issuer_name_hash".to_string(),
                    issuer_key_hash: "issuer_key_hash".to_string(),
                    serial_number: "serial_number".to_string(),
                }]),
            }]),
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetInstalledCertificateIdsResponse.json");
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
        let schema = include_str!("json_schemas/GetLocalListVersionRequest.json");
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
        let schema = include_str!("json_schemas/GetLocalListVersionResponse.json");
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
            retries: Some(1),
            retry_interval: Some(1),
            log: LogParametersType {
                remote_location: "remote_location".to_string(),
                oldest_timestamp: Some(Utc::now()),
                latest_timestamp: Some(Utc::now()),
            },
        };
        let schema = include_str!("json_schemas/GetLogRequest.json");
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
            filename: Some("/filename".to_string()),
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetLogResponse.json");
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
            monitoring_criteria: Some(vec![MonitoringCriterionEnumType::DeltaMonitoring]),
            component_variable: Some(vec![ComponentVariableType {
                component: ComponentType {
                    name: "".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(1),
                    }),
                },
                variable: Some(VariableType {
                    name: "".to_string(),
                    instance: Some("instance".to_string()),
                }),
            }]),
        };
        let schema = include_str!("json_schemas/GetMonitoringReportRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetMonitoringReportResponse.json");
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
            component_criteria: Some(vec![ComponentCriterionEnumType::Active]),
            component_variable: Some(vec![ComponentVariableType {
                component: ComponentType {
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(1),
                    }),
                },
                variable: Some(VariableType {
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                }),
            }]),
        };
        let schema = include_str!("json_schemas/GetReportRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/GetReportResponse.json");
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
            transaction_id: Some("transaction_id".to_string()),
        };
        let schema = include_str!("json_schemas/GetTransactionStatusRequest.json");
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
            ongoing_indicator: Some(true),
            messages_in_queue: false,
        };
        let schema = include_str!("json_schemas/GetTransactionStatusResponse.json");
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
                attribute_type: Some(AttributeEnumType::MaxSet),
                component: ComponentType {
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(1),
                    }),
                },
                variable: VariableType {
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                },
            }],
        };
        let schema = include_str!("json_schemas/GetVariablesRequest.json");
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
                attribute_type: Some(AttributeEnumType::Actual),
                attribute_value: Some("attribute_value".to_string()),
                component: ComponentType {
                    name: "name".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
                attribute_status_info: Some(StatusInfoType {
                    reason_code: "".to_string(),
                    additional_info: Some("".to_string()),
                }),
            }],
        };
        let schema = include_str!("json_schemas/GetVariablesResponse.json");
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
        let schema = include_str!("json_schemas/HeartbeatRequest.json");
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
        let schema = include_str!("json_schemas/HeartbeatResponse.json");
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
        let schema = include_str!("json_schemas/InstallCertificateRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/InstallCertificateResponse.json");
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
            request_id: Some(1),
        };
        let schema = include_str!("json_schemas/LogStatusNotificationRequest.json");
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
        let schema = include_str!("json_schemas/LogStatusNotificationResponse.json");
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
                    context: Some(ReadingContextEnumType::SampleClock),
                    measurand: Some(MeasurandEnumType::CurrentExport),
                    phase: Some(PhaseEnumType::L1),
                    location: Some(LocationEnumType::Body),
                    signed_meter_value: Some(SignedMeterValueType {
                        signed_meter_data: "signed_meter_data".to_string(),
                        signing_method: "signing_method".to_string(),
                        encoding_method: "encoding_method".to_string(),
                        public_key: "public_key".to_string(),
                    }),
                    unit_of_measure: Some(UnitOfMeasureType {
                        unit: Some("unit".to_string()),
                        multiplier: Some(1),
                    }),
                }],
            }],
        };
        let schema = include_str!("json_schemas/MeterValuesRequest.json");
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
        let schema = include_str!("json_schemas/MeterValuesResponse.json");
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
            evse_id: Some(0),
            charging_limit: ChargingLimitType {
                charging_limit_source: ChargingLimitSourceEnumType::EMS,
                is_grid_critical: Some(false),
            },
            charging_schedule: Some(vec![ChargingScheduleType {
                id: 0,
                start_schedule: Some(Utc::now()),
                duration: Some(10),
                charging_rate_unit: ChargingRateUnitEnumType::W,
                min_charging_rate: Some(10.0),
                charging_schedule_period: vec![ChargingSchedulePeriodType {
                    start_period: 0,
                    limit: 0.0,
                    number_phases: Some(0),
                    phase_to_use: Some(0),
                }],
                sales_tariff: Some(SalesTariffType {
                    id: Some(0),
                    sales_tariff_description: Some("sales_tariff_description".to_string()),
                    num_e_price_levels: Some(0),
                    sales_tariff_entry: vec![SalesTariffEntryType {
                        e_price_level: Some(0),
                        relative_time_interval: RelativeTimeIntervalType {
                            start: 0,
                            duration: 0,
                        },
                        consumption_cost: Some(vec![ConsumptionCostType {
                            start_value: 0,
                            cost: vec![CostType {
                                cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                amount: 0,
                                amount_multiplier: Some(0),
                            }],
                        }]),
                    }],
                }),
            }]),
        };
        let schema = include_str!("json_schemas/NotifyChargingLimitRequest.json");
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
        let schema = include_str!("json_schemas/NotifyChargingLimitResponse.json");
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
            tbc: Some(false),
            seq_no: 0,
            generated_at: Utc::now(),
            request_id: 0,
        };
        let schema = include_str!("json_schemas/NotifyCustomerInformationRequest.json");
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
        let schema = include_str!("json_schemas/NotifyCustomerInformationResponse.json");
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
            tbc: Some(false),
            message_info: Some(vec![MessageInfoType {
                id: 0,
                priority: MessagePriorityEnumType::AlwaysFront,
                state: MessageStateEnumType::Charging,
                start_date_time: Some(Utc::now()),
                end_date_time: Some(Utc::now()),
                transaction_id: Some("transaction_id".to_string()),
                message: MessageContentType {
                    format: MessageFormatEnumType::ASCII,
                    language: Some("Swedish".to_string()),
                    content: "".to_string(),
                },
                display: Some(ComponentType {
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(1),
                    }),
                }),
            }]),
        };
        let schema = include_str!("json_schemas/NotifyDisplayMessagesRequest.json");
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
        let schema = include_str!("json_schemas/NotifyDisplayMessagesResponse.json");
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
            max_schedule_tuples: Some(0),
            evse_id: 0,
            charging_needs: ChargingNeedsType {
                requested_energy_transfer: EnergyTransferModeEnumType::DC,
                departure_time: Some(Utc::now()),
                ac_charging_parameters: Some(ACChargingParametersType {
                    energy_amount: 1,
                    ev_min_current: 1,
                    ev_max_current: 1,
                    ev_max_voltage: 1,
                }),
                dc_charging_parameters: Some(DCChargingParametersType {
                    ev_max_current: 1,
                    ev_max_voltage: 1,
                    energy_amount: Some(0),
                    ev_max_power: Some(0),
                    state_of_charge: Some(100),
                    ev_energy_capacity: Some(0),
                    full_soc: Some(100),
                    bulk_soc: Some(100),
                }),
            },
        };
        let schema = include_str!("json_schemas/NotifyEVChargingNeedsRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/NotifyEVChargingNeedsResponse.json");
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
                start_schedule: Some(Utc::now()),
                duration: Some(0),
                charging_rate_unit: ChargingRateUnitEnumType::W,
                min_charging_rate: Some(0.0),
                charging_schedule_period: vec![ChargingSchedulePeriodType {
                    start_period: 0,
                    limit: 0.0,
                    number_phases: Some(0),
                    phase_to_use: Some(0),
                }],
                sales_tariff: Some(SalesTariffType {
                    id: Some(0),
                    sales_tariff_description: Some("sales_tariff_description".to_string()),
                    num_e_price_levels: Some(0),
                    sales_tariff_entry: vec![SalesTariffEntryType {
                        e_price_level: Some(0),
                        relative_time_interval: RelativeTimeIntervalType {
                            start: 0,
                            duration: 0,
                        },
                        consumption_cost: Some(vec![ConsumptionCostType {
                            start_value: 0,
                            cost: vec![CostType {
                                cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                amount: 0,
                                amount_multiplier: Some(1),
                            }],
                        }]),
                    }],
                }),
            },
        };
        let schema = include_str!("json_schemas/NotifyEVChargingScheduleRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/NotifyEVChargingScheduleResponse.json");
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
            tbc: Some(false),
            seq_no: 0,
            event_data: vec![EventDataType {
                event_id: 0,
                timestamp: Utc::now(),
                trigger: EventTriggerEnumType::Alerting,
                cause: Some(0),
                actual_value: "".to_string(),
                tech_code: Some("tech_code".to_string()),
                tech_info: Some("tech_info".to_string()),
                cleared: Some(false),
                transaction_id: Some("transaction_id".to_string()),
                variable_monitoring_id: Some(0),
                event_notification_type: EventNotificationEnumType::HardWiredNotification,
                component: ComponentType {
                    name: "".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: Some("instance".to_string()),
                },
            }],
        };
        let schema = include_str!("json_schemas/NotifyEventRequest.json");
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
        let schema = include_str!("json_schemas/NotifyEventResponse.json");
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
            tbc: Some(true),
            seq_no: 0,
            generated_at: Utc::now(),
            monitor: Some(vec![MonitoringDataType {
                component: ComponentType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
                variable_monitoring: vec![VariableMonitoringType {
                    id: 0,
                    transaction: false,
                    value: 0.0,
                    kind: MonitorEnumType::UpperThreshold,
                    severity: 0,
                }],
            }]),
        };
        let schema = include_str!("json_schemas/NotifyMonitoringReportRequest.json");
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
        let schema = include_str!("json_schemas/NotifyMonitoringReportResponse.json");
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
            tbc: Some(false),
            seq_no: 0,
            generated_at: Utc::now(),
            report_data: Some(vec![ReportDataType {
                component: ComponentType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    name: "name".to_string(),
                    instance: Some("".to_string()),
                },
                variable_attribute: vec![VariableAttributeType {
                    kind: Some(AttributeEnumType::Actual),
                    value: Some("value".to_string()),
                    mutability: Some(MutabilityEnumType::ReadOnly),
                    persistent: Some(false),
                    constant: Some(false),
                }],
                variable_characteristics: Some(VariableCharacteristicsType {
                    unit: Some("unit".to_string()),
                    data_type: DataEnumType::String,
                    min_limit: Some(0.0),
                    max_limit: Some(0.0),
                    values_list: Some("values_list".to_string()),
                    supports_monitoring: false,
                }),
            }]),
        };
        let schema = include_str!("json_schemas/NotifyReportRequest.json");
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
        let test = NotifyReportResponse {};
        let schema = include_str!("json_schemas/NotifyReportResponse.json");
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
            retries: Some(0),
            checksum: "checksum".to_string(),
            request_id: 0,
            retry_interval: Some(0),
        };
        let schema = include_str!("json_schemas/PublishFirmwareRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/PublishFirmwareResponse.json");
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
            location: Some(vec!["location".to_string()]),
            request_id: Some(1),
        };
        let schema = include_str!("json_schemas/PublishFirmwareStatusNotificationRequest.json");
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
        let test = PublishFirmwareStatusNotificationResponse {};
        let schema = include_str!("json_schemas/PublishFirmwareStatusNotificationResponse.json");
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
            tbc: Some(true),
            evse_id: 0,
            charging_profile: vec![ChargingProfileType {
                id: 0,
                stack_level: 0,
                charging_profile_purpose:
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                charging_profile_kind: ChargingProfileKindEnumType::Absolute,
                recurrency_kind: Some(RecurrencyKindEnumType::Daily),
                valid_from: Some(Utc::now()),
                valid_to: Some(Utc::now()),
                transaction_id: Some("transaction_id".to_string()),
                charging_schedule: vec![ChargingScheduleType {
                    id: 0,
                    start_schedule: Some(Utc::now()),
                    duration: Some(1),
                    charging_rate_unit: ChargingRateUnitEnumType::W,
                    min_charging_rate: Some(1.0),
                    charging_schedule_period: vec![ChargingSchedulePeriodType {
                        start_period: 0,
                        limit: 0.0,
                        number_phases: Some(1),
                        phase_to_use: Some(4),
                    }],
                    sales_tariff: Some(SalesTariffType {
                        id: Some(1),
                        sales_tariff_description: Some("sales_tariff_description".to_string()),
                        num_e_price_levels: Some(1),
                        sales_tariff_entry: vec![SalesTariffEntryType {
                            e_price_level: Some(1),
                            relative_time_interval: RelativeTimeIntervalType {
                                start: 1,
                                duration: 100,
                            },
                            consumption_cost: Some(vec![ConsumptionCostType {
                                start_value: 0,
                                cost: vec![CostType {
                                    cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                    amount: 0,
                                    amount_multiplier: Some(1),
                                }],
                            }]),
                        }],
                    }),
                }],
            }],
        };
        let schema = include_str!("json_schemas/ReportChargingProfilesRequest.json");
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
        let test = ReportChargingProfilesResponse {};
        let schema = include_str!("json_schemas/ReportChargingProfilesResponse.json");
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
            evse_id: Some(0),
            remote_start_id: 0,
            id_token: IdTokenType {
                id_token: "id_token".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            },
            charging_profile: Some(ChargingProfileType {
                id: 0,
                stack_level: 0,
                charging_profile_purpose:
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                charging_profile_kind: ChargingProfileKindEnumType::Absolute,
                recurrency_kind: Some(RecurrencyKindEnumType::Daily),
                valid_from: Some(Utc::now()),
                valid_to: Some(Utc::now()),
                transaction_id: Some("transaction_id".to_string()),
                charging_schedule: vec![ChargingScheduleType {
                    id: 0,
                    start_schedule: Some(Utc::now()),
                    duration: Some(1),
                    charging_rate_unit: ChargingRateUnitEnumType::W,
                    min_charging_rate: Some(0.1),
                    charging_schedule_period: vec![ChargingSchedulePeriodType {
                        start_period: 0,
                        limit: 0.0,
                        number_phases: Some(1),
                        phase_to_use: Some(1),
                    }],
                    sales_tariff: Some(SalesTariffType {
                        id: Some(1),
                        sales_tariff_description: Some("".to_string()),
                        num_e_price_levels: Some(2),
                        sales_tariff_entry: vec![SalesTariffEntryType {
                            e_price_level: Some(1),
                            relative_time_interval: RelativeTimeIntervalType {
                                start: 0,
                                duration: 0,
                            },
                            consumption_cost: Some(vec![ConsumptionCostType {
                                start_value: 0,
                                cost: vec![CostType {
                                    cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                    amount: 0,
                                    amount_multiplier: Some(1),
                                }],
                            }]),
                        }],
                    }),
                }],
            }),
            group_id_token: Some(IdTokenType {
                id_token: "id_token".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            }),
        };
        let schema = include_str!("json_schemas/RequestStartTransactionRequest.json");
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
            transaction_id: Some("".to_string()),
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/RequestStartTransactionResponse.json");
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
            transaction_id: "".to_string(),
        };
        let schema = include_str!("json_schemas/RequestStopTransactionRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/RequestStopTransactionResponse.json");
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
            reservation_update_status: ReservationUpdateStatusEnumType::Expired,
        };
        let schema = include_str!("json_schemas/ReservationStatusUpdateRequest.json");
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
        let test = ReservationStatusUpdateResponse {};
        let schema = include_str!("json_schemas/ReservationStatusUpdateResponse.json");
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
            connector_type: Some(ConnectorEnumType::CCCS1),
            evse_id: Some(0),
            id_token: IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            },
            group_id_token: Some(IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            }),
        };
        let schema = include_str!("json_schemas/ReserveNowRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/ReserveNowResponse.json");
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
            evse_id: Some(0),
        };
        let schema = include_str!("json_schemas/ResetRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/ResetResponse.json");
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
            tech_info: Some("".to_string()),
        };
        let schema = include_str!("json_schemas/SecurityEventNotificationRequest.json");
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
        let test = SecurityEventNotificationResponse {};
        let schema = include_str!("json_schemas/SecurityEventNotificationResponse.json");
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
            local_authorization_list: Some(vec![AuthorizationData {
                id_token_info: Some(IdTokenInfoType {
                    status: AuthorizationStatusEnumType::Accepted,
                    cache_expiry_date_time: Some(Utc::now()),
                    charging_priority: Some(0),
                    language1: Some("lang1".to_string()),
                    evse_id: Some(vec![1, 2, 3]),
                    language2: Some("lang2".to_string()),
                    group_id_token: Some(IdTokenType {
                        id_token: "id_token".to_string(),
                        kind: IdTokenEnumType::Central,
                        additional_info: Some(vec![AdditionalInfoType {
                            additional_id_token: "additional_id_token".to_string(),
                            kind: "type".to_string(),
                        }]),
                    }),
                    personal_message: Some(MessageContentType {
                        format: MessageFormatEnumType::ASCII,
                        language: Some("English".to_string()),
                        content: "Hello, world!".to_string(),
                    }),
                }),
                id_token: IdTokenType {
                    id_token: "".to_string(),
                    kind: IdTokenEnumType::Central,
                    additional_info: Some(vec![AdditionalInfoType {
                        additional_id_token: "additional_id_token".to_string(),
                        kind: "type".to_string(),
                    }]),
                },
            }]),
        };
        let schema = include_str!("json_schemas/SendLocalListRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/SendLocalListResponse.json");
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
                charging_profile_purpose:
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                charging_profile_kind: ChargingProfileKindEnumType::Absolute,
                recurrency_kind: Some(RecurrencyKindEnumType::Daily),
                valid_from: Some(Utc::now()),
                valid_to: Some(Utc::now()),
                transaction_id: Some("".to_string()),
                charging_schedule: vec![ChargingScheduleType {
                    id: 0,
                    start_schedule: Some(Utc::now()),
                    duration: Some(0),
                    charging_rate_unit: ChargingRateUnitEnumType::W,
                    min_charging_rate: Some(0.0),
                    charging_schedule_period: vec![ChargingSchedulePeriodType {
                        start_period: 0,
                        limit: 0.0,
                        number_phases: Some(0),
                        phase_to_use: Some(0),
                    }],
                    sales_tariff: Some(SalesTariffType {
                        id: Some(0),
                        sales_tariff_description: Some("".to_string()),
                        num_e_price_levels: Some(0),
                        sales_tariff_entry: vec![SalesTariffEntryType {
                            e_price_level: Some(0),
                            relative_time_interval: RelativeTimeIntervalType {
                                start: 0,
                                duration: 0,
                            },
                            consumption_cost: Some(vec![ConsumptionCostType {
                                start_value: 0,
                                cost: vec![CostType {
                                    cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                    amount: 0,
                                    amount_multiplier: Some(0),
                                }],
                            }]),
                        }],
                    }),
                }],
            },
        };
        let schema = include_str!("json_schemas/SetChargingProfileRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/SetChargingProfileResponse.json");
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
                start_date_time: Some(Utc::now()),
                end_date_time: Some(Utc::now()),
                transaction_id: Some("".to_string()),
                message: MessageContentType {
                    format: MessageFormatEnumType::ASCII,
                    language: Some("lang".to_string()),
                    content: "".to_string(),
                },
                display: Some(ComponentType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                }),
            },
        };
        let schema = include_str!("json_schemas/SetDisplayMessageRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/SetDisplayMessageResponse.json");
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
            monitoring_base: MonitoringBaseEnumType::All,
        };
        let schema = include_str!("json_schemas/SetMonitoringBaseRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/SetMonitoringBaseResponse.json");
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
        let test = SetMonitoringLevelRequest { severity: 0 };
        let schema = include_str!("json_schemas/SetMonitoringLevelRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/SetMonitoringLevelResponse.json");
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
                vpn: Some(VPNType {
                    server: "server".to_string(),
                    user: "user".to_string(),
                    group: Some("group".to_string()),
                    password: "password".to_string(),
                    key: "key".to_string(),
                    kind: VPNEnumType::IKEv2,
                }),
                apn: Some(APNType {
                    apn: "apn".to_string(),
                    apn_user_name: Some("apn_user_name".to_string()),
                    apn_password: Some("apn_password".to_string()),
                    sim_pin: Some(1),
                    preferred_network: Some("6chars".to_string()),
                    use_only_preferred_network: Some(false),
                    apn_authentication: APNAuthenticationEnumType::CHAP,
                }),
            },
        };
        let schema = include_str!("json_schemas/SetNetworkProfileRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/SetNetworkProfileResponse.json");
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
                id: Some(0),
                transaction: Some(false),
                value: 0.0,
                kind: MonitorEnumType::UpperThreshold,
                severity: 0,
                component: ComponentType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
            }],
        };
        let schema = include_str!("json_schemas/SetVariableMonitoringRequest.json");
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
                id: Some(0),
                status: SetMonitoringStatusEnumType::Accepted,
                kind: MonitorEnumType::UpperThreshold,
                severity: 0,
                component: ComponentType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
                status_info: Some(StatusInfoType {
                    reason_code: "".to_string(),
                    additional_info: Some("".to_string()),
                }),
            }],
        };
        let schema = include_str!("json_schemas/SetVariableMonitoringResponse.json");
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
                attribute_type: Some(AttributeEnumType::Actual),
                attribute_value: "".to_string(),
                component: ComponentType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
            }],
        };
        let schema = include_str!("json_schemas/SetVariablesRequest.json");
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
                attribute_type: Some(AttributeEnumType::Actual),
                attribute_status: SetVariableStatusEnumType::Accepted,
                component: ComponentType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
                attribute_status_info: Some(StatusInfoType {
                    reason_code: "".to_string(),
                    additional_info: Some("".to_string()),
                }),
            }],
        };
        let schema = include_str!("json_schemas/SetVariablesResponse.json");
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
            certificate_type: Some(CertificateSigningUseEnumType::ChargingStationCertificate),
        };
        let schema = include_str!("json_schemas/SignCertificateRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/SignCertificateResponse.json");
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
            connector_id: 0,
        };
        let schema = include_str!("json_schemas/StatusNotificationRequest.json");
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
        let test = StatusNotificationResponse {};
        let schema = include_str!("json_schemas/StatusNotificationResponse.json");
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
            offline: Some(false),
            number_of_phases_used: Some(0),
            cable_max_current: Some(0),
            reservation_id: Some(0),
            transaction_info: TransactionType {
                transaction_id: "".to_string(),
                charging_state: Some(ChargingStateEnumType::Charging),
                time_spent_charging: Some(0),
                stopped_reason: Some(ReasonEnumType::DeAuthorized),
                remote_start_id: Some(0),
            },
            id_token: Some(IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            }),
            evse: Some(EVSEType {
                id: 0,
                connector_id: Some(0),
            }),
            meter_value: Some(vec![MeterValueType {
                timestamp: Utc::now(),
                sampled_value: vec![SampledValueType {
                    value: 0.0,
                    context: Some(ReadingContextEnumType::InterruptionBegin),
                    measurand: Some(MeasurandEnumType::CurrentExport),
                    phase: Some(PhaseEnumType::L1),
                    location: Some(LocationEnumType::Body),
                    signed_meter_value: Some(SignedMeterValueType {
                        signed_meter_data: "signed_meter_data".to_string(),
                        signing_method: "signing_method".to_string(),
                        encoding_method: "encoding_method".to_string(),
                        public_key: "public_key".to_string(),
                    }),
                    unit_of_measure: Some(UnitOfMeasureType {
                        unit: Some("unit".to_string()),
                        multiplier: Some(0),
                    }),
                }],
            }]),
        };
        let schema = include_str!("json_schemas/TransactionEventRequest.json");
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
            total_cost: Some(0.0),
            charging_priority: Some(0),
            id_token_info: Some(IdTokenInfoType {
                status: AuthorizationStatusEnumType::Accepted,
                cache_expiry_date_time: Some(Utc::now()),
                charging_priority: Some(0),
                language1: Some("".to_string()),
                evse_id: Some(vec![1]),
                language2: Some("".to_string()),
                group_id_token: Some(IdTokenType {
                    id_token: "".to_string(),
                    kind: IdTokenEnumType::Central,
                    additional_info: Some(vec![AdditionalInfoType {
                        additional_id_token: "additional_id_token".to_string(),
                        kind: "type".to_string(),
                    }]),
                }),
                personal_message: Some(MessageContentType {
                    format: MessageFormatEnumType::ASCII,
                    language: Some("language".to_string()),
                    content: "content".to_string(),
                }),
            }),
            updated_personal_message: Some(MessageContentType {
                format: MessageFormatEnumType::ASCII,
                language: Some("language".to_string()),
                content: "content".to_string(),
            }),
        };
        let schema = include_str!("json_schemas/TransactionEventResponse.json");
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
            evse: Some(EVSEType {
                id: 0,
                connector_id: Some(0),
            }),
        };
        let schema = include_str!("json_schemas/TriggerMessageRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/TriggerMessageResponse.json");
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
            connector_id: 0,
        };
        let schema = include_str!("json_schemas/UnlockConnectorRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/UnlockConnectorResponse.json");
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
            checksum: "".to_string(),
        };
        let schema = include_str!("json_schemas/UnpublishFirmwareRequest.json");
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
            status: UnpublishFirmwareStatusEnumType::DownloadOngoing,
        };
        let schema = include_str!("json_schemas/UnpublishFirmwareResponse.json");
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
            retries: Some(0),
            retry_interval: Some(0),
            request_id: 0,
            firmware: FirmwareType {
                location: "".to_string(),
                retrieve_date_time: Utc::now(),
                install_date_time: Some(Utc::now()),
                signing_certificate: Some("signing_certificate".to_string()),
                signature: Some("signature".to_string()),
            },
        };
        let schema = include_str!("json_schemas/UpdateFirmwareRequest.json");
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
            status_info: Some(StatusInfoType {
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("json_schemas/UpdateFirmwareResponse.json");
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
