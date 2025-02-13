use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Signed meter value data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Base64 encoded, contains the signed data that needs to be verified.
    #[validate(length(max = 2500))]
    pub signed_meter_data: String,

    /// Required. Base64 encoded, contains the signing method information used to create the signature.
    #[validate(length(max = 2500))]
    pub signing_method: String,

    /// Required. Base64 encoded, contains the encoded X.509 certificate used to create the signature.
    #[validate(length(max = 5500))]
    pub encoding_method: String,

    /// Required. Base64 encoded, contains the actual signature.
    #[validate(length(max = 800))]
    pub public_key: String,
}
