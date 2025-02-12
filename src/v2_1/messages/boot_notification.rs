use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Value};
use std::{fs, path::Path};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub charge_point_model: String,
    pub charge_point_vendor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_point_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_box_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_serial_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub current_time: String,
    pub interval: i32,
    pub status: BootNotificationStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]

#[cfg(test)]
mod tests {
    use super::*;

    fn read_json_file<P: AsRef<Path>>(path: P) -> String {
        fs::read_to_string(path).expect("Failed to read JSON file")
    }

    #[test]
    fn test_boot_notification_request_from_json() {
        let json_str =
            read_json_file("src/tests/schema_validation/schemas/v2.0.1/BootNotificationRequest.json");

        // Deserialize JSON into struct
        let req: BootNotificationRequest = from_str(&json_str)
            .expect("Failed to deserialize BootNotificationRequest JSON");
        
        // Serialize back to JSON Value for round-trip comparison (ordering-insensitive)
        let serialized = to_string(&req).expect("Failed to serialize BootNotificationRequest");
        let original_value: Value = from_str(&json_str).expect("Invalid original JSON");
        let new_value: Value = from_str(&serialized).expect("Invalid serialized JSON");
        assert_eq!(original_value, new_value);
    }

    #[test]
    fn test_boot_notification_request_to_json() {
        let req = BootNotificationRequest {
            charge_point_model: "ModelX".into(),
            charge_point_vendor: "VendorY".into(),
            charge_point_serial_number: Some("123456".into()),
            charge_box_serial_number: None,
            firmware_version: Some("v1.2.3".into()),
            iccid: None,
            imsi: None,
            meter_type: None,
            meter_serial_number: None,
        };

        // Serialize struct to JSON Value
        let serialized = to_string(&req).expect("Failed to serialize BootNotificationRequest");
        let serialized_value: Value = from_str(&serialized).expect("Invalid serialized JSON");

        let file_json =
            read_json_file("src/tests/schema_validation/schemas/v2.0.1/BootNotificationRequest.json");
        let file_value: Value =
            from_str(&file_json).expect("Invalid JSON from BootNotificationRequest file");

        // Compare the two JSON representations
        assert_eq!(serialized_value, file_value);
    }

    #[test]
    fn test_boot_notification_response_from_json() {
        let json_str =
            read_json_file("src/tests/schema_validation/schemas/v2.0.1/BootNotificationResponse.json");

        // Deserialize JSON into struct
        let resp: BootNotificationResponse = from_str(&json_str)
            .expect("Failed to deserialize BootNotificationResponse JSON");
        
        // Serialize back to JSON Value
        let serialized = to_string(&resp).expect("Failed to serialize BootNotificationResponse");
        let original_value: Value = from_str(&json_str).expect("Invalid original JSON");
        let new_value: Value = from_str(&serialized).expect("Invalid serialized JSON");
        assert_eq!(original_value, new_value);
    }

    #[test]
    fn test_boot_notification_response_to_json() {
        let resp = BootNotificationResponse {
            current_time: "2023-10-10T10:00:00Z".into(),
            interval: 300,
            status: BootNotificationStatus::Accepted,
        };

        // Serialize struct to JSON Value
        let serialized = to_string(&resp).expect("Failed to serialize BootNotificationResponse");
        let serialized_value: Value = from_str(&serialized).expect("Invalid serialized JSON");

        let file_json =
            read_json_file("src/tests/schema_validation/schemas/v2.0.1/BootNotificationResponse.json");
        let file_value: Value =
            from_str(&file_json).expect("Invalid JSON from BootNotificationResponse file");

        // Compare the two JSON representations
        assert_eq!(serialized_value, file_value);
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;

    #[test]
    fn test_boot_notification_request_full_options() {
        let req = BootNotificationRequest {
            charge_point_model: "ModelZ".into(),
            charge_point_vendor: "VendorZ".into(),
            charge_point_serial_number: Some("987654321".into()),
            charge_box_serial_number: Some("Box123".into()),
            firmware_version: Some("v9.9.9".into()),
            iccid: Some("iccid12345".into()),
            imsi: Some("imsi54321".into()),
            meter_type: Some("electric".into()),
            meter_serial_number: Some("meter987".into()),
        };
        let serialized = to_string(&req).expect("Failed to serialize BootNotificationRequest");
        let deserialized: BootNotificationRequest = from_str(&serialized)
            .expect("Failed to deserialize BootNotificationRequest");
        assert_eq!(req, deserialized);
    }
}