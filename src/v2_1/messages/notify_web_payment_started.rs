use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::custom_data::CustomDataType;

/// Request to notify the CSMS that a web payment has been started.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyWebPaymentStartedRequest {
    /// Required. EVSE id for which transaction is requested.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Required. Timeout value in seconds after which no result of web payment process (e.g. QR code scanning) is to be expected anymore.
    pub timeout: i32,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyWebPaymentStartedRequest. This message has no fields except for optional custom data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyWebPaymentStartedResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl NotifyWebPaymentStartedRequest {
    /// Creates a new `NotifyWebPaymentStartedRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - EVSE id for which transaction is requested
    /// * `timeout` - Timeout value in seconds after which no result of web payment process is to be expected
    ///
    /// # Returns
    ///
    /// A new instance of `NotifyWebPaymentStartedRequest` with optional fields set to `None`
    pub fn new(evse_id: i32, timeout: i32) -> Self {
        Self {
            evse_id,
            timeout,
            custom_data: None,
        }
    }
}

impl NotifyWebPaymentStartedResponse {
    /// Creates a new `NotifyWebPaymentStartedResponse`.
    ///
    /// # Returns
    ///
    /// A new instance of `NotifyWebPaymentStartedResponse` with optional fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
    }
}
