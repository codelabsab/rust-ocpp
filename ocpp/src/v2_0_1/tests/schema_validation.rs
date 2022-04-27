#[cfg(test)]
mod tests {
    use crate::v2_0_1::datatypes::charging_station_type::ChargingStationType;
    use crate::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;
    use crate::v2_0_1::messages::boot_notification::BootNotificationRequest;
    use jsonschema::is_valid;

    #[test]
    fn schema_validation_test() {
        let bnr = BootNotificationRequest {
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
        let instance = serde_json::to_value(&bnr).unwrap();
        assert!(is_valid(&schema, &instance));
    }
}