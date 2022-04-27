#[cfg(test)]
mod tests {
    use crate::v2_0_1::datatypes::charging_station_type::ChargingStationType;
    use crate::v2_0_1::datatypes::clear_monitoring_result_type::ClearMonitoringResultType;
    use crate::v2_0_1::datatypes::id_token_info_type::IdTokenInfoType;
    use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
    use crate::v2_0_1::enumerations::authorization_status_enum_type::AuthorizationStatusEnumType;
    use crate::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;
    use crate::v2_0_1::enumerations::cancel_reservation_status_enum_type::CancelReservationStatusEnumType;
    use crate::v2_0_1::enumerations::certificate_signed_status_enum_type::CertificateSignedStatusEnumType;
    use crate::v2_0_1::enumerations::change_availability_status_enum_type::ChangeAvailabilityStatusEnumType;
    use crate::v2_0_1::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;
    use crate::v2_0_1::enumerations::clear_cache_status_enum_type::ClearCacheStatusEnumType;
    use crate::v2_0_1::enumerations::clear_charging_profile_status_enum_type::ClearChargingProfileStatusEnumType;
    use crate::v2_0_1::enumerations::clear_message_status_enum_type::ClearMessageStatusEnumType;
    use crate::v2_0_1::enumerations::clear_monitoring_status_enum_type::ClearMonitoringStatusEnumType;
    use crate::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;
    use crate::v2_0_1::enumerations::operational_status_enum_type::OperationalStatusEnumType;
    use crate::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType;
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
    use chrono::Utc;
    use jsonschema::JSONSchema;

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

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/AuthorizeRequest.json");
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

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/AuthorizeResponse.json");
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
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/BootNotificationRequest.json");
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

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/BootNotificationResponse.json");
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
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/CancelReservationRequest.json");
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

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/CancelReservationResponse.json");
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
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/CertificateSignedRequest.json");
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

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/CertificateSignedResponse.json");
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
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/ChangeAvailabilityRequest.json");
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
            include_str!("../../../../schemas/ocpp/v2.0.1/ChangeAvailabilityResponse.json");
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
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/ClearCacheRequest.json");
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

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/ClearCacheResponse.json");
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
            include_str!("../../../../schemas/ocpp/v2.0.1/ClearChargingProfileRequest.json");
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
            include_str!("../../../../schemas/ocpp/v2.0.1/ClearChargingProfileResponse.json");
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
            include_str!("../../../../schemas/ocpp/v2.0.1/ClearDisplayMessageRequest.json");
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
            include_str!("../../../../schemas/ocpp/v2.0.1/ClearDisplayMessageResponse.json");
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
            include_str!("../../../../schemas/ocpp/v2.0.1/ClearedChargingLimitRequest.json");
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
            include_str!("../../../../schemas/ocpp/v2.0.1/ClearedChargingLimitResponse.json");
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
            include_str!("../../../../schemas/ocpp/v2.0.1/ClearVariableMonitoringRequest.json");
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
            status_info: None}
            ]
        };
        let schema =
            include_str!("../../../../schemas/ocpp/v2.0.1/ClearVariableMonitoringResponse.json");
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
    fn validate_cost_updated_request() {}
    #[test]
    fn validate_cost_updated_response() {}
    #[test]
    fn validate_customer_information_request() {}
    #[test]
    fn validate_customer_information_response() {}
    #[test]
    fn validate_data_transfer_request() {}
    #[test]
    fn validate_data_transfer_response() {}
    #[test]
    fn validate_delete_certificate_request() {}
    #[test]
    fn validate_delete_certificate_response() {}
    #[test]
    fn validate_firmware_status_notification_request() {}
    #[test]
    fn validate_firmware_status_notification_response() {}
    #[test]
    fn validate_get15118ev_certificate_request() {}
    #[test]
    fn validate_get15118ev_certificate_response() {}
    #[test]
    fn validate_get_base_report_request() {}
    #[test]
    fn validate_get_base_report_response() {}
    #[test]
    fn validate_get_certificate_status_request() {}
    #[test]
    fn validate_get_certificate_status_response() {}
    #[test]
    fn validate_get_charging_profiles_request() {}
    #[test]
    fn validate_get_charging_profiles_response() {}
    #[test]
    fn validate_get_composite_schedule_request() {}
    #[test]
    fn validate_get_composite_schedule_response() {}
    #[test]
    fn validate_get_display_messages_request() {}
    #[test]
    fn validate_get_display_messages_response() {}
    #[test]
    fn validate_get_installed_certificate_ids_request() {}
    #[test]
    fn validate_get_installed_certificate_ids_response() {}
    #[test]
    fn validate_get_local_list_version_request() {}
    #[test]
    fn validate_get_local_list_version_response() {}
    #[test]
    fn validate_get_log_request() {}
    #[test]
    fn validate_get_log_response() {}
    #[test]
    fn validate_get_monitoring_report_request() {}
    #[test]
    fn validate_get_monitoring_report_response() {}
    #[test]
    fn validate_get_report_request() {}
    #[test]
    fn validate_get_report_response() {}
    #[test]
    fn validate_get_transaction_status_request() {}
    #[test]
    fn validate_get_transaction_status_response() {}
    #[test]
    fn validate_get_variables_request() {}
    #[test]
    fn validate_get_variables_response() {}
    #[test]
    fn validate_heartbeat_request() {}
    #[test]
    fn validate_heartbeat_response() {}
    #[test]
    fn validate_install_certificate_request() {}
    #[test]
    fn validate_install_certificate_response() {}
    #[test]
    fn validate_log_status_notification_request() {}
    #[test]
    fn validate_log_status_notification_response() {}
    #[test]
    fn validate_meter_values_request() {}
    #[test]
    fn validate_meter_values_response() {}
    #[test]
    fn validate_notify_charging_limit_request() {}
    #[test]
    fn validate_notify_charging_limit_response() {}
    #[test]
    fn validate_notify_customer_information_request() {}
    #[test]
    fn validate_notify_customer_information_response() {}
    #[test]
    fn validate_notify_display_messages_request() {}
    #[test]
    fn validate_notify_display_messages_response() {}
    #[test]
    fn validate_notify_ev_charging_needs_request() {}
    #[test]
    fn validate_notify_ev_charging_needs_response() {}
    #[test]
    fn validate_notify_ev_charging_schedule_request() {}
    #[test]
    fn validate_notify_ev_charging_schedule_response() {}
    #[test]
    fn validate_notify_event_request() {}
    #[test]
    fn validate_notify_event_response() {}
    #[test]
    fn validate_notify_monitoring_report_request() {}
    #[test]
    fn validate_notify_monitoring_report_response() {}
    #[test]
    fn validate_notify_report_request() {}
    #[test]
    fn validate_notify_report_response() {}
    #[test]
    fn validate_publish_firmware_request() {}
    #[test]
    fn validate_publish_firmware_response() {}
    #[test]
    fn validate_publish_firmware_status_notification_request() {}
    #[test]
    fn validate_publish_firmware_status_notification_response() {}
    #[test]
    fn validate_report_charging_profiles_request() {}
    #[test]
    fn validate_report_charging_profiles_response() {}
    #[test]
    fn validate_request_start_transaction_request() {}
    #[test]
    fn validate_request_start_transaction_response() {}
    #[test]
    fn validate_request_stop_transaction_request() {}
    #[test]
    fn validate_request_stop_transaction_response() {}
    #[test]
    fn validate_reservation_status_update_request() {}
    #[test]
    fn validate_reservation_status_update_response() {}
    #[test]
    fn validate_reserve_now_request() {}
    #[test]
    fn validate_reserve_now_response() {}
    #[test]
    fn validate_reset_request() {}
    #[test]
    fn validate_reset_response() {}
    #[test]
    fn validate_security_event_notification_request() {}
    #[test]
    fn validate_security_event_notification_response() {}
    #[test]
    fn validate_send_local_list_request() {}
    #[test]
    fn validate_send_local_list_response() {}
    #[test]
    fn validate_set_charging_profile_request() {}
    #[test]
    fn validate_set_charging_profile_response() {}
    #[test]
    fn validate_set_display_message_request() {}
    #[test]
    fn validate_set_display_message_response() {}
    #[test]
    fn validate_set_monitoring_base_request() {}
    #[test]
    fn validate_set_monitoring_base_response() {}
    #[test]
    fn validate_set_monitoring_level_request() {}
    #[test]
    fn validate_set_monitoring_level_response() {}
    #[test]
    fn validate_set_network_profile_request() {}
    #[test]
    fn validate_set_network_profile_response() {}
    #[test]
    fn validate_set_variable_monitoring_request() {}
    #[test]
    fn validate_set_variable_monitoring_response() {}
    #[test]
    fn validate_set_variables_request() {}
    #[test]
    fn validate_set_variables_response() {}
    #[test]
    fn validate_sign_certificate_request() {}
    #[test]
    fn validate_sign_certificate_response() {}
    #[test]
    fn validate_status_notification_request() {}
    #[test]
    fn validate_status_notification_response() {}
    #[test]
    fn validate_transaction_event_request() {}
    #[test]
    fn validate_transaction_event_response() {}
    #[test]
    fn validate_trigger_message_request() {}
    #[test]
    fn validate_trigger_message_response() {}
    #[test]
    fn validate_unlock_connector_request() {}
    #[test]
    fn validate_unlock_connector_response() {}
    #[test]
    fn validate_unpublish_firmware_request() {}
    #[test]
    fn validate_unpublish_firmware_response() {}
    #[test]
    fn validate_update_firmware_request() {}
    #[test]
    fn validate_update_firmware_response() {}
}
