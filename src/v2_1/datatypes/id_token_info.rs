use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, status_info::StatusInfoType};
use crate::v2_1::enumerations::AuthorizationStatusEnumType;

/// Contains status information about an identifier.
/// It is advised to not stop charging if the status is Accepted or ConcurrentTx.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains whether the identifier is allowed for charging.
    pub status: AuthorizationStatusEnumType,

    /// Optional. Only filled in when the status is ConcurrentTx.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiry_date_time: Option<DateTime<Utc>>,

    /// Optional. Priority from a business point of view.
    /// Default priority is 0, The range is from -9 to 9.
    /// Higher values indicate a higher priority.
    /// The chargingPriority in a ChargingProfile SHALL overrule this priority range.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = -9, max = 9))]
    pub charging_priority: Option<i8>,

    /// Optional. Contains information about authorization status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. A case insensitive identifier to use for the authorization and the load profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub group_id_token: Option<String>,

    /// Optional. Contains the date at which idToken should be removed from the Authorization Cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,

    /// Optional. This contains the identifier to use for charging.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub parent_id_token: Option<String>,

    /// Optional. Contains a case insensitive identifier to use for the user profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub personal_message: Option<String>,
}

impl IdTokenInfoType {
    /// Creates a new `IdTokenInfoType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Authorization status for the identifier
    ///
    /// # Returns
    ///
    /// A new instance of `IdTokenInfoType` with optional fields set to `None`
    pub fn new(status: AuthorizationStatusEnumType) -> Self {
        Self {
            status,
            custom_data: None,
            cache_expiry_date_time: None,
            charging_priority: None,
            status_info: None,
            group_id_token: None,
            expiry_date: None,
            parent_id_token: None,
            personal_message: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this ID token info
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the cache expiry date time.
    ///
    /// # Arguments
    ///
    /// * `cache_expiry_date_time` - Date and time when the token should be removed from cache
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_cache_expiry_date_time(mut self, cache_expiry_date_time: DateTime<Utc>) -> Self {
        self.cache_expiry_date_time = Some(cache_expiry_date_time);
        self
    }

    /// Sets the charging priority.
    ///
    /// # Arguments
    ///
    /// * `charging_priority` - Priority from a business point of view (-9 to 9)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_priority(mut self, charging_priority: i8) -> Self {
        self.charging_priority = Some(charging_priority);
        self
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Information about authorization status
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the group ID token.
    ///
    /// # Arguments
    ///
    /// * `group_id_token` - Identifier to use for authorization and load profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_group_id_token(mut self, group_id_token: String) -> Self {
        self.group_id_token = Some(group_id_token);
        self
    }

    /// Sets the expiry date.
    ///
    /// # Arguments
    ///
    /// * `expiry_date` - Date at which the token should be removed from Authorization Cache
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_expiry_date(mut self, expiry_date: DateTime<Utc>) -> Self {
        self.expiry_date = Some(expiry_date);
        self
    }

    /// Sets the parent ID token.
    ///
    /// # Arguments
    ///
    /// * `parent_id_token` - Identifier to use for charging
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_parent_id_token(mut self, parent_id_token: String) -> Self {
        self.parent_id_token = Some(parent_id_token);
        self
    }

    /// Sets the personal message.
    ///
    /// # Arguments
    ///
    /// * `personal_message` - Case insensitive identifier to use for the user profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_personal_message(mut self, personal_message: String) -> Self {
        self.personal_message = Some(personal_message);
        self
    }

    /// Gets the status.
    ///
    /// # Returns
    ///
    /// The authorization status
    pub fn status(&self) -> AuthorizationStatusEnumType {
        self.status.clone()
    }

    /// Sets the status.
    ///
    /// # Arguments
    ///
    /// * `status` - Authorization status for the identifier
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: AuthorizationStatusEnumType) -> &mut Self {
        self.status = status;
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
    /// * `custom_data` - Custom data for this ID token info, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the cache expiry date time.
    ///
    /// # Returns
    ///
    /// An optional reference to the cache expiry date time
    pub fn cache_expiry_date_time(&self) -> Option<&DateTime<Utc>> {
        self.cache_expiry_date_time.as_ref()
    }

    /// Sets the cache expiry date time.
    ///
    /// # Arguments
    ///
    /// * `cache_expiry_date_time` - Date and time when the token should be removed from cache, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_cache_expiry_date_time(
        &mut self,
        cache_expiry_date_time: Option<DateTime<Utc>>,
    ) -> &mut Self {
        self.cache_expiry_date_time = cache_expiry_date_time;
        self
    }

    /// Gets the charging priority.
    ///
    /// # Returns
    ///
    /// An optional charging priority value (-9 to 9)
    pub fn charging_priority(&self) -> Option<i8> {
        self.charging_priority
    }

    /// Sets the charging priority.
    ///
    /// # Arguments
    ///
    /// * `charging_priority` - Priority from a business point of view (-9 to 9), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_priority(&mut self, charging_priority: Option<i8>) -> &mut Self {
        self.charging_priority = charging_priority;
        self
    }

    /// Gets the status info.
    ///
    /// # Returns
    ///
    /// An optional reference to information about authorization status
    pub fn status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Information about authorization status, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    /// Gets the group ID token.
    ///
    /// # Returns
    ///
    /// An optional reference to the identifier for authorization and load profile
    pub fn group_id_token(&self) -> Option<&str> {
        self.group_id_token.as_deref()
    }

    /// Sets the group ID token.
    ///
    /// # Arguments
    ///
    /// * `group_id_token` - Identifier to use for authorization and load profile, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_group_id_token(&mut self, group_id_token: Option<String>) -> &mut Self {
        self.group_id_token = group_id_token;
        self
    }

    /// Gets the expiry date.
    ///
    /// # Returns
    ///
    /// An optional reference to the date at which the token should be removed
    pub fn expiry_date(&self) -> Option<&DateTime<Utc>> {
        self.expiry_date.as_ref()
    }

    /// Sets the expiry date.
    ///
    /// # Arguments
    ///
    /// * `expiry_date` - Date at which the token should be removed from Authorization Cache, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_expiry_date(&mut self, expiry_date: Option<DateTime<Utc>>) -> &mut Self {
        self.expiry_date = expiry_date;
        self
    }

    /// Gets the parent ID token.
    ///
    /// # Returns
    ///
    /// An optional reference to the identifier to use for charging
    pub fn parent_id_token(&self) -> Option<&str> {
        self.parent_id_token.as_deref()
    }

    /// Sets the parent ID token.
    ///
    /// # Arguments
    ///
    /// * `parent_id_token` - Identifier to use for charging, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_parent_id_token(&mut self, parent_id_token: Option<String>) -> &mut Self {
        self.parent_id_token = parent_id_token;
        self
    }

    /// Gets the personal message.
    ///
    /// # Returns
    ///
    /// An optional reference to the identifier for the user profile
    pub fn personal_message(&self) -> Option<&str> {
        self.personal_message.as_deref()
    }

    /// Sets the personal message.
    ///
    /// # Arguments
    ///
    /// * `personal_message` - Case insensitive identifier for the user profile, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_personal_message(&mut self, personal_message: Option<String>) -> &mut Self {
        self.personal_message = personal_message;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_id_token_info() {
        let status = AuthorizationStatusEnumType::Accepted;

        let token_info = IdTokenInfoType::new(status.clone());

        assert_eq!(token_info.status(), status);
        assert_eq!(token_info.custom_data(), None);
        assert_eq!(token_info.cache_expiry_date_time(), None);
        assert_eq!(token_info.charging_priority(), None);
        assert_eq!(token_info.status_info(), None);
        assert_eq!(token_info.group_id_token(), None);
        assert_eq!(token_info.expiry_date(), None);
        assert_eq!(token_info.parent_id_token(), None);
        assert_eq!(token_info.personal_message(), None);
    }

    #[test]
    fn test_with_methods() {
        let status = AuthorizationStatusEnumType::Accepted;
        let now = Utc::now();
        let tomorrow = now + chrono::Duration::days(1);

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let status_info = StatusInfoType {
            reason_code: "200".to_string(),
            additional_info: Some("Additional Info".to_string()),
            custom_data: None,
        };

        let token_info = IdTokenInfoType::new(status.clone())
            .with_custom_data(custom_data.clone())
            .with_cache_expiry_date_time(now)
            .with_charging_priority(5)
            .with_status_info(status_info.clone())
            .with_group_id_token("group123".to_string())
            .with_expiry_date(tomorrow)
            .with_parent_id_token("parent456".to_string())
            .with_personal_message("Welcome User!".to_string());

        assert_eq!(token_info.status(), status);
        assert_eq!(token_info.custom_data(), Some(&custom_data));
        assert_eq!(token_info.cache_expiry_date_time(), Some(&now));
        assert_eq!(token_info.charging_priority(), Some(5));
        assert_eq!(token_info.status_info(), Some(&status_info));
        assert_eq!(token_info.group_id_token(), Some("group123"));
        assert_eq!(token_info.expiry_date(), Some(&tomorrow));
        assert_eq!(token_info.parent_id_token(), Some("parent456"));
        assert_eq!(token_info.personal_message(), Some("Welcome User!"));
    }

    #[test]
    fn test_setter_methods() {
        let status1 = AuthorizationStatusEnumType::Accepted;
        let status2 = AuthorizationStatusEnumType::Blocked;
        let now = Utc::now();
        let tomorrow = now + chrono::Duration::days(1);

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let status_info = StatusInfoType {
            reason_code: "200".to_string(),
            additional_info: Some("Additional Info".to_string()),
            custom_data: None,
        };

        let mut token_info = IdTokenInfoType::new(status1.clone());

        token_info
            .set_status(status2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_cache_expiry_date_time(Some(now))
            .set_charging_priority(Some(5))
            .set_status_info(Some(status_info.clone()))
            .set_group_id_token(Some("group123".to_string()))
            .set_expiry_date(Some(tomorrow))
            .set_parent_id_token(Some("parent456".to_string()))
            .set_personal_message(Some("Welcome User!".to_string()));

        assert_eq!(token_info.status(), status2);
        assert_eq!(token_info.custom_data(), Some(&custom_data));
        assert_eq!(token_info.cache_expiry_date_time(), Some(&now));
        assert_eq!(token_info.charging_priority(), Some(5));
        assert_eq!(token_info.status_info(), Some(&status_info));
        assert_eq!(token_info.group_id_token(), Some("group123"));
        assert_eq!(token_info.expiry_date(), Some(&tomorrow));
        assert_eq!(token_info.parent_id_token(), Some("parent456"));
        assert_eq!(token_info.personal_message(), Some("Welcome User!"));

        // Test clearing optional fields
        token_info
            .set_custom_data(None)
            .set_cache_expiry_date_time(None)
            .set_charging_priority(None)
            .set_status_info(None)
            .set_group_id_token(None)
            .set_expiry_date(None)
            .set_parent_id_token(None)
            .set_personal_message(None);

        assert_eq!(token_info.custom_data(), None);
        assert_eq!(token_info.cache_expiry_date_time(), None);
        assert_eq!(token_info.charging_priority(), None);
        assert_eq!(token_info.status_info(), None);
        assert_eq!(token_info.group_id_token(), None);
        assert_eq!(token_info.expiry_date(), None);
        assert_eq!(token_info.parent_id_token(), None);
        assert_eq!(token_info.personal_message(), None);
    }
}
