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

impl SignedMeterValueType {
    /// Creates a new `SignedMeterValueType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `signed_meter_data` - Base64 encoded, contains the signed data that needs to be verified
    /// * `signing_method` - Method used to create the digital signature
    /// * `encoding_method` - Base64 encoded, contains the public key to verify the signature
    /// * `public_key` - Base64 encoded SHA256 hash of the public key that is used for the encoding method
    ///
    /// # Returns
    ///
    /// A new instance of `SignedMeterValueType` with optional fields set to `None`
    pub fn new(
        signed_meter_data: String,
        signing_method: SigningMethodEnumType,
        encoding_method: String,
        public_key: String,
    ) -> Self {
        Self {
            custom_data: None,
            signed_meter_data,
            signing_method,
            encoding_method,
            public_key,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this signed meter value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the signed meter data.
    ///
    /// # Returns
    ///
    /// The Base64 encoded signed data that needs to be verified
    pub fn signed_meter_data(&self) -> &str {
        &self.signed_meter_data
    }

    /// Sets the signed meter data.
    ///
    /// # Arguments
    ///
    /// * `signed_meter_data` - Base64 encoded, contains the signed data that needs to be verified
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signed_meter_data(&mut self, signed_meter_data: String) -> &mut Self {
        self.signed_meter_data = signed_meter_data;
        self
    }

    /// Gets the signing method.
    ///
    /// # Returns
    ///
    /// The method used to create the digital signature
    pub fn signing_method(&self) -> &SigningMethodEnumType {
        &self.signing_method
    }

    /// Sets the signing method.
    ///
    /// # Arguments
    ///
    /// * `signing_method` - Method used to create the digital signature
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signing_method(&mut self, signing_method: SigningMethodEnumType) -> &mut Self {
        self.signing_method = signing_method;
        self
    }

    /// Gets the encoding method.
    ///
    /// # Returns
    ///
    /// The Base64 encoded public key to verify the signature
    pub fn encoding_method(&self) -> &str {
        &self.encoding_method
    }

    /// Sets the encoding method.
    ///
    /// # Arguments
    ///
    /// * `encoding_method` - Base64 encoded, contains the public key to verify the signature
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_encoding_method(&mut self, encoding_method: String) -> &mut Self {
        self.encoding_method = encoding_method;
        self
    }

    /// Gets the public key.
    ///
    /// # Returns
    ///
    /// The Base64 encoded SHA256 hash of the public key
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    /// Sets the public key.
    ///
    /// # Arguments
    ///
    /// * `public_key` - Base64 encoded SHA256 hash of the public key that is used for the encoding method
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_public_key(&mut self, public_key: String) -> &mut Self {
        self.public_key = public_key;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this signed meter value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_signed_meter_value() {
        let signed_meter_data = "signed_data".to_string();
        let signing_method = SigningMethodEnumType::Custom("ECDSA-256".to_string());
        let encoding_method = "encoding_method".to_string();
        let public_key = "public_key".to_string();

        let value = SignedMeterValueType::new(
            signed_meter_data.clone(),
            signing_method.clone(),
            encoding_method.clone(),
            public_key.clone(),
        );

        assert_eq!(value.signed_meter_data(), signed_meter_data);
        assert_eq!(value.signing_method(), &signing_method);
        assert_eq!(value.encoding_method(), encoding_method);
        assert_eq!(value.public_key(), public_key);
        assert_eq!(value.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let signed_meter_data = "signed_data".to_string();
        let signing_method = SigningMethodEnumType::Custom("ECDSA-256".to_string());
        let encoding_method = "encoding_method".to_string();
        let public_key = "public_key".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let value = SignedMeterValueType::new(
            signed_meter_data.clone(),
            signing_method.clone(),
            encoding_method.clone(),
            public_key.clone(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(value.signed_meter_data(), signed_meter_data);
        assert_eq!(value.signing_method(), &signing_method);
        assert_eq!(value.encoding_method(), encoding_method);
        assert_eq!(value.public_key(), public_key);
        assert_eq!(value.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let signed_meter_data1 = "signed_data1".to_string();
        let signing_method1 = SigningMethodEnumType::Custom("ECDSA-256".to_string());
        let encoding_method1 = "encoding_method1".to_string();
        let public_key1 = "public_key1".to_string();

        let mut value = SignedMeterValueType::new(
            signed_meter_data1,
            signing_method1,
            encoding_method1,
            public_key1,
        );

        let signed_meter_data2 = "signed_data2".to_string();
        let signing_method2 = SigningMethodEnumType::Custom("ECDSA-512".to_string());
        let encoding_method2 = "encoding_method2".to_string();
        let public_key2 = "public_key2".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        value
            .set_signed_meter_data(signed_meter_data2.clone())
            .set_signing_method(signing_method2.clone())
            .set_encoding_method(encoding_method2.clone())
            .set_public_key(public_key2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(value.signed_meter_data(), signed_meter_data2);
        assert_eq!(value.signing_method(), &signing_method2);
        assert_eq!(value.encoding_method(), encoding_method2);
        assert_eq!(value.public_key(), public_key2);
        assert_eq!(value.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        value.set_custom_data(None);

        assert_eq!(value.custom_data(), None);
    }
}
