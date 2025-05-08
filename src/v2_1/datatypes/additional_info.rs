use super::custom_data::CustomDataType;
use crate::v2_1::helpers::validator::validate_identifier_string;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Contains additional information about an identifier.
///
/// The format of the additionalIdToken is pending standardization.
/// This type is used to provide additional identification information for authorization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    /// This field specifies the type of the additionalIdToken.
    ///
    /// The format of the additionalIdToken is pending standardization.
    #[validate(
        length(max = 255),
        custom(function = "validate_identifier_string")
    )]
    pub additional_id_token: String,

    /// This defines the type of the additionalIdToken.
    ///
    /// This is a custom type, so the implementation needs to be agreed upon by all involved parties.
    #[serde(rename = "type")]
    #[validate(length(max = 50))]
    pub type_: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl AdditionalInfoType {
    /// Creates a new `AdditionalInfoType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `additional_id_token` - The additional ID token value
    /// * `type_` - The type of the additional ID token
    ///
    /// # Returns
    ///
    /// A new instance of `AdditionalInfoType` with optional fields set to `None`
    pub fn new(additional_id_token: String, type_: String) -> Self {
        Self {
            additional_id_token,
            type_,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this additional info
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the additional ID token.
    ///
    /// # Returns
    ///
    /// The additional ID token as a string
    pub fn additional_id_token(&self) -> &str {
        &self.additional_id_token
    }

    /// Sets the additional ID token.
    ///
    /// # Arguments
    ///
    /// * `additional_id_token` - The additional ID token value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_additional_id_token(&mut self, additional_id_token: String) -> &mut Self {
        self.additional_id_token = additional_id_token;
        self
    }

    /// Gets the type of the additional ID token.
    ///
    /// # Returns
    ///
    /// The type of the additional ID token as a string
    pub fn type_(&self) -> &str {
        &self.type_
    }

    /// Sets the type of the additional ID token.
    ///
    /// # Arguments
    ///
    /// * `type_` - The type of the additional ID token
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, type_: String) -> &mut Self {
        self.type_ = type_;
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
    /// * `custom_data` - Custom data for this additional info, or None to clear
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
    fn test_new_additional_info() {
        let info = AdditionalInfoType::new("token123".to_string(), "RFID".to_string());

        assert_eq!(info.additional_id_token(), "token123");
        assert_eq!(info.type_(), "RFID");
        assert_eq!(info.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let info = AdditionalInfoType::new("token123".to_string(), "RFID".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(info.additional_id_token(), "token123");
        assert_eq!(info.type_(), "RFID");
        assert_eq!(info.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut info = AdditionalInfoType::new("token123".to_string(), "RFID".to_string());

        info.set_additional_id_token("token456".to_string())
            .set_type("NFC".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(info.additional_id_token(), "token456");
        assert_eq!(info.type_(), "NFC");
        assert_eq!(info.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        info.set_custom_data(None);
        assert_eq!(info.custom_data(), None);
    }
}
