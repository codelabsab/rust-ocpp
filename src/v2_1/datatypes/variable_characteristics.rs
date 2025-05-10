use super::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Defines the datatype of the variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataEnumType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "dateTime")]
    DateTime,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "OptionList")]
    OptionList,
    #[serde(rename = "SequenceList")]
    SequenceList,
    #[serde(rename = "MemberList")]
    MemberList,
}

/// Fixed read-only parameters of a variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Unit of the variable. When the variable represents a measurand from the measurand enumeration, this field SHALL contain the unit of the measurand as used in the signedMeterValue field defined in Part 2.
    #[validate(length(max = 16))]
    pub unit: String,

    /// Required. Data type of this variable.
    pub data_type: DataEnumType,

    /// Required. Minimum possible value of this variable.
    #[validate(length(max = 1000))]
    pub min_limit: String,

    /// Required. Maximum possible value of this variable.
    #[validate(length(max = 1000))]
    pub max_limit: String,

    /// Required. When true, value from the Charging Station may not be set to null.
    pub supports_monitoring: bool,
}

impl VariableCharacteristicsType {
    /// Creates a new `VariableCharacteristicsType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `unit` - Unit of the variable
    /// * `data_type` - Data type of this variable
    /// * `min_limit` - Minimum possible value of this variable
    /// * `max_limit` - Maximum possible value of this variable
    /// * `supports_monitoring` - When true, value from the Charging Station may not be set to null
    ///
    /// # Returns
    ///
    /// A new `VariableCharacteristicsType` instance with optional fields set to `None`
    pub fn new(
        unit: String,
        data_type: DataEnumType,
        min_limit: String,
        max_limit: String,
        supports_monitoring: bool,
    ) -> Self {
        Self {
            custom_data: None,
            unit,
            data_type,
            min_limit,
            max_limit,
            supports_monitoring,
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
    /// The modified `VariableCharacteristicsType` instance
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
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the unit.
    ///
    /// # Returns
    ///
    /// The unit of the variable
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Sets the unit.
    ///
    /// # Arguments
    ///
    /// * `unit` - Unit of the variable
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_unit(&mut self, unit: String) -> &mut Self {
        self.unit = unit;
        self
    }

    /// Gets the data type.
    ///
    /// # Returns
    ///
    /// The data type of this variable
    pub fn data_type(&self) -> &DataEnumType {
        &self.data_type
    }

    /// Sets the data type.
    ///
    /// # Arguments
    ///
    /// * `data_type` - Data type of this variable
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_data_type(&mut self, data_type: DataEnumType) -> &mut Self {
        self.data_type = data_type;
        self
    }

    /// Gets the minimum limit.
    ///
    /// # Returns
    ///
    /// The minimum possible value of this variable
    pub fn min_limit(&self) -> &str {
        &self.min_limit
    }

    /// Sets the minimum limit.
    ///
    /// # Arguments
    ///
    /// * `min_limit` - Minimum possible value of this variable
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_min_limit(&mut self, min_limit: String) -> &mut Self {
        self.min_limit = min_limit;
        self
    }

    /// Gets the maximum limit.
    ///
    /// # Returns
    ///
    /// The maximum possible value of this variable
    pub fn max_limit(&self) -> &str {
        &self.max_limit
    }

    /// Sets the maximum limit.
    ///
    /// # Arguments
    ///
    /// * `max_limit` - Maximum possible value of this variable
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_max_limit(&mut self, max_limit: String) -> &mut Self {
        self.max_limit = max_limit;
        self
    }

    /// Gets the supports monitoring flag.
    ///
    /// # Returns
    ///
    /// When true, value from the Charging Station may not be set to null
    pub fn supports_monitoring(&self) -> bool {
        self.supports_monitoring
    }

    /// Sets the supports monitoring flag.
    ///
    /// # Arguments
    ///
    /// * `supports_monitoring` - When true, value from the Charging Station may not be set to null
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_supports_monitoring(&mut self, supports_monitoring: bool) -> &mut Self {
        self.supports_monitoring = supports_monitoring;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_characteristics_new() {
        let unit = "kWh".to_string();
        let data_type = DataEnumType::Decimal;
        let min_limit = "0".to_string();
        let max_limit = "100".to_string();
        let supports_monitoring = true;

        let characteristics = VariableCharacteristicsType::new(
            unit.clone(),
            data_type.clone(),
            min_limit.clone(),
            max_limit.clone(),
            supports_monitoring,
        );

        assert_eq!(characteristics.unit(), unit);
        assert_eq!(characteristics.data_type(), &data_type);
        assert_eq!(characteristics.min_limit(), min_limit);
        assert_eq!(characteristics.max_limit(), max_limit);
        assert_eq!(characteristics.supports_monitoring(), supports_monitoring);
        assert_eq!(characteristics.custom_data(), None);
    }

    #[test]
    fn test_variable_characteristics_with_methods() {
        let unit = "kWh".to_string();
        let data_type = DataEnumType::Decimal;
        let min_limit = "0".to_string();
        let max_limit = "100".to_string();
        let supports_monitoring = true;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let characteristics = VariableCharacteristicsType::new(
            unit.clone(),
            data_type.clone(),
            min_limit.clone(),
            max_limit.clone(),
            supports_monitoring,
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(characteristics.unit(), unit);
        assert_eq!(characteristics.data_type(), &data_type);
        assert_eq!(characteristics.min_limit(), min_limit);
        assert_eq!(characteristics.max_limit(), max_limit);
        assert_eq!(characteristics.supports_monitoring(), supports_monitoring);
        assert_eq!(characteristics.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_variable_characteristics_setters() {
        let unit1 = "kWh".to_string();
        let unit2 = "A".to_string();
        let data_type1 = DataEnumType::Decimal;
        let data_type2 = DataEnumType::Integer;
        let min_limit1 = "0".to_string();
        let min_limit2 = "-10".to_string();
        let max_limit1 = "100".to_string();
        let max_limit2 = "200".to_string();
        let supports_monitoring1 = true;
        let supports_monitoring2 = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut characteristics = VariableCharacteristicsType::new(
            unit1.clone(),
            data_type1.clone(),
            min_limit1.clone(),
            max_limit1.clone(),
            supports_monitoring1,
        );

        characteristics
            .set_unit(unit2.clone())
            .set_data_type(data_type2.clone())
            .set_min_limit(min_limit2.clone())
            .set_max_limit(max_limit2.clone())
            .set_supports_monitoring(supports_monitoring2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(characteristics.unit(), unit2);
        assert_eq!(characteristics.data_type(), &data_type2);
        assert_eq!(characteristics.min_limit(), min_limit2);
        assert_eq!(characteristics.max_limit(), max_limit2);
        assert_eq!(characteristics.supports_monitoring(), supports_monitoring2);
        assert_eq!(characteristics.custom_data(), Some(&custom_data));

        // Test clearing custom data
        characteristics.set_custom_data(None);
        assert_eq!(characteristics.custom_data(), None);
    }
}
