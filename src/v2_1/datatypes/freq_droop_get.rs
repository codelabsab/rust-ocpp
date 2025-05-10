use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, freq_droop::FreqDroopType};

/// Frequency droop get type for retrieving frequency droop settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FreqDroopGetType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The frequency droop settings.
    pub freq_droop: FreqDroopType,

    /// Id of the setting.
    #[validate(length(max = 36))]
    pub id: String,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,
}

impl FreqDroopGetType {
    /// Creates a new `FreqDroopGetType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `freq_droop` - The frequency droop settings
    /// * `id` - Id of the setting
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    ///
    /// # Returns
    ///
    /// A new instance of `FreqDroopGetType` with optional fields set to `None`
    pub fn new(freq_droop: FreqDroopType, id: String, is_superseded: bool) -> Self {
        Self {
            freq_droop,
            id,
            is_superseded,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this frequency droop get
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the frequency droop settings.
    ///
    /// # Returns
    ///
    /// A reference to the frequency droop settings
    pub fn freq_droop(&self) -> &FreqDroopType {
        &self.freq_droop
    }

    /// Sets the frequency droop settings.
    ///
    /// # Arguments
    ///
    /// * `freq_droop` - The frequency droop settings
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_freq_droop(&mut self, freq_droop: FreqDroopType) -> &mut Self {
        self.freq_droop = freq_droop;
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
    /// * `custom_data` - Custom data for this frequency droop get, or None to clear
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
    fn test_new_freq_droop_get() {
        let freq_droop = FreqDroopType::new(1, 5.0, 0.1, 0.05, 2.0);
        let id = "setting1".to_string();
        let is_superseded = false;

        let freq_droop_get = FreqDroopGetType::new(freq_droop.clone(), id.clone(), is_superseded);

        assert_eq!(freq_droop_get.freq_droop(), &freq_droop);
        assert_eq!(freq_droop_get.id(), id);
        assert_eq!(freq_droop_get.is_superseded(), is_superseded);
        assert_eq!(freq_droop_get.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let freq_droop = FreqDroopType::new(1, 5.0, 0.1, 0.05, 2.0);
        let id = "setting1".to_string();
        let is_superseded = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let freq_droop_get = FreqDroopGetType::new(freq_droop.clone(), id.clone(), is_superseded)
            .with_custom_data(custom_data.clone());

        assert_eq!(freq_droop_get.freq_droop(), &freq_droop);
        assert_eq!(freq_droop_get.id(), id);
        assert_eq!(freq_droop_get.is_superseded(), is_superseded);
        assert_eq!(freq_droop_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let freq_droop1 = FreqDroopType::new(1, 5.0, 0.1, 0.05, 2.0);
        let freq_droop2 = FreqDroopType::new(2, 6.0, 0.2, 0.1, 3.0);
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let is_superseded1 = false;
        let is_superseded2 = true;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut freq_droop_get =
            FreqDroopGetType::new(freq_droop1.clone(), id1.clone(), is_superseded1);

        freq_droop_get
            .set_freq_droop(freq_droop2.clone())
            .set_id(id2.clone())
            .set_is_superseded(is_superseded2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(freq_droop_get.freq_droop(), &freq_droop2);
        assert_eq!(freq_droop_get.id(), id2);
        assert_eq!(freq_droop_get.is_superseded(), is_superseded2);
        assert_eq!(freq_droop_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        freq_droop_get.set_custom_data(None);
        assert_eq!(freq_droop_get.custom_data(), None);
    }
}
