use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, gradient::GradientType};

/// Gradient get type for retrieving gradient settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GradientGetType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The gradient settings.
    pub gradient: GradientType,

    /// Id of the setting.
    #[validate(length(max = 36))]
    pub id: String,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,
}

impl GradientGetType {
    /// Creates a new `GradientGetType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `gradient` - The gradient settings
    /// * `id` - Id of the setting
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    ///
    /// # Returns
    ///
    /// A new `GradientGetType` instance with optional fields set to `None`
    pub fn new(gradient: GradientType, id: String, is_superseded: bool) -> Self {
        Self {
            custom_data: None,
            gradient,
            id,
            is_superseded,
        }
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `GradientGetType` instance
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
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `GradientGetType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the gradient settings.
    ///
    /// # Returns
    ///
    /// A reference to the gradient settings
    pub fn gradient(&self) -> &GradientType {
        &self.gradient
    }

    /// Sets the gradient settings.
    ///
    /// # Arguments
    ///
    /// * `gradient` - The gradient settings
    ///
    /// # Returns
    ///
    /// The modified `GradientGetType` instance
    pub fn set_gradient(&mut self, gradient: GradientType) -> &mut Self {
        self.gradient = gradient;
        self
    }

    /// Gets the id of the setting.
    ///
    /// # Returns
    ///
    /// A reference to the id of the setting
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the id of the setting.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the setting
    ///
    /// # Returns
    ///
    /// The modified `GradientGetType` instance
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
    /// The modified `GradientGetType` instance
    pub fn set_is_superseded(&mut self, is_superseded: bool) -> &mut Self {
        self.is_superseded = is_superseded;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_get_new() {
        let gradient = GradientType::new(1, 5.0);
        let id = "setting1".to_string();
        let is_superseded = false;

        let gradient_get = GradientGetType::new(gradient.clone(), id.clone(), is_superseded);

        assert_eq!(gradient_get.gradient(), &gradient);
        assert_eq!(gradient_get.id(), id);
        assert_eq!(gradient_get.is_superseded(), is_superseded);
        assert_eq!(gradient_get.custom_data(), None);
    }

    #[test]
    fn test_gradient_get_with_methods() {
        let gradient = GradientType::new(1, 5.0);
        let id = "setting1".to_string();
        let is_superseded = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let gradient_get = GradientGetType::new(gradient.clone(), id.clone(), is_superseded)
            .with_custom_data(custom_data.clone());

        assert_eq!(gradient_get.gradient(), &gradient);
        assert_eq!(gradient_get.id(), id);
        assert_eq!(gradient_get.is_superseded(), is_superseded);
        assert_eq!(gradient_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_gradient_get_setters() {
        let gradient1 = GradientType::new(1, 5.0);
        let gradient2 = GradientType::new(2, 10.0);
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let is_superseded1 = false;
        let is_superseded2 = true;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut gradient_get = GradientGetType::new(gradient1.clone(), id1.clone(), is_superseded1);

        gradient_get
            .set_gradient(gradient2.clone())
            .set_id(id2.clone())
            .set_is_superseded(is_superseded2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(gradient_get.gradient(), &gradient2);
        assert_eq!(gradient_get.id(), id2);
        assert_eq!(gradient_get.is_superseded(), is_superseded2);
        assert_eq!(gradient_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        gradient_get.set_custom_data(None);
        assert_eq!(gradient_get.custom_data(), None);
    }
}
