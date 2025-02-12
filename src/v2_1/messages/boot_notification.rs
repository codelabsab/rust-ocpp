use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::v2_1::enumerations::{BootReasonEnumType, RegistrationStatusEnumType};

lazy_static! {
    // This regex pattern validates ISO 8601 datetime strings in the format:
    // YYYY-MM-DDThh:mm:ss[.fraction]Z or
    // YYYY-MM-DDThh:mm:ss[.fraction]±hh:mm
    // Where:
    // - YYYY is a 4 digit year
    // - MM is a 2 digit month
    // - DD is a 2 digit day
    // - T is the literal 'T' separator
    // - hh:mm:ss is hours, minutes, seconds
    // - [.fraction] is an optional decimal fraction of a second
    // - Z represents UTC or ±hh:mm represents timezone offset
    static ref DATETIME_REGEX: Regex =
        Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})$")
            .unwrap();
}

/// Validates that a datetime string matches the ISO 8601 format.
///
/// The expected format is:
/// - YYYY-MM-DDThh:mm:ss[.fraction]Z or
/// - YYYY-MM-DDThh:mm:ss[.fraction]±hh:mm
///
/// # Arguments
/// * `datetime` - The datetime string to validate
///
/// # Returns
/// * `Ok(())` if the datetime string is valid
/// * `Err(ValidationError)` if the datetime string is invalid
fn validate_datetime(datetime: &str) -> Result<(), ValidationError> {
    if DATETIME_REGEX.is_match(datetime) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_datetime"))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomData {
    #[validate(length(max = 255))]
    pub vendor_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    #[validate(length(max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[validate(length(max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStation {
    #[validate(length(max = 20))]
    pub model: String,
    #[validate(length(max = 50))]
    pub vendor_name: String,
    #[validate(length(max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[validate(length(max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub reason: BootReasonEnumType,
    #[validate(nested)]
    pub charging_station: ChargingStation,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoType {
    #[validate(length(max = 20))]
    pub reason_code: String,
    #[validate(length(max = 1024))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    #[validate(custom(function = "validate_datetime"))]
    pub current_time: String,
    pub interval: i32,
    pub status: RegistrationStatusEnumType,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_boot_notification_request() {
        let request = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStation {
                model: "ModelX".into(),
                vendor_name: "VendorY".into(),
                serial_number: Some("123456".into()),
                firmware_version: Some("v1.0.0".into()),
                modem: Some(ModemType {
                    iccid: Some("89123456789".into()),
                    imsi: Some("123456789".into()),
                    custom_data: None,
                }),
                custom_data: None,
            },
            custom_data: None,
        };

        // Validate the request
        assert!(request.validate().is_ok());

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(
            json,
            json!({
                "reason": "PowerUp",
                "chargingStation": {
                    "model": "ModelX",
                    "vendorName": "VendorY",
                    "serialNumber": "123456",
                    "firmwareVersion": "v1.0.0",
                    "modem": {
                        "iccid": "89123456789",
                        "imsi": "123456789"
                    }
                }
            })
        );
    }

    #[test]
    fn test_invalid_boot_notification_request() {
        let request = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStation {
                model: "This model name is way too long and should fail validation".into(),
                vendor_name: "VendorY".into(),
                serial_number: Some("This serial number is also too long to be valid".into()),
                firmware_version: None,
                modem: None,
                custom_data: None,
            },
            custom_data: None,
        };

        // Validate the request - should fail
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_valid_boot_notification_response() {
        let response = BootNotificationResponse {
            current_time: "2023-10-10T10:10:10Z".into(),
            interval: 300,
            status: RegistrationStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                reason_code: "OK".into(),
                additional_info: Some("All good".into()),
                custom_data: None,
            }),
            custom_data: None,
        };

        // Validate the response
        assert!(response.validate().is_ok());

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(
            json,
            json!({
                "currentTime": "2023-10-10T10:10:10Z",
                "interval": 300,
                "status": "Accepted",
                "statusInfo": {
                    "reasonCode": "OK",
                    "additionalInfo": "All good"
                }
            })
        );
    }

    #[test]
    fn test_invalid_boot_notification_response() {
        let response = BootNotificationResponse {
            current_time: "invalid-datetime".into(),
            interval: 300,
            status: RegistrationStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                reason_code: "This reason code is way too long to be valid".into(),
                additional_info: None,
                custom_data: None,
            }),
            custom_data: None,
        };

        // Validate the response - should fail
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_boot_notification_request_with_custom_data() {
        let request = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStation {
                model: "ModelZ".into(),
                vendor_name: "VendorZ".into(),
                serial_number: Some("987654321".into()),
                firmware_version: Some("v9.9.9".into()),
                modem: Some(ModemType {
                    iccid: Some("iccid12345".into()),
                    imsi: Some("imsi54321".into()),
                    custom_data: Some(CustomData {
                        vendor_id: "VendorZ".into(),
                    }),
                }),
                custom_data: Some(CustomData {
                    vendor_id: "VendorZ".into(),
                }),
            },
            custom_data: Some(CustomData {
                vendor_id: "VendorZ".into(),
            }),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BootNotificationRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }
}
