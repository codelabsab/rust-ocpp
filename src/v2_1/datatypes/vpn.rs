use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// VPN Configuration settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VPNType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. VPN Server Address.
    #[validate(length(max = 512))]
    pub server: String,

    /// Required. VPN User.
    #[validate(length(max = 20))]
    pub user: String,

    /// Required. VPN Password.
    #[validate(length(max = 20))]
    pub password: String,

    /// Required. VPN Key.
    #[validate(length(max = 255))]
    pub key: String,

    /// Required. VPN Type.
    #[validate(length(max = 32))]
    pub r#type: String,
}

impl VPNType {
    /// Creates a new `VPNType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `server` - VPN Server Address
    /// * `user` - VPN User
    /// * `password` - VPN Password
    /// * `key` - VPN Key
    /// * `type` - VPN Type
    ///
    /// # Returns
    ///
    /// A new instance of `VPNType` with optional fields set to `None`
    pub fn new(
        server: String,
        user: String,
        password: String,
        key: String,
        r#type: String,
    ) -> Self {
        Self {
            server,
            user,
            password,
            key,
            r#type,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this VPN configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the server address.
    ///
    /// # Returns
    ///
    /// A reference to the VPN server address
    pub fn server(&self) -> &str {
        &self.server
    }

    /// Sets the server address.
    ///
    /// # Arguments
    ///
    /// * `server` - VPN Server Address
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_server(&mut self, server: String) -> &mut Self {
        self.server = server;
        self
    }

    /// Gets the user.
    ///
    /// # Returns
    ///
    /// A reference to the VPN user
    pub fn user(&self) -> &str {
        &self.user
    }

    /// Sets the user.
    ///
    /// # Arguments
    ///
    /// * `user` - VPN User
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_user(&mut self, user: String) -> &mut Self {
        self.user = user;
        self
    }

    /// Gets the password.
    ///
    /// # Returns
    ///
    /// A reference to the VPN password
    pub fn password(&self) -> &str {
        &self.password
    }

    /// Sets the password.
    ///
    /// # Arguments
    ///
    /// * `password` - VPN Password
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }

    /// Gets the key.
    ///
    /// # Returns
    ///
    /// A reference to the VPN key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Sets the key.
    ///
    /// # Arguments
    ///
    /// * `key` - VPN Key
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_key(&mut self, key: String) -> &mut Self {
        self.key = key;
        self
    }

    /// Gets the VPN type.
    ///
    /// # Returns
    ///
    /// A reference to the VPN type
    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    /// Sets the VPN type.
    ///
    /// # Arguments
    ///
    /// * `type` - VPN Type
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, r#type: String) -> &mut Self {
        self.r#type = r#type;
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
    /// * `custom_data` - Custom data for this VPN configuration, or None to clear
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
    fn test_new_vpn() {
        let vpn = VPNType::new(
            "vpn.example.com".to_string(),
            "user1".to_string(),
            "password123".to_string(),
            "secret_key".to_string(),
            "IKEv2".to_string(),
        );

        assert_eq!(vpn.server(), "vpn.example.com");
        assert_eq!(vpn.user(), "user1");
        assert_eq!(vpn.password(), "password123");
        assert_eq!(vpn.key(), "secret_key");
        assert_eq!(vpn.r#type(), "IKEv2");
        assert_eq!(vpn.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let vpn = VPNType::new(
            "vpn.example.com".to_string(),
            "user1".to_string(),
            "password123".to_string(),
            "secret_key".to_string(),
            "IKEv2".to_string(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(vpn.server(), "vpn.example.com");
        assert_eq!(vpn.user(), "user1");
        assert_eq!(vpn.password(), "password123");
        assert_eq!(vpn.key(), "secret_key");
        assert_eq!(vpn.r#type(), "IKEv2");
        assert_eq!(vpn.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut vpn = VPNType::new(
            "vpn.example.com".to_string(),
            "user1".to_string(),
            "password123".to_string(),
            "secret_key".to_string(),
            "IKEv2".to_string(),
        );

        vpn.set_server("new-vpn.example.com".to_string())
            .set_user("user2".to_string())
            .set_password("new_password".to_string())
            .set_key("new_key".to_string())
            .set_type("IPSec".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(vpn.server(), "new-vpn.example.com");
        assert_eq!(vpn.user(), "user2");
        assert_eq!(vpn.password(), "new_password");
        assert_eq!(vpn.key(), "new_key");
        assert_eq!(vpn.r#type(), "IPSec");
        assert_eq!(vpn.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        vpn.set_custom_data(None);
        assert_eq!(vpn.custom_data(), None);
    }
}
