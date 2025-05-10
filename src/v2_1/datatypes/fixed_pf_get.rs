use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, fixed_pf::FixedPFType};

/// Fixed power factor get type for retrieving fixed power factor settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FixedPFGetType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The fixed power factor settings.
    pub fixed_pf: FixedPFType,

    /// Id of the setting.
    #[validate(length(max = 36))]
    pub id: String,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,
}

impl FixedPFGetType {
    /// Creates a new `FixedPFGetType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `fixed_pf` - The fixed power factor settings
    /// * `id` - Id of the setting
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    ///
    /// # Returns
    ///
    /// A new instance of `FixedPFGetType` with optional fields set to `None`
    pub fn new(fixed_pf: FixedPFType, id: String, is_superseded: bool) -> Self {
        Self {
            fixed_pf,
            id,
            is_superseded,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this fixed power factor get
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the fixed power factor settings.
    ///
    /// # Returns
    ///
    /// A reference to the fixed power factor settings
    pub fn fixed_pf(&self) -> &FixedPFType {
        &self.fixed_pf
    }

    /// Sets the fixed power factor settings.
    ///
    /// # Arguments
    ///
    /// * `fixed_pf` - The fixed power factor settings
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_fixed_pf(&mut self, fixed_pf: FixedPFType) -> &mut Self {
        self.fixed_pf = fixed_pf;
        self
    }

    /// Gets the ID of the setting.
    ///
    /// # Returns
    ///
    /// A reference to the ID of the setting
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the ID of the setting.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets whether this setting is superseded.
    ///
    /// # Returns
    ///
    /// True if this setting is superseded by a higher priority setting
    pub fn is_superseded(&self) -> bool {
        self.is_superseded
    }

    /// Sets whether this setting is superseded.
    ///
    /// # Arguments
    ///
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_is_superseded(&mut self, is_superseded: bool) -> &mut Self {
        self.is_superseded = is_superseded;
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
    /// * `custom_data` - Custom data for this fixed power factor get, or None to clear
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
    fn test_new_fixed_pf_get() {
        let fixed_pf = FixedPFType::new(1, 0.95);
        let id = "setting1".to_string();
        let is_superseded = false;

        let fixed_pf_get = FixedPFGetType::new(fixed_pf.clone(), id.clone(), is_superseded);

        assert_eq!(fixed_pf_get.fixed_pf(), &fixed_pf);
        assert_eq!(fixed_pf_get.id(), id);
        assert_eq!(fixed_pf_get.is_superseded(), is_superseded);
        assert_eq!(fixed_pf_get.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let fixed_pf = FixedPFType::new(1, 0.95);
        let id = "setting1".to_string();
        let is_superseded = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let fixed_pf_get = FixedPFGetType::new(fixed_pf.clone(), id.clone(), is_superseded)
            .with_custom_data(custom_data.clone());

        assert_eq!(fixed_pf_get.fixed_pf(), &fixed_pf);
        assert_eq!(fixed_pf_get.id(), id);
        assert_eq!(fixed_pf_get.is_superseded(), is_superseded);
        assert_eq!(fixed_pf_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let fixed_pf1 = FixedPFType::new(1, 0.95);
        let fixed_pf2 = FixedPFType::new(2, -0.9);
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let is_superseded1 = false;
        let is_superseded2 = true;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut fixed_pf_get = FixedPFGetType::new(fixed_pf1.clone(), id1.clone(), is_superseded1);

        fixed_pf_get
            .set_fixed_pf(fixed_pf2.clone())
            .set_id(id2.clone())
            .set_is_superseded(is_superseded2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(fixed_pf_get.fixed_pf(), &fixed_pf2);
        assert_eq!(fixed_pf_get.id(), id2);
        assert_eq!(fixed_pf_get.is_superseded(), is_superseded2);
        assert_eq!(fixed_pf_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        fixed_pf_get.set_custom_data(None);
        assert_eq!(fixed_pf_get.custom_data(), None);
    }
}
