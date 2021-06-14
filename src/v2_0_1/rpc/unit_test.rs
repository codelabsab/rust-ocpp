/// tests
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::v2_0_1::{
        core::{
            enumerations::boot_reason_enum_type::BootReasonEnumType,
            messages::boot_notification::BootNotificationRequest,
        },
        rpc::{
            call::{Call, CallActionTypeEnum, CallError, CallResult},
            errors::RpcErrorCodes,
        },
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

    // #[test]
    // fn test_call_result() {
    //     let result = CallResult {
    //         message_type_id: 3,
    //         message_id: "19223201".to_string(),
    //         payload: json!({"currentTime": "2013-02-01T20:53:32.486Z","interval": 300,"status":"Accepted"}),
    //     };

    //     assert_eq!(result.message_type_id, 3);
    //     assert_eq!(result.message_id, "19223201".to_string());
    //     assert_eq!(
    //         result.payload,
    //         json!({"currentTime": "2013-02-01T20:53:32.486Z", "interval": 300, "status":"Accepted"})
    //     );
    // }

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

    #[test]
    fn test_arr() {
        let json_str = "[2,\"19223201\",\"BootNotification\",{\"reason\": \"PowerUp\",\"chargingStation\": {\"model\": \"SingleSocketCharger\",\"vendorName\": \"VendorX\"}}]";
        let val = serde_json::Value::from_str(&json_str).unwrap();
        let action = val.get(2);
        match action {
            Some(v) => {
                println!("Got {} on val.get(2)", v);
                let action_str = v.as_str();
                match action_str {
                    Some(s) => {
                        println!("Got str {}", s);
                        let cat = CallActionTypeEnum::from_str(s);
                        match cat {
                            Ok(o) => {
                                println!("cat is ok");
                                assert_eq!(o, CallActionTypeEnum::BootNotification);
                            }
                            Err(_) => {
                                println!("cat failed");
                            }
                        };
                    }
                    None => {
                        println!("Did not get str in action_str");
                    }
                }
            }
            None => {
                println!("got none");
            }
        }
        println!("{:?}", val);
    }

    #[test]
    fn test_rpc_error_codes() {
        // format validation error
        let format_violation_err = RpcErrorCodes::FormatViolation;
        assert_eq!(
            format_violation_err.description(),
            "Payload for Action is syntactically incorrect"
        );

        // generic error
        let generic_err = RpcErrorCodes::GenericError;
        assert_eq!(
            generic_err.description(),
            "Any other error not covered by the more specific error codes in this table"
        );

        // internal error
        let internal_err = RpcErrorCodes::InternalError;
        assert_eq!(internal_err.description(), "An internal error occurred and the receiver was not able to process the requested Action successfully");

        // message type not supported
        let messagetypenotsupported_err = RpcErrorCodes::MessageTypeNotSupported;
        assert_eq!(messagetypenotsupported_err.description(), "A message with an Message Type Number received that is not supported by this implementation.");

        // not implemented
        let not_implemented_err = RpcErrorCodes::NotImplemented;
        assert_eq!(
            not_implemented_err.description(),
            "Requested Action is not known by receiver"
        );

        // not supported
        let not_supported = RpcErrorCodes::NotSupported;
        assert_eq!(
            not_supported.description(),
            "Requested Action is recognized but not supported by the receiver"
        );

        // cccurrence constraint violation
        let occurrence_constraint_violation_err = RpcErrorCodes::OccurrenceConstraintViolation;
        assert_eq!(occurrence_constraint_violation_err.description(), "Payload for Action is syntactically correct but at least one of the fields violates occurrence constraints");

        // property constraint violation
        let property_constraint_violation_err = RpcErrorCodes::PropertyConstraintViolation;
        assert_eq!(
            property_constraint_violation_err.description(),
            "Payload is syntactically correct but at least one field contains an invalid value"
        );

        // protocol error
        let protocol_error_err = RpcErrorCodes::ProtocolError;
        assert_eq!(
            protocol_error_err.description(),
            "Payload for Action is not conform the PDU structure"
        );

        // rpc framework error
        let rpc_framework_error = RpcErrorCodes::RpcFrameworkError;
        assert_eq!(rpc_framework_error.description(), "Content of the call is not a valid RPC Request, for example: MessageId could not be read.");

        // security error
        let security_error_err = RpcErrorCodes::SecurityError;
        assert_eq!(security_error_err.description(), "During the processing of Action a security issue occurred preventing receiver from completing the Action successfully");

        // type constraint violation
        let type_constraint_violation = RpcErrorCodes::TypeConstraintViolation;
        assert_eq!(type_constraint_violation.description(), "Payload for Action is syntactically correct but at least one of the fields violates data type constraints (e.g. \"somestring\": 12)");
    }
}
