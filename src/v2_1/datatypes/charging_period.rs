use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{cost_dimension::CostDimensionType, custom_data::CustomDataType};

/// A ChargingPeriodType consists of a start time, and a list of possible values that influence this period,
/// for example: amount of energy charged this period, maximum current during this period etc.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingPeriodType {
    /// Start timestamp of charging period. A period ends when the next period starts.
    /// The last period ends when the session ends.
    pub start_period: DateTime<Utc>,

    /// List of dimensions that influence this period.
    pub dimensions: Vec<CostDimensionType>,

    /// Unique identifier of the Tariff that was used to calculate cost.
    /// If not provided, then cost was calculated by some other means.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 60))]
    pub tariff_id: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingPeriodType {
    /// Creates a new `ChargingPeriodType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start timestamp of charging period
    /// * `dimensions` - List of dimensions that influence this period
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingPeriodType` with optional fields set to `None`
    pub fn new(start_period: DateTime<Utc>, dimensions: Vec<CostDimensionType>) -> Self {
        Self {
            start_period,
            dimensions,
            tariff_id: None,
            custom_data: None,
        }
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Unique identifier of the Tariff that was used to calculate cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_tariff_id(mut self, tariff_id: String) -> Self {
        self.tariff_id = Some(tariff_id);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the start period.
    ///
    /// # Returns
    ///
    /// The start timestamp of the charging period
    pub fn start_period(&self) -> &DateTime<Utc> {
        &self.start_period
    }

    /// Sets the start period.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start timestamp of charging period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_period(&mut self, start_period: DateTime<Utc>) -> &mut Self {
        self.start_period = start_period;
        self
    }

    /// Gets the dimensions.
    ///
    /// # Returns
    ///
    /// A reference to the list of dimensions that influence this period
    pub fn dimensions(&self) -> &Vec<CostDimensionType> {
        &self.dimensions
    }

    /// Sets the dimensions.
    ///
    /// # Arguments
    ///
    /// * `dimensions` - List of dimensions that influence this period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_dimensions(&mut self, dimensions: Vec<CostDimensionType>) -> &mut Self {
        self.dimensions = dimensions;
        self
    }

    /// Gets the tariff ID.
    ///
    /// # Returns
    ///
    /// An optional reference to the tariff ID
    pub fn tariff_id(&self) -> Option<&String> {
        self.tariff_id.as_ref()
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Unique identifier of the Tariff that was used to calculate cost, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tariff_id(&mut self, tariff_id: Option<String>) -> &mut Self {
        self.tariff_id = tariff_id;
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
    /// * `custom_data` - Custom data for this charging period, or None to clear
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
    use crate::v2_1::enumerations::CostDimensionEnumType;

    #[test]
    fn test_new_charging_period() {
        let start_time = Utc::now();
        let dimension = CostDimensionType {
            r#type: CostDimensionEnumType::Energy,
            volume: 10.5,
            custom_data: None,
        };

        let period = ChargingPeriodType::new(start_time, vec![dimension.clone()]);

        assert_eq!(period.start_period(), &start_time);
        assert_eq!(period.dimensions().len(), 1);
        assert_eq!(period.dimensions()[0].r#type, CostDimensionEnumType::Energy);
        assert_eq!(period.dimensions()[0].volume, 10.5);
        assert_eq!(period.tariff_id(), None);
        assert_eq!(period.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let start_time = Utc::now();
        let dimension = CostDimensionType {
            r#type: CostDimensionEnumType::Energy,
            volume: 10.5,
            custom_data: None,
        };
        let custom_data = CustomDataType::new("VendorX".to_string());

        let period = ChargingPeriodType::new(start_time, vec![dimension.clone()])
            .with_tariff_id("tariff-123".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(period.start_period(), &start_time);
        assert_eq!(period.dimensions().len(), 1);
        assert_eq!(period.dimensions()[0].r#type, CostDimensionEnumType::Energy);
        assert_eq!(period.dimensions()[0].volume, 10.5);
        assert_eq!(period.tariff_id(), Some(&"tariff-123".to_string()));
        assert_eq!(period.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let start_time = Utc::now();
        let dimension1 = CostDimensionType {
            r#type: CostDimensionEnumType::Energy,
            volume: 10.5,
            custom_data: None,
        };
        let dimension2 = CostDimensionType {
            r#type: CostDimensionEnumType::ChargingTime,
            volume: 30.0,
            custom_data: None,
        };
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut period = ChargingPeriodType::new(start_time, vec![dimension1.clone()]);

        let new_time = Utc::now();
        period
            .set_start_period(new_time)
            .set_dimensions(vec![dimension1.clone(), dimension2.clone()])
            .set_tariff_id(Some("tariff-456".to_string()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(period.start_period(), &new_time);
        assert_eq!(period.dimensions().len(), 2);
        assert_eq!(period.dimensions()[0].r#type, CostDimensionEnumType::Energy);
        assert_eq!(period.dimensions()[0].volume, 10.5);
        assert_eq!(period.dimensions()[1].r#type, CostDimensionEnumType::ChargingTime);
        assert_eq!(period.dimensions()[1].volume, 30.0);
        assert_eq!(period.tariff_id(), Some(&"tariff-456".to_string()));
        assert_eq!(period.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        period
            .set_tariff_id(None)
            .set_custom_data(None);

        assert_eq!(period.tariff_id(), None);
        assert_eq!(period.custom_data(), None);
    }
}
