use jsonschema::Validator;
use serde_json::Value;

const SCHEMA_DIR: &str = "src/tests/schema_validation/schemas/v2.1";

// Helper function to validate schema and instance with detailed error reporting
fn validate_schema_instance(
    schema_name: &str,
    instance: Value,
) -> Result<bool, Box<dyn std::error::Error>> {
    let schema_path = format!("{}/{}", SCHEMA_DIR, schema_name);
    let schema_str = std::fs::read_to_string(schema_path)?;
    let schema = serde_json::from_str(&schema_str)?;
    let compiled = Validator::new(&schema).expect("A valid schema");
    let result = compiled.validate(&instance);

    if result.is_err() {
        for error in compiled.iter_errors(&instance) {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    }

    Ok(compiled.is_valid(&instance))
}

#[test]
fn test_valid_boot_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        }
    });

    assert!(validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_boot_notification_missing_required_field() -> Result<(), Box<dyn std::error::Error>>
{
    let instance = serde_json::json!({
        "reason": "PowerUp",
        // Missing required chargingStation field
    });

    assert!(!validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_authorize_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234",
            "type": "ISO14443"
        }
    });

    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_invalid_authorize_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234",
            // Missing required 'type' field
        }
    });

    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_boot_notification_request_additional_field() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        },
        "additionalField": "this should NOT be allowed"  // OCPP 2.1 is strict about additional properties
    });

    // The validation should fail because OCPP 2.1 doesn't allow additional properties
    assert!(!validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_boot_notification_request_v2_1() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        }
    });

    assert!(validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_boot_notification_response_v2_1() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "currentTime": "2023-10-10T10:10:10Z",
        "interval": 300,
        "status": "Accepted"
    });

    assert!(validate_schema_instance(
        "BootNotificationResponse.json",
        instance
    )?);
    Ok(())
}

// We recommend installing an extension to run rust tests.
