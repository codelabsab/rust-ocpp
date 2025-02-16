use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{datatypes::CustomDataType, enumerations::signing_method::SigningMethodEnumType};

/// Contains a signed version of the meter value.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Base64 encoded, contains the signed data that needs to be verified.
    #[validate(length(max = 32768))]
    pub signed_meter_data: String,

    /// Required. Method used to create the digital signature.
    pub signing_method: SigningMethodEnumType,

    /// Required. Base64 encoded, contains the public key to verify the signature.
    #[validate(length(max = 50))]
    pub encoding_method: String,

    /// Required. Base64 encoded SHA256 hash of the public key that is used for the encoding method.
    #[validate(length(max = 2500))]
    pub public_key: String,
}
