#[cfg(test)]
mod tests {
    use crate::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
    use crate::messages::boot_notification::{BootNotificationRequest, BootNotificationResponse};
    use crate::messages::cancel_reservation::{
        CancelReservationRequest, CancelReservationResponse,
    };
    use crate::messages::change_availability::{
        ChangeAvailabilityRequest, ChangeAvailabilityResponse,
    };
    use crate::messages::change_configuration::{
        ChangeConfigurationRequest, ChangeConfigurationResponse,
    };
    use crate::messages::clear_cache::{ClearCacheRequest, ClearCacheResponse};
    use crate::messages::clear_charging_profile::{
        ClearChargingProfileRequest, ClearChargingProfileResponse,
    };
    use crate::messages::data_transfer::{DataTransferRequest, DataTransferResponse};
    use crate::messages::diagnostics_status_notification::{
        DiagnosticsStatusNotificationRequest, DiagnosticsStatusNotificationResponse,
    };
    use crate::messages::firmware_status_notification::{
        FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
    };
    use crate::messages::get_composite_schedule::{
        GetCompositeScheduleRequest, GetCompositeScheduleResponse,
    };
    use crate::messages::get_configuration::{GetConfigurationRequest, GetConfigurationResponse};
    use crate::messages::get_diagnostics::{GetDiagnosticsRequest, GetDiagnosticsResponse};
    use crate::messages::get_local_list_version::{
        GetLocalListVersionRequest, GetLocalListVersionResponse,
    };
    use crate::messages::heart_beat::{HeartbeatRequest, HeartbeatResponse};
    use crate::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
    use crate::messages::remote_start_transaction::{
        RemoteStartTransactionRequest, RemoteStartTransactionResponse,
    };
    use crate::messages::remote_stop_transaction::{
        RemoteStopTransactionRequest, RemoteStopTransactionResponse,
    };
    use crate::messages::reserve_now::{ReserveNowRequest, ReserveNowResponse};
    use crate::messages::reset::{ResetRequest, ResetResponse};
    use crate::messages::send_local_list::{SendLocalListRequest, SendLocalListResponse};
    use crate::messages::set_charging_profile::{
        SetChargingProfileRequest, SetChargingProfileResponse,
    };
    use crate::messages::start_transaction::{StartTransactionRequest, StartTransactionResponse};
    use crate::messages::status_notification::{
        StatusNotificationRequest, StatusNotificationResponse,
    };
    use crate::messages::stop_transaction::{StopTransactionRequest, StopTransactionResponse};
    use crate::messages::trigger_message::{TriggerMessageRequest, TriggerMessageResponse};
    use crate::messages::unlock_connector::{UnlockConnectorRequest, UnlockConnectorResponse};
    use crate::messages::update_firmware::{UpdateFirmwareRequest, UpdateFirmwareResponse};
    use crate::types::{
        AuthorizationStatus, AvailabilityStatus, AvailabilityType, CancelReservationStatus,
        ChargePointErrorCode, ChargePointStatus, ChargingProfile, ChargingProfileKindType,
        ChargingProfilePurposeType, ChargingProfileStatus, ChargingRateUnitType, ChargingSchedule,
        ChargingSchedulePeriod, ClearCacheStatus, ClearChargingProfileStatus, ConfigurationStatus,
        DataTransferStatus, DiagnosticsStatus, FirmwareStatus, GetCompositeScheduleStatus,
        IdTagInfo, KeyValue, MessageTrigger, MeterValue, RegistrationStatus, RemoteStartStopStatus,
        ReservationStatus, ResetRequestStatus, ResetResponseStatus, SampledValue,
        TriggerMessageStatus, UnlockStatus, UpdateStatus, UpdateType,
    };
    use chrono::Utc;
    use jsonschema::JSONSchema;

    #[test]
    fn validate_authorize() {
        let test = AuthorizeRequest {
            id_tag: "".to_string(),
        };

        let schema = include_str!("json_schemas/json/Authorize.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
                expiry_date: Some(Utc::now()),
                parent_id_tag: Some("".to_string()),
                status: AuthorizationStatus::Accepted,
            },
        };

        let schema = include_str!("json_schemas/json/AuthorizeResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            charge_box_serial_number: Some("".to_string()),
            charge_point_model: "".to_string(),
            charge_point_serial_number: Some("charge_point_serial".to_string()),
            charge_point_vendor: "".to_string(),
            firmware_version: Some("".to_string()),
            iccid: Some("".to_string()),
            imsi: Some("".to_string()),
            meter_serial_number: Some("".to_string()),
            meter_type: Some("".to_string()),
        };

        let schema = include_str!("json_schemas/json/BootNotification.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: RegistrationStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/BootNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
        let test = CancelReservationRequest { reservation_id: 0 };

        let schema = include_str!("json_schemas/json/CancelReservation.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: CancelReservationStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/CancelReservationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            kind: AvailabilityType::Inoperative,
        };

        let schema = include_str!("json_schemas/json/ChangeAvailability.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: AvailabilityStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/ChangeAvailabilityResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            value: "".to_string(),
        };

        let schema = include_str!("json_schemas/json/ChangeConfiguration.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: ConfigurationStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/ChangeConfigurationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
        let test = ClearCacheRequest {};

        let schema = include_str!("json_schemas/json/ClearCache.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: ClearCacheStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/ClearCacheResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            id: Some(1),
            connector_id: Some(1),
            charging_profile_purpose: Some(ChargingProfilePurposeType::TxProfile),
            stack_level: Some(1),
        };

        let schema = include_str!("json_schemas/json/ClearChargingProfile.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: ClearChargingProfileStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/ClearChargingProfileResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            vendor_string: "vendor_string".to_string(),
            message_id: Some("message_id".to_string()),
            data: Some("data".to_string()),
        };

        let schema = include_str!("json_schemas/json/DataTransfer.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            data: Some("data".to_string()),
        };

        let schema = include_str!("json_schemas/json/DataTransferResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: DiagnosticsStatus::Idle,
        };

        let schema = include_str!("json_schemas/json/DiagnosticsStatusNotification.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
        let test = DiagnosticsStatusNotificationResponse {};

        let schema = include_str!("json_schemas/json/DiagnosticsStatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: FirmwareStatus::Downloaded,
        };

        let schema = include_str!("json_schemas/json/FirmwareStatusNotification.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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

        let schema = include_str!("json_schemas/json/FirmwareStatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            charging_rate_unit: Some(ChargingRateUnitType::W),
        };

        let schema = include_str!("json_schemas/json/GetCompositeSchedule.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            connector_id: Some(1),
            schedule_start: Some(Utc::now()),
            charging_schedule: Some(ChargingSchedule {
                duration: Some(1),
                start_schedule: Some(Utc::now()),
                charging_rate_unit: ChargingRateUnitType::W,
                charging_schedule_period: vec![ChargingSchedulePeriod {
                    start_period: 0,
                    limit: 0.0,
                    number_phases: Some(1),
                }],
                min_charging_rate: Some(1.0),
            }),
        };

        let schema = include_str!("json_schemas/json/GetCompositeScheduleResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            key: Some(vec!["".to_string()]),
        };

        let schema = include_str!("json_schemas/json/GetConfiguration.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            configuration_key: Some(vec![KeyValue {
                key: "".to_string(),
                readonly: false,
                value: Some("".to_string()),
            }]),
            unknown_key: Some(vec!["".to_string()]),
        };

        let schema = include_str!("json_schemas/json/GetConfigurationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            retries: Some(1),
            retry_interval: Some(1),
            start_time: Some(Utc::now()),
            stop_time: Some(Utc::now()),
        };

        let schema = include_str!("json_schemas/json/GetDiagnostics.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
        let test = GetDiagnosticsResponse { file_name: None };

        let schema = include_str!("json_schemas/json/GetDiagnosticsResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
        let test = GetLocalListVersionRequest {};

        let schema = include_str!("json_schemas/json/GetLocalListVersion.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
        let test = GetLocalListVersionResponse { list_version: 0 };

        let schema = include_str!("json_schemas/json/GetLocalListVersionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_heartbeat() {
        let test = HeartbeatRequest {};

        let schema = include_str!("json_schemas/json/Heartbeat.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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

        let schema = include_str!("json_schemas/json/HeartbeatResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_meter_values() {
        let test = MeterValuesRequest {
            connector_id: 0,
            transaction_id: None,
            meter_value: vec![MeterValue {
                timestamp: Utc::now(),
                sampled_value: vec![SampledValue {
                    value: "".to_string(),
                    context: None,
                    format: None,
                    measurand: None,
                    phase: None,
                    location: None,
                    unit: None,
                }],
            }],
        };

        let schema = include_str!("json_schemas/json/MeterValues.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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

        let schema = include_str!("json_schemas/json/MeterValuesResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_remote_start_transaction() {
        let test = RemoteStartTransactionRequest {
            connector_id: None,
            id_tag: "".to_string(),
            charging_profile: None,
        };

        let schema = include_str!("json_schemas/json/RemoteStartTransaction.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_remote_start_transaction_response() {
        let test = RemoteStartTransactionResponse {
            status: RemoteStartStopStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/RemoteStartTransactionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_remote_stop_transaction() {
        let test = RemoteStopTransactionRequest { transaction_id: 0 };

        let schema = include_str!("json_schemas/json/RemoteStopTransaction.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_remote_stop_transaction_response() {
        let test = RemoteStopTransactionResponse {
            status: RemoteStartStopStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/RemoteStopTransactionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reserve_now() {
        let test = ReserveNowRequest {
            connector_id: 0,
            expiry_date: Utc::now(),
            id_tag: "".to_string(),
            parent_id_tag: None,
            reservation_id: 0,
        };

        let schema = include_str!("json_schemas/json/ReserveNow.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: ReservationStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/ReserveNowResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reset() {
        let test = ResetRequest {
            kind: ResetRequestStatus::Hard,
        };

        let schema = include_str!("json_schemas/json/Reset.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: ResetResponseStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/ResetResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_send_local_list() {
        let test = SendLocalListRequest {
            list_version: 0,
            local_authorization_list: None,
            update_type: UpdateType::Differential,
        };

        let schema = include_str!("json_schemas/json/SendLocalList.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: UpdateStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/SendLocalListResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_charging_profile() {
        let test = SetChargingProfileRequest {
            connector_id: 0,
            cs_charging_profiles: ChargingProfile {
                charging_profile_id: 0,
                transaction_id: None,
                stack_level: 0,
                charging_profile_purpose: Default::default(),
                charging_profile_kind: ChargingProfileKindType::Absolute,
                recurrency_kind: None,
                valid_from: None,
                valid_to: None,
                charging_schedule: ChargingSchedule {
                    duration: None,
                    start_schedule: None,
                    charging_rate_unit: ChargingRateUnitType::W,
                    charging_schedule_period: vec![ChargingSchedulePeriod {
                        start_period: 0,
                        limit: 0.0,
                        number_phases: None,
                    }],
                    min_charging_rate: None,
                },
            },
        };

        let schema = include_str!("json_schemas/json/SetChargingProfile.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
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
            status: ChargingProfileStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/SetChargingProfileResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_start_transaction() {
        let test = StartTransactionRequest {
            connector_id: 0,
            id_tag: "".to_string(),
            meter_start: 0,
            reservation_id: None,
            timestamp: Utc::now(),
        };

        let schema = include_str!("json_schemas/json/StartTransaction.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_start_transaction_response() {
        let test = StartTransactionResponse {
            id_tag_info: IdTagInfo {
                expiry_date: None,
                parent_id_tag: None,
                status: AuthorizationStatus::Accepted,
            },
            transaction_id: 0,
        };

        let schema = include_str!("json_schemas/json/StartTransactionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_status_notification() {
        let test = StatusNotificationRequest {
            connector_id: 0,
            error_code: ChargePointErrorCode::ConnectorLockFailure,
            info: None,
            status: ChargePointStatus::Available,
            timestamp: Some(Utc::now()),
            vendor_id: None,
            vendor_error_code: None,
        };

        let schema = include_str!("json_schemas/json/StatusNotification.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_status_notification_response() {
        let test = StatusNotificationResponse {};

        let schema = include_str!("json_schemas/json/StatusNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_stop_transaction() {
        let test = StopTransactionRequest {
            id_tag: None,
            meter_stop: 0,
            timestamp: Utc::now(),
            transaction_id: 0,
            reason: None,
            transaction_data: None,
        };

        let schema = include_str!("json_schemas/json/StopTransaction.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_stop_transaction_response() {
        let test = StopTransactionResponse {
            id_tag_info: Some(IdTagInfo {
                expiry_date: None,
                parent_id_tag: None,
                status: AuthorizationStatus::Accepted,
            }),
        };

        let schema = include_str!("json_schemas/json/StopTransactionResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_trigger_message() {
        let test = TriggerMessageRequest {
            requested_message: MessageTrigger::BootNotification,
            connector_id: None,
        };

        let schema = include_str!("json_schemas/json/TriggerMessage.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_trigger_message_response() {
        let test = TriggerMessageResponse {
            status: TriggerMessageStatus::Accepted,
        };

        let schema = include_str!("json_schemas/json/TriggerMessageResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_unlock_connector() {
        let test = UnlockConnectorRequest { connector_id: 0 };

        let schema = include_str!("json_schemas/json/UnlockConnector.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_unlock_connector_response() {
        let test = UnlockConnectorResponse {
            status: UnlockStatus::NotSupported,
        };

        let schema = include_str!("json_schemas/json/UnlockConnectorResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_update_firmware() {
        let test = UpdateFirmwareRequest {
            location: "https://codelabs.se".to_string(),
            retries: None,
            retrieve_date: Utc::now(),
            retry_interval: None,
        };

        let schema = include_str!("json_schemas/json/UpdateFirmware.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
        assert!(compiled.is_valid(&instance))
    }
    #[test]
    fn validate_update_firmware_response() {
        let test = UpdateFirmwareResponse {};

        let schema = include_str!("json_schemas/json/UpdateFirmwareResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema).expect("A valid schema");
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
