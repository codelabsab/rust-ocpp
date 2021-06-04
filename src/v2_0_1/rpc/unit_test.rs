/// tests
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::v2_0_1::{
        core::{
            enumerations::boot_reason_enum_type::BootReasonEnumType,
            messages::boot_notification::BootNotificationRequest,
        },
        rpc::call::{Call, CallActionTypeEnum, CallError, CallResult},
    };
    use serde_json::{self, json};

    #[test]
    fn test_call_to_boot_nofitication_request() {
        let call = Call {
            message_type_id: 2,
            message_id: "19223201".to_string(),
            action: CallActionTypeEnum::BootNotification,
            payload: r#"
            {
                "reason": "PowerUp",
                "chargingStation": {
                    "serialNumber": "101",
                    "model": "CS100",
                    "vendorName": "CLBox",
                    "firmwareVersion": "v0.1",
                    "modem": {
                        "iccid": "iccid",
                        "imsi": "imsi"
                    }
                }
            }"#
            .to_string(),
        };

        assert_eq!(call.message_type_id, 2);
        assert_eq!(call.message_id, "19223201".to_string());
        assert_eq!(call.action, CallActionTypeEnum::BootNotification);

        // Create a BootNotificationRequest from the payload
        let bnr: BootNotificationRequest = serde_json::from_str(&call.payload).unwrap();
        assert_eq!(bnr.reason, BootReasonEnumType::PowerUp);
    }

    #[test]
    fn test_call_result() {
        let result = CallResult {
            message_type_id: 3,
            message_id: "19223201".to_string(),
            payload: json!({"currentTime": "2013-02-01T20:53:32.486Z","interval": 300,"status":"Accepted"}),
        };

        assert_eq!(result.message_type_id, 3);
        assert_eq!(result.message_id, "19223201".to_string());
        assert_eq!(
            result.payload,
            json!({"currentTime": "2013-02-01T20:53:32.486Z", "interval": 300, "status":"Accepted"})
        );
    }

    #[test]
    fn test_call_error() {
        let error = CallError {
            message_type_id: 4,
            message_id: "19223201".to_string(),
            error_code: "NotSupported".to_string(),
            error_description: "SetDisplayMessageRequest not implemented".to_string(),
            error_details: "{}".to_string(),
        };

        assert_eq!(error.message_type_id, 4, "Testing message_type_id");
        assert_eq!(
            error.message_id,
            "19223201".to_string(),
            "Testing message_id"
        );
        assert_eq!(
            error.error_code,
            "NotSupported".to_string(),
            "Testing error_code"
        );
        assert_eq!(
            error.error_details,
            "{}".to_string(),
            "Testing error_details"
        );
    }

    #[test]
    fn test_call_to_value() {
        let json = r#"
            [
                2,
                "19223201",
                "BootNotification",
                {
                    "reason": "PowerUp",
                    "chargingStation": {
                        "model": "SingleSocketCharger",
                        "vendorName": "VendorX"
                    }
                }
            ]
            "#;

        let val = serde_json::Value::from_str(json).unwrap();

        let call: Call = Call {
            message_type_id: (*val.get(0).unwrap()).as_i64().unwrap(),
            message_id: (*val.get(1).unwrap()).to_string(),
            action: CallActionTypeEnum::from_str(val.get(2).unwrap().as_str().unwrap()).unwrap(),
            payload: (val.get(3).unwrap().to_string()),
        };

        let bnr: BootNotificationRequest = serde_json::from_str(&call.payload).unwrap();
        assert_eq!(bnr.reason, BootReasonEnumType::PowerUp);
    }

    #[test]
    fn test_call_action_enum() {
        let call_action = CallActionTypeEnum::BootNotification;
        assert_eq!(call_action.to_string(), "BootNotification".to_string());
        let auth = "Authorize".to_string();
        let auth_action_from_str = CallActionTypeEnum::from_str(&auth);
        assert_eq!(CallActionTypeEnum::Authorize, auth_action_from_str.unwrap());
        let boot_notification_action =
            CallActionTypeEnum::from_str("BootNotification".to_string().as_str());
        assert_eq!(boot_notification_action.is_ok(), true);
    }
}
