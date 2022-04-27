#[cfg(test)]
mod tests {
    use crate::v2_0_1::datatypes::charging_station_type::ChargingStationType;
    use crate::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;
    use crate::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType;
    use crate::v2_0_1::messages::boot_notification::{
        BootNotificationRequest, BootNotificationResponse,
    };
    use chrono::Utc;
    use jsonschema::is_valid;

    #[test]
    fn schema_validation_test() {
        let boot_notification_request = BootNotificationRequest {
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
        let instance = serde_json::to_value(&boot_notification_request).unwrap();
        assert!(is_valid(&schema, &instance));

        let boot_notification_response = BootNotificationResponse {
            current_time: Utc::now(),
            interval: 10,
            status: RegistrationStatusEnumType::Accepted,
            status_info: None,
        };

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/BootNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&boot_notification_response).unwrap();
        assert!(is_valid(&schema, &instance));
    }
}
