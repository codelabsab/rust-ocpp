use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{datatypes::CustomDataType, enumerations::ChargingProfilePurposeEnumType};

/// A ChargingProfileCriterionType is a filter for charging profiles to be selected by a GetChargingProfilesRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileCriterionType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Defines the purpose of the schedule transferred by this profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,

    /// Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values. Lowest level is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,

    /// List of all the chargingProfileIds requested. Any ChargingProfile that matches one of these profiles will be reported.
    /// If omitted, the Charging Station SHALL not filter on chargingProfileId.
    /// This field SHALL NOT contain more ids than set in ChargingProfileEntries.maxLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<Vec<i32>>,
}

impl ChargingProfileCriterionType {
    /// Creates a new `ChargingProfileCriterionType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingProfileCriterionType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
            charging_profile_purpose: None,
            stack_level: None,
            charging_profile_id: None,
        }
    }

    /// Sets the charging profile purpose.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_purpose` - Defines the purpose of the schedule transferred by this profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_profile_purpose(mut self, charging_profile_purpose: ChargingProfilePurposeEnumType) -> Self {
        self.charging_profile_purpose = Some(charging_profile_purpose);
        self
    }

    /// Sets the stack level.
    ///
    /// # Arguments
    ///
    /// * `stack_level` - Value determining level in hierarchy stack of profiles
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_stack_level(mut self, stack_level: i32) -> Self {
        self.stack_level = Some(stack_level);
        self
    }

    /// Sets the charging profile IDs.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_id` - List of all the chargingProfileIds requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_profile_id(mut self, charging_profile_id: Vec<i32>) -> Self {
        self.charging_profile_id = Some(charging_profile_id);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging profile criterion
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `charging_profile_purpose` - Defines the purpose of the schedule transferred by this profile, or None to clear
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
    /// An optional stack level value
    pub fn stack_level(&self) -> Option<i32> {
        self.stack_level
    }

    /// Sets the stack level.
    ///
    /// # Arguments
    ///
    /// * `stack_level` - Value determining level in hierarchy stack of profiles, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_stack_level(&mut self, stack_level: Option<i32>) -> &mut Self {
        self.stack_level = stack_level;
        self
    }

    /// Gets the charging profile IDs.
    ///
    /// # Returns
    ///
    /// An optional reference to the list of charging profile IDs
    pub fn charging_profile_id(&self) -> Option<&Vec<i32>> {
        self.charging_profile_id.as_ref()
    }

    /// Sets the charging profile IDs.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_id` - List of all the chargingProfileIds requested, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_profile_id(&mut self, charging_profile_id: Option<Vec<i32>>) -> &mut Self {
        self.charging_profile_id = charging_profile_id;
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
    /// * `custom_data` - Custom data for this charging profile criterion, or None to clear
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
    fn test_new_charging_profile_criterion() {
        let criterion = ChargingProfileCriterionType::new();

        assert_eq!(criterion.charging_profile_purpose(), None);
        assert_eq!(criterion.stack_level(), None);
        assert_eq!(criterion.charging_profile_id(), None);
        assert_eq!(criterion.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let criterion = ChargingProfileCriterionType::new()
            .with_charging_profile_purpose(ChargingProfilePurposeEnumType::ChargingStationExternalConstraints)
            .with_stack_level(3)
            .with_charging_profile_id(vec![1, 2, 3])
            .with_custom_data(custom_data.clone());

        assert_eq!(
            criterion.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::ChargingStationExternalConstraints)
        );
        assert_eq!(criterion.stack_level(), Some(3));
        assert_eq!(criterion.charging_profile_id(), Some(&vec![1, 2, 3]));
        assert_eq!(criterion.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut criterion = ChargingProfileCriterionType::new();

        criterion
            .set_charging_profile_purpose(Some(ChargingProfilePurposeEnumType::TxDefaultProfile))
            .set_stack_level(Some(2))
            .set_charging_profile_id(Some(vec![4, 5, 6]))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            criterion.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::TxDefaultProfile)
        );
        assert_eq!(criterion.stack_level(), Some(2));
        assert_eq!(criterion.charging_profile_id(), Some(&vec![4, 5, 6]));
        assert_eq!(criterion.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        criterion
            .set_charging_profile_purpose(None)
            .set_stack_level(None)
            .set_charging_profile_id(None)
            .set_custom_data(None);

        assert_eq!(criterion.charging_profile_purpose(), None);
        assert_eq!(criterion.stack_level(), None);
        assert_eq!(criterion.charging_profile_id(), None);
        assert_eq!(criterion.custom_data(), None);
    }
}
