/// tests
#[cfg(test)]

mod tests {
    use crate::v2_0_1::core::enumerations::boot_reason_enum_type::BootReasonEnumType;
    use crate::v2_0_1::core::messages::boot_notification::{
        BootNotificationRequest, BootNotificationResponse,
    };
    use crate::v2_0_1::core::{
        datatypes::{
            charging_station_type::ChargingStationType, modem_type::ModemType,
            status_info_type::StatusInfoType,
        },
        enumerations::registration_status_enum_type::RegistrationStatusEnumType,
    };
    use chrono::prelude::*;
    use serde_json;

    #[test]
    fn create_boot_request_notification_two() {
        let req = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                serial_number: Some("101".to_string()),
                model: "exa1000".to_string(),
                vendor_name: "exawatt".to_string(),
                firmware_version: None,
                modem: Some(ModemType {
                    iccid: Some("10101".to_string()),
                    imsi: Some("10101".to_string()),
                }),
            },
        };
        let json = serde_json::to_string(&req).unwrap();
        println!("{}", json);
        // Test serial number field
        assert_eq!(
            req.charging_station.serial_number.unwrap(),
            "101".to_string(),
            "Does the serial number field work?"
        );
    }

    #[test]
    fn create_boot_notification_request() {
        let req = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                serial_number: Some("101".to_string()),
                model: "exa1000".to_string(),
                vendor_name: "exawatt".to_string(),
                firmware_version: Some("v1.0".to_string()),
                modem: Some(ModemType {
                    iccid: Some("10101".to_string()),
                    imsi: Some("10101".to_string()),
                }),
            },
        };
        // Test serial number field
        assert_eq!(
            req.charging_station.serial_number.unwrap(),
            "101".to_string(),
            "Does the serial number field work?"
        );
    }

    #[test]
    fn create_boot_notification_request_from_json() {
        let json = r#"
        {
          "reason": "PowerUp",
          "chargingStation": {
            "serialNumber": "1010",
            "model": "T-1000",
            "vendorName": "Skynet",
            "firmwareVersion": "v0.1",
            "modem": {
              "iccid": "iccid",
              "imsi": "imsi"
            }
          }
        }"#;

        let req: BootNotificationRequest = serde_json::from_str(json).unwrap();

        assert_eq!(
            req.charging_station.serial_number.unwrap(),
            "1010".to_string()
        );
        assert_eq!(req.charging_station.model, "T-1000".to_string());
        assert_eq!(req.charging_station.vendor_name, "Skynet".to_string());
        assert_eq!(
            req.charging_station.firmware_version.unwrap(),
            "v0.1".to_string()
        );
        let modem = req.charging_station.modem.unwrap();
        let imsi = modem.imsi.unwrap();
        let iccid = modem.iccid.unwrap();
        assert_eq!(imsi, "imsi".to_string());
        assert_eq!(iccid, "iccid".to_string());

        assert!(matches!(req.reason, BootReasonEnumType::PowerUp));
    }

    #[test]
    fn create_boot_notification_response() {
        let b = BootNotificationResponse {
            current_time: Utc::now(),
            interval: 1,
            status: RegistrationStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                reason_code: "Accepted".to_string(),
                additional_info: None,
            }),
        };
        assert_eq!(b.interval, 1);
        assert!(matches!(b.status, RegistrationStatusEnumType::Accepted));

        // status_info_type
        let status_info = b.status_info.unwrap();
        assert_eq!(status_info.reason_code, "Accepted".to_string());
        assert!(matches!(status_info.additional_info, None));
    }
}
