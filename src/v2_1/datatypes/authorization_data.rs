use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, id_token::IdTokenType, id_token_info::IdTokenInfoType};

/// Contains the identifier to use for authorization.
///
/// This type represents authorization data including the identifier token and its status information.
/// It is used in authorization-related messages to provide information about an identifier's authorization status.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The identifier to be authorized.
    pub id_token: IdTokenType,

    /// Required. Status information about the identifier.
    pub id_token_info: IdTokenInfoType,
}

impl AuthorizationData {
    /// Creates a new `AuthorizationData` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id_token` - The identifier to be authorized
    /// * `id_token_info` - Status information about the identifier
    ///
    /// # Returns
    ///
    /// A new instance of `AuthorizationData` with optional fields set to `None`
    pub fn new(id_token: IdTokenType, id_token_info: IdTokenInfoType) -> Self {
        Self {
            custom_data: None,
            id_token,
            id_token_info,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this authorization data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `custom_data` - Custom data for this authorization data, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the identifier token.
    ///
    /// # Returns
    ///
    /// A reference to the identifier token
    pub fn id_token(&self) -> &IdTokenType {
        &self.id_token
    }

    /// Sets the identifier token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - The identifier to be authorized
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id_token(&mut self, id_token: IdTokenType) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Gets the identifier token information.
    ///
    /// # Returns
    ///
    /// A reference to the identifier token information
    pub fn id_token_info(&self) -> &IdTokenInfoType {
        &self.id_token_info
    }

    /// Sets the identifier token information.
    ///
    /// # Arguments
    ///
    /// * `id_token_info` - Status information about the identifier
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id_token_info(&mut self, id_token_info: IdTokenInfoType) -> &mut Self {
        self.id_token_info = id_token_info;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::AuthorizationStatusEnumType;

    #[test]
    fn test_new_authorization_data() {
        let id_token = IdTokenType {
            id_token: "tag123".to_string(),
            r#type: "RFID".to_string(),
            additional_info: None,
            custom_data: None,
        };

        let id_token_info = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: None,
            status_info: None,
            group_id_token: None,
            expiry_date: None,
            parent_id_token: None,
            personal_message: None,
            custom_data: None,
        };

        let auth_data = AuthorizationData::new(id_token.clone(), id_token_info.clone());

        assert_eq!(auth_data.id_token(), &id_token);
        assert_eq!(auth_data.id_token_info(), &id_token_info);
        assert_eq!(auth_data.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let id_token = IdTokenType {
            id_token: "tag123".to_string(),
            r#type: "RFID".to_string(),
            additional_info: None,
            custom_data: None,
        };

        let id_token_info = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: None,
            status_info: None,
            group_id_token: None,
            expiry_date: None,
            parent_id_token: None,
            personal_message: None,
            custom_data: None,
        };

        let custom_data = CustomDataType::new("VendorX".to_string());

        let auth_data = AuthorizationData::new(id_token.clone(), id_token_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(auth_data.id_token(), &id_token);
        assert_eq!(auth_data.id_token_info(), &id_token_info);
        assert_eq!(auth_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id_token1 = IdTokenType {
            id_token: "tag123".to_string(),
            r#type: "RFID".to_string(),
            additional_info: None,
            custom_data: None,
        };

        let id_token2 = IdTokenType {
            id_token: "tag456".to_string(),
            r#type: "ISO15693".to_string(),
            additional_info: None,
            custom_data: None,
        };

        let id_token_info1 = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: None,
            status_info: None,
            group_id_token: None,
            expiry_date: None,
            parent_id_token: None,
            personal_message: None,
            custom_data: None,
        };

        let id_token_info2 = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Blocked,
            cache_expiry_date_time: None,
            charging_priority: Some(1),
            status_info: None,
            group_id_token: None,
            expiry_date: None,
            parent_id_token: None,
            personal_message: None,
            custom_data: None,
        };

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut auth_data = AuthorizationData::new(id_token1.clone(), id_token_info1.clone());

        auth_data
            .set_id_token(id_token2.clone())
            .set_id_token_info(id_token_info2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(auth_data.id_token(), &id_token2);
        assert_eq!(auth_data.id_token_info(), &id_token_info2);
        assert_eq!(auth_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        auth_data.set_custom_data(None);
        assert_eq!(auth_data.custom_data(), None);
    }
}
