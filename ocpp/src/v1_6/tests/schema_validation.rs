#[cfg(test)]
mod tests {
    use chrono::Utc;
    use jsonschema::JSONSchema;
    use crate::v1_6::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
    use crate::v1_6::messages::boot_notification::{BootNotificationRequest, BootNotificationResponse};
    use crate::v1_6::messages::cancel_reservation::{CancelReservationRequest, CancelReservationResponse};
    use crate::v1_6::messages::change_availability::{ChangeAvailabilityRequest, ChangeAvailabilityResponse};
    use crate::v1_6::messages::change_configuration::{ChangeConfigurationRequest, ChangeConfigurationResponse};
    use crate::v1_6::messages::clear_cache::{ClearCacheRequest, ClearCacheResponse};
    use crate::v1_6::messages::clear_charging_profile::{ClearChargingProfileRequest, ClearChargingProfileResponse};
    use crate::v1_6::messages::data_transfer::{DataTransferRequest, DataTransferResponse};
    use crate::v1_6::messages::diagnostics_status_notification::{DiagnosticsStatusNotificationRequest, DiagnosticsStatusNotificationResponse};
    use crate::v1_6::messages::firmware_status_notification::{FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse};
    use crate::v1_6::messages::get_composite_schedule::{GetCompositeScheduleRequest, GetCompositeScheduleResponse};
    use crate::v1_6::messages::get_configuration::{GetConfigurationRequest, GetConfigurationResponse};
    use crate::v1_6::messages::get_diagnostics::{GetDiagnosticsRequest, GetDiagnosticsResponse};
    use crate::v1_6::messages::get_local_list_version::{GetLocalListVersionRequest, GetLocalListVersionResponse};
    use crate::v1_6::types::{AuthorizationStatus, AvailabilityStatus, AvailabilityType, CancelReservationStatus, ClearCacheStatus, ClearChargingProfileStatus, ConfigurationStatus, DataTransferStatus, DiagnosticsStatus, FirmwareStatus, GetCompositeScheduleStatus, IdTagInfo, RegistrationStatus};

    #[test]
    fn validate_authorize() {
        let test = AuthorizeRequest {
            id_tag: "".to_string()
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/Authorize.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            id_tag_info: IdTagInfo {
                expiry_date: None,
                parent_id_tag: None,
                status: AuthorizationStatus::Accepted
            }
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/AuthorizeResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_boot_notification() {
        let test = BootNotificationRequest {
            charge_box_serial_number: None,
            charge_point_model: "".to_string(),
            charge_point_serial_number: None,
            charge_point_vendor: "".to_string(),
            firmware_version: None,
            iccid: None,
            imsi: None,
            meter_serial_number: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/BootNotification.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            interval: 0,
            status: RegistrationStatus::Accepted
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/BootNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_cancel_reservation() {
        let test = CancelReservationRequest {
            reservation_id: 0
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/CancelReservation.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: CancelReservationStatus::Accepted
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/CancelReservationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_change_availability() {
        let test = ChangeAvailabilityRequest {
            connector_id: 0,
            kind: AvailabilityType::Inoperative
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/ChangeAvailability.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: AvailabilityStatus::Accepted
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/ChangeAvailabilityResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_change_configuration() {
        let test = ChangeConfigurationRequest {
            key: "".to_string(),
            value: "".to_string()
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/ChangeConfiguration.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_change_configuration_response() {
        let test = ChangeConfigurationResponse {
            status: ConfigurationStatus::Accepted
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/ChangeConfigurationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_clear_cache() {
        let test = ClearCacheRequest {
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/ClearCache.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: ClearCacheStatus::Accepted
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/ClearCacheResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_clear_charging_profile() {
        let test = ClearChargingProfileRequest {
            id: None,
            connector_id: None,
            charging_profile_purpose: None,
            stack_level: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/ClearChargingProfile.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: ClearChargingProfileStatus::Accepted
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/ClearChargingProfileResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_data_transfer() {
        let test = DataTransferRequest {
            vendor_string: "".to_string(),
            message_id: None,
            data: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/DataTransfer.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: DataTransferStatus::Accepted,
            data: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/DataTransferResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_diagnostics_status_notification() {
        let test = DiagnosticsStatusNotificationRequest {
            status: DiagnosticsStatus::Idle
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/DiagnosticsStatusNotification.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_diagnostics_status_notification_response() {
        let test = DiagnosticsStatusNotificationResponse {
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/DiagnosticsStatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_firmware_status_notification() {
        let test = FirmwareStatusNotificationRequest {
            status: FirmwareStatus::Downloaded
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/FirmwareStatusNotification.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
        let test = FirmwareStatusNotificationResponse {
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/FirmwareStatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_composite_schedule() {
        let test = GetCompositeScheduleRequest {
            connector_id: 0,
            duration: 0,
            charging_rate_unit: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/GetCompositeSchedule.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: GetCompositeScheduleStatus::Accepted,
            connector_id: None,
            schedule_start: None,
            charging_schedule: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/GetCompositeScheduleResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_configuration() {
        let test = GetConfigurationRequest {
            key: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/GetConfiguration.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_configuration_response() {
        let test = GetConfigurationResponse {
            configuration_key: None,
            unknown_key: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/GetConfigurationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_diagnostics() {
        let test = GetDiagnosticsRequest {
            location: "https://codelabs.se/".to_string(),
            retries: None,
            retry_interval: None,
            start_time: None,
            stop_time: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/GetDiagnostics.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_diagnostics_response() {
        let test = GetDiagnosticsResponse {
            file_name: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/GetDiagnosticsResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_local_list_version() {
        let test = GetLocalListVersionRequest {
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/GetLocalListVersion.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
        let test = GetLocalListVersionResponse {
            list_version: 0
        };

        let schema = include_str!("../../../../schemas/ocpp/v1.6/json/GetLocalListVersionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_heartbeat() {}
    #[test]
    fn validate_heartbeat_response() {}
    #[test]
    fn validate_meter_values() {}
    #[test]
    fn validate_meter_values_response() {}
    #[test]
    fn validate_ocpp_1() {}
    #[test]
    fn validate_remote_start_transaction() {}
    #[test]
    fn validate_remote_start_transaction_response() {}
    #[test]
    fn validate_remote_stop_transaction() {}
    #[test]
    fn validate_remote_stop_transaction_response() {}
    #[test]
    fn validate_reserve_now() {}
    #[test]
    fn validate_reserve_now_response() {}
    #[test]
    fn validate_reset() {}
    #[test]
    fn validate_reset_response() {}
    #[test]
    fn validate_send_local_list() {}
    #[test]
    fn validate_send_local_list_response() {}
    #[test]
    fn validate_set_charging_profile() {}
    #[test]
    fn validate_set_charging_profile_response() {}
    #[test]
    fn validate_start_transaction() {}
    #[test]
    fn validate_start_transaction_response() {}
    #[test]
    fn validate_status_notification() {}
    #[test]
    fn validate_status_notification_response() {}
    #[test]
    fn validate_stop_transaction() {}
    #[test]
    fn validate_stop_transaction_response() {}
    #[test]
    fn validate_trigger_message() {}
    #[test]
    fn validate_trigger_message_response() {}
    #[test]
    fn validate_unlock_connector() {}
    #[test]
    fn validate_unlock_connector_response() {}
    #[test]
    fn validate_update_firmware() {}
    #[test]
    fn validate_update_firmware_response() {}
}