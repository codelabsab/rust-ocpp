use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::ChargingProfilePurposeEnumType;

/// A ClearChargingProfileType is a filter for charging profiles to be cleared by ClearChargingProfileRequest.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Specifies the id of the EVSE for which to clear charging profiles. An evseId of zero (0) specifies the charging profile for the overall Charging Station. Absence of this parameter means the clearing applies to all charging profiles that match the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    /// Specifies to purpose of the charging profiles that will be cleared, if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,

    /// Specifies the stackLevel for which charging profiles will be cleared, if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub stack_level: Option<i32>,
}

impl ClearChargingProfileType {
    /// Creates a new `ClearChargingProfileType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `ClearChargingProfileType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
            evse_id: None,
            charging_profile_purpose: None,
            stack_level: None,
        }
    }

    /// Sets the EVSE ID.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - ID of the EVSE for which to clear charging profiles
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
    }

    /// Sets the charging profile purpose.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_purpose` - Purpose of the charging profiles to be cleared
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_profile_purpose(
        mut self,
        charging_profile_purpose: ChargingProfilePurposeEnumType,
    ) -> Self {
        self.charging_profile_purpose = Some(charging_profile_purpose);
        self
    }

    /// Sets the stack level.
    ///
    /// # Arguments
    ///
    /// * `stack_level` - Stack level for which charging profiles will be cleared
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_stack_level(mut self, stack_level: i32) -> Self {
        self.stack_level = Some(stack_level);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this clear charging profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the EVSE ID.
    ///
    /// # Returns
    ///
    /// An optional EVSE ID
    pub fn evse_id(&self) -> Option<i32> {
        self.evse_id
    }

    /// Sets the EVSE ID.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - ID of the EVSE for which to clear charging profiles, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Gets the charging profile purpose.
    ///
    /// # Returns
    ///
    /// An optional reference to the charging profile purpose
    pub fn charging_profile_purpose(&self) -> Option<&ChargingProfilePurposeEnumType> {
        self.charging_profile_purpose.as_ref()
    }

    /// Sets the charging profile purpose.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_purpose` - Purpose of the charging profiles to be cleared, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_profile_purpose(
        &mut self,
        charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    ) -> &mut Self {
        self.charging_profile_purpose = charging_profile_purpose;
        self
    }

    /// Gets the stack level.
    ///
    /// # Returns
    ///
    /// An optional stack level
    pub fn stack_level(&self) -> Option<i32> {
        self.stack_level
    }

    /// Sets the stack level.
    ///
    /// # Arguments
    ///
    /// * `stack_level` - Stack level for which charging profiles will be cleared, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_stack_level(&mut self, stack_level: Option<i32>) -> &mut Self {
        self.stack_level = stack_level;
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
    /// * `custom_data` - Custom data for this clear charging profile, or None to clear
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
    fn test_new_clear_charging_profile() {
        let profile = ClearChargingProfileType::new();

        assert_eq!(profile.evse_id(), None);
        assert_eq!(profile.charging_profile_purpose(), None);
        assert_eq!(profile.stack_level(), None);
        assert_eq!(profile.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let profile = ClearChargingProfileType::new()
            .with_evse_id(1)
            .with_charging_profile_purpose(
                ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
            )
            .with_stack_level(3)
            .with_custom_data(custom_data.clone());

        assert_eq!(profile.evse_id(), Some(1));
        assert_eq!(
            profile.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::ChargingStationExternalConstraints)
        );
        assert_eq!(profile.stack_level(), Some(3));
        assert_eq!(profile.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut profile = ClearChargingProfileType::new();

        profile
            .set_evse_id(Some(2))
            .set_charging_profile_purpose(Some(ChargingProfilePurposeEnumType::TxDefaultProfile))
            .set_stack_level(Some(4))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(profile.evse_id(), Some(2));
        assert_eq!(
            profile.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::TxDefaultProfile)
        );
        assert_eq!(profile.stack_level(), Some(4));
        assert_eq!(profile.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        profile
            .set_evse_id(None)
            .set_charging_profile_purpose(None)
            .set_stack_level(None)
            .set_custom_data(None);

        assert_eq!(profile.evse_id(), None);
        assert_eq!(profile.charging_profile_purpose(), None);
        assert_eq!(profile.stack_level(), None);
        assert_eq!(profile.custom_data(), None);
    }
}
