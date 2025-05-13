use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{additional_info::AdditionalInfoType, custom_data::CustomDataType};

/// Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    /// Optional. Additional information about the identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub additional_info: Option<Vec<AdditionalInfoType>>,

    /// Required. IdToken is case insensitive. Might hold the hidden id of an RFID tag, but can for example also contain a UUID.
    #[validate(length(max = 255))]
    pub id_token: String,

    /// Required. Type of identification used to authorize charging.
    /// Allowed values: "Central", "DirectPayment", "eMAID", "EVCCID", "ISO14443", "ISO15693",
    /// "KeyCode", "Local", "MacAddress", "NoAuthorization", "VIN"
    #[serde(rename = "type")]
    #[validate(length(max = 20))]
    pub r#type: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl IdTokenType {
    /// Creates a new `IdTokenType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id_token` - ID token string, case insensitive
    /// * `type` - Type of identification used to authorize charging
    ///
    /// # Returns
    ///
    /// A new instance of `IdTokenType` with optional fields set to `None`
    pub fn new(id_token: String, r#type: String) -> Self {
        Self {
            id_token,
            r#type,
            additional_info: None,
            custom_data: None,
        }
    }

    /// Sets the additional information.
    ///
    /// # Arguments
    ///
    /// * `additional_info` - Vector of additional information about the identifier
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_additional_info(mut self, additional_info: Vec<AdditionalInfoType>) -> Self {
        self.additional_info = Some(additional_info);
        self
    }

    /// Adds a single additional information item to the existing list.
    /// If no list exists yet, a new one is created.
    ///
    /// # Arguments
    ///
    /// * `info` - Additional information item to add
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn add_additional_info(mut self, info: AdditionalInfoType) -> Self {
        match self.additional_info {
            Some(ref mut list) => {
                list.push(info);
            }
            None => {
                self.additional_info = Some(vec![info]);
            }
        }
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this ID token
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ID token.
    ///
    /// # Returns
    ///
    /// The ID token string
    pub fn id_token(&self) -> &str {
        &self.id_token
    }

    /// Sets the ID token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - ID token string, case insensitive
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id_token(&mut self, id_token: String) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Gets the token type.
    ///
    /// # Returns
    ///
    /// The token type string
    pub fn type_field(&self) -> &str {
        &self.r#type
    }

    /// Sets the token type.
    ///
    /// # Arguments
    ///
    /// * `type` - Type of identification used to authorize charging
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, r#type: String) -> &mut Self {
        self.r#type = r#type;
        self
    }

    /// Gets the additional information.
    ///
    /// # Returns
    ///
    /// An optional reference to the vector of additional information
    pub fn additional_info(&self) -> Option<&Vec<AdditionalInfoType>> {
        self.additional_info.as_ref()
    }

    /// Sets the additional information.
    ///
    /// # Arguments
    ///
    /// * `additional_info` - Vector of additional information about the identifier, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_additional_info(
        &mut self,
        additional_info: Option<Vec<AdditionalInfoType>>,
    ) -> &mut Self {
        self.additional_info = additional_info;
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
    /// * `custom_data` - Custom data for this ID token, or None to clear
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
    use validator::Validate;

    #[test]
    fn test_new_id_token() {
        let id = "4F62C4E0123456789".to_string();
        let token_type = "ISO14443".to_string();

        let token = IdTokenType::new(id.clone(), token_type.clone());

        assert_eq!(token.id_token(), &id);
        assert_eq!(token.type_field(), &token_type);
        assert_eq!(token.additional_info(), None);
        assert_eq!(token.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = "4F62C4E0123456789".to_string();
        let token_type = "ISO14443".to_string();

        let additional_info = vec![AdditionalInfoType {
            additional_id_token: "Card123".to_string(),
            type_: "CardType".to_string(),
            custom_data: None,
        }];

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let token = IdTokenType::new(id.clone(), token_type.clone())
            .with_additional_info(additional_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(token.id_token(), &id);
        assert_eq!(token.type_field(), &token_type);
        assert_eq!(token.additional_info(), Some(&additional_info));
        assert_eq!(token.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_add_additional_info() {
        let id = "4F62C4E0123456789".to_string();
        let token_type = "ISO14443".to_string();

        let info1 = AdditionalInfoType {
            additional_id_token: "Card123".to_string(),
            type_: "CardType".to_string(),
            custom_data: None,
        };

        let info2 = AdditionalInfoType {
            additional_id_token: "Card456".to_string(),
            type_: "CardType".to_string(),
            custom_data: None,
        };

        // Test adding to an empty list
        let token = IdTokenType::new(id.clone(), token_type.clone())
            .add_additional_info(info1.clone());

        assert_eq!(token.additional_info().unwrap().len(), 1);
        assert_eq!(token.additional_info().unwrap()[0], info1);

        // Test adding to an existing list
        let token = token.add_additional_info(info2.clone());

        assert_eq!(token.additional_info().unwrap().len(), 2);
        assert_eq!(token.additional_info().unwrap()[0], info1);
        assert_eq!(token.additional_info().unwrap()[1], info2);
    }

    #[test]
    fn test_setter_methods() {
        let id1 = "4F62C4E0123456789".to_string();
        let id2 = "ABCDEF0123456789".to_string();
        let token_type1 = "ISO14443".to_string();
        let token_type2 = "RFID".to_string();

        let additional_info = vec![AdditionalInfoType {
            additional_id_token: "Card123".to_string(),
            type_: "CardType".to_string(),
            custom_data: None,
        }];

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut token = IdTokenType::new(id1.clone(), token_type1.clone());

        token
            .set_id_token(id2.clone())
            .set_type(token_type2.clone())
            .set_additional_info(Some(additional_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(token.id_token(), &id2);
        assert_eq!(token.type_field(), &token_type2);
        assert_eq!(token.additional_info(), Some(&additional_info));
        assert_eq!(token.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        token.set_additional_info(None).set_custom_data(None);

        assert_eq!(token.additional_info(), None);
        assert_eq!(token.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid token
        let valid_token = IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string());
        assert!(valid_token.validate().is_ok(), "Valid token should pass validation");

        // Invalid token - id_token too long (>255 chars)
        let invalid_id_token = IdTokenType::new("A".repeat(256), "ISO14443".to_string());
        let result = invalid_id_token.validate();
        assert!(result.is_err(), "Token with too long id_token should fail validation");
        let error = result.unwrap_err();
        assert!(error.to_string().contains("id_token"),
                "Error should mention id_token: {}", error);

        // Invalid token - type too long (>20 chars)
        let invalid_type = IdTokenType::new("4F62C4E0123456789".to_string(), "A".repeat(21));
        let result = invalid_type.validate();
        assert!(result.is_err(), "Token with too long type should fail validation");
        let error = result.unwrap_err();
        assert!(error.to_string().contains("type"),
                "Error should mention type: {}", error);

        // Invalid token - empty additional_info vector
        let mut invalid_additional_info = IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string());
        invalid_additional_info.additional_info = Some(vec![]);
        let result = invalid_additional_info.validate();
        assert!(result.is_err(), "Token with empty additional_info vector should fail validation");
        let error = result.unwrap_err();
        assert!(error.to_string().contains("additional_info"),
                "Error should mention additional_info: {}", error);

        // Invalid token - invalid custom_data
        let mut invalid_custom_data = CustomDataType::new("VendorX".to_string());
        invalid_custom_data.vendor_id = "A".repeat(256); // Too long vendor_id

        let mut invalid_token = IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string());
        invalid_token.custom_data = Some(invalid_custom_data);

        let result = invalid_token.validate();
        assert!(result.is_err(), "Token with invalid custom_data should fail validation");
        let error = result.unwrap_err();
        assert!(error.to_string().contains("custom_data"),
                "Error should mention custom_data: {}", error);
    }
}
