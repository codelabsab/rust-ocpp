use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, status_info::StatusInfoType, id_token::IdTokenType, message_content::MessageContentType};
use crate::v2_1::enumerations::AuthorizationStatusEnumType;

/// Contains status information about an identifier.
/// It is advised to not stop charging if the status is Accepted or ConcurrentTx.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
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

    /// Optional. First preferred user interface language of identifier user.
    /// Contains a language code as defined in [RFC5646].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 8))]
    pub language1: Option<String>,

    /// Optional. Second preferred user interface language of identifier user.
    /// Don't use when language1 is omitted, has to be different from language1.
    /// Contains a language code as defined in [RFC5646].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 8))]
    pub language2: Option<String>,

    /// Optional. Only used when the IdToken is only valid for one or more specific EVSEs,
    /// not for the entire Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<Vec<i32>>,

    /// Optional. A case insensitive identifier to use for the authorization and the load profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub group_id_token: Option<IdTokenType>,

    /// Optional. Contains a case insensitive identifier to use for the user profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub personal_message: Option<MessageContentType>,

    /// Optional. Status information about the authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Contains the date at which idToken should be removed from the Authorization Cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,

    /// Optional. Parent identifier used for charging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_token: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
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
            cache_expiry_date_time: None,
            charging_priority: None,
            language1: None,
            language2: None,
            evse_id: None,
            group_id_token: None,
            personal_message: None,
            status_info: None,
            expiry_date: None,
            parent_id_token: None,
            custom_data: None,
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

    /// Sets the first preferred language.
    ///
    /// # Arguments
    ///
    /// * `language1` - First preferred user interface language
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_language1(mut self, language1: String) -> Self {
        self.language1 = Some(language1);
        self
    }

    /// Sets the second preferred language.
    ///
    /// # Arguments
    ///
    /// * `language2` - Second preferred user interface language
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_language2(mut self, language2: String) -> Self {
        self.language2 = Some(language2);
        self
    }

    /// Sets the EVSE IDs.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - Vector of EVSE IDs where the token is valid
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_evse_id(mut self, evse_id: Vec<i32>) -> Self {
        self.evse_id = Some(evse_id);
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
    pub fn with_group_id_token(mut self, group_id_token: IdTokenType) -> Self {
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
    pub fn with_personal_message(mut self, personal_message: MessageContentType) -> Self {
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

    /// Gets the first preferred language.
    ///
    /// # Returns
    ///
    /// An optional reference to the first preferred language
    pub fn language1(&self) -> Option<&str> {
        self.language1.as_deref()
    }

    /// Sets the first preferred language.
    ///
    /// # Arguments
    ///
    /// * `language1` - First preferred user interface language, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_language1(&mut self, language1: Option<String>) -> &mut Self {
        self.language1 = language1;
        self
    }

    /// Gets the second preferred language.
    ///
    /// # Returns
    ///
    /// An optional reference to the second preferred language
    pub fn language2(&self) -> Option<&str> {
        self.language2.as_deref()
    }

    /// Sets the second preferred language.
    ///
    /// # Arguments
    ///
    /// * `language2` - Second preferred user interface language, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_language2(&mut self, language2: Option<String>) -> &mut Self {
        self.language2 = language2;
        self
    }

    /// Gets the EVSE IDs.
    ///
    /// # Returns
    ///
    /// An optional reference to the vector of EVSE IDs
    pub fn evse_id(&self) -> Option<&Vec<i32>> {
        self.evse_id.as_ref()
    }

    /// Sets the EVSE IDs.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - Vector of EVSE IDs where the token is valid, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_evse_id(&mut self, evse_id: Option<Vec<i32>>) -> &mut Self {
        self.evse_id = evse_id;
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
    pub fn group_id_token(&self) -> Option<&IdTokenType> {
        self.group_id_token.as_ref()
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
    pub fn set_group_id_token(&mut self, group_id_token: Option<IdTokenType>) -> &mut Self {
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
    /// An optional reference to the message for the user profile
    pub fn personal_message(&self) -> Option<&MessageContentType> {
        self.personal_message.as_ref()
    }

    /// Sets the personal message.
    ///
    /// # Arguments
    ///
    /// * `personal_message` - Message for the user profile, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_personal_message(&mut self, personal_message: Option<MessageContentType>) -> &mut Self {
        self.personal_message = personal_message;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::MessageFormatEnumType;

    #[test]
    fn test_new_id_token_info() {
        let status = AuthorizationStatusEnumType::Accepted;

        let token_info = IdTokenInfoType::new(status.clone());

        assert_eq!(token_info.status(), status);
        assert_eq!(token_info.custom_data(), None);
        assert_eq!(token_info.cache_expiry_date_time(), None);
        assert_eq!(token_info.charging_priority(), None);
        assert_eq!(token_info.language1(), None);
        assert_eq!(token_info.language2(), None);
        assert_eq!(token_info.evse_id(), None);
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

        let group_id_token = IdTokenType::new("GROUP123".to_string(), "Local".to_string());

        let personal_message = MessageContentType::new(
            "Welcome User!".to_string(),
            MessageFormatEnumType::ASCII,
            "en".to_string(),
        );

        let evse_ids = vec![1, 2, 3];

        let token_info = IdTokenInfoType::new(status.clone())
            .with_custom_data(custom_data.clone())
            .with_cache_expiry_date_time(now)
            .with_charging_priority(5)
            .with_language1("en".to_string())
            .with_language2("fr".to_string())
            .with_evse_id(evse_ids.clone())
            .with_status_info(status_info.clone())
            .with_group_id_token(group_id_token.clone())
            .with_expiry_date(tomorrow)
            .with_parent_id_token("parent456".to_string())
            .with_personal_message(personal_message.clone());

        assert_eq!(token_info.status(), status);
        assert_eq!(token_info.custom_data(), Some(&custom_data));
        assert_eq!(token_info.cache_expiry_date_time(), Some(&now));
        assert_eq!(token_info.charging_priority(), Some(5));
        assert_eq!(token_info.language1(), Some("en"));
        assert_eq!(token_info.language2(), Some("fr"));
        assert_eq!(token_info.evse_id(), Some(&evse_ids));
        assert_eq!(token_info.status_info(), Some(&status_info));
        assert_eq!(token_info.group_id_token(), Some(&group_id_token));
        assert_eq!(token_info.expiry_date(), Some(&tomorrow));
        assert_eq!(token_info.parent_id_token(), Some("parent456"));
        assert_eq!(token_info.personal_message(), Some(&personal_message));
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

        let group_id_token = IdTokenType::new("GROUP123".to_string(), "Local".to_string());

        let personal_message = MessageContentType::new(
            "Welcome User!".to_string(),
            MessageFormatEnumType::ASCII,
            "en".to_string(),
        );

        let evse_ids = vec![1, 2, 3];

        let mut token_info = IdTokenInfoType::new(status1.clone());

        token_info
            .set_status(status2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_cache_expiry_date_time(Some(now))
            .set_charging_priority(Some(5))
            .set_language1(Some("en".to_string()))
            .set_language2(Some("fr".to_string()))
            .set_evse_id(Some(evse_ids.clone()))
            .set_status_info(Some(status_info.clone()))
            .set_group_id_token(Some(group_id_token.clone()))
            .set_expiry_date(Some(tomorrow))
            .set_parent_id_token(Some("parent456".to_string()))
            .set_personal_message(Some(personal_message.clone()));

        assert_eq!(token_info.status(), status2);
        assert_eq!(token_info.custom_data(), Some(&custom_data));
        assert_eq!(token_info.cache_expiry_date_time(), Some(&now));
        assert_eq!(token_info.charging_priority(), Some(5));
        assert_eq!(token_info.language1(), Some("en"));
        assert_eq!(token_info.language2(), Some("fr"));
        assert_eq!(token_info.evse_id(), Some(&evse_ids));
        assert_eq!(token_info.status_info(), Some(&status_info));
        assert_eq!(token_info.group_id_token(), Some(&group_id_token));
        assert_eq!(token_info.expiry_date(), Some(&tomorrow));
        assert_eq!(token_info.parent_id_token(), Some("parent456"));
        assert_eq!(token_info.personal_message(), Some(&personal_message));

        // Test clearing optional fields
        token_info
            .set_custom_data(None)
            .set_cache_expiry_date_time(None)
            .set_charging_priority(None)
            .set_language1(None)
            .set_language2(None)
            .set_evse_id(None)
            .set_status_info(None)
            .set_group_id_token(None)
            .set_expiry_date(None)
            .set_parent_id_token(None)
            .set_personal_message(None);

        assert_eq!(token_info.custom_data(), None);
        assert_eq!(token_info.cache_expiry_date_time(), None);
        assert_eq!(token_info.charging_priority(), None);
        assert_eq!(token_info.language1(), None);
        assert_eq!(token_info.language2(), None);
        assert_eq!(token_info.evse_id(), None);
        assert_eq!(token_info.status_info(), None);
        assert_eq!(token_info.group_id_token(), None);
        assert_eq!(token_info.expiry_date(), None);
        assert_eq!(token_info.parent_id_token(), None);
        assert_eq!(token_info.personal_message(), None);
    }
}
