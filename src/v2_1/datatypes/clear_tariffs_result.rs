use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, status_info::StatusInfoType};

/// Status of operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TariffClearStatusEnumType {
    Accepted,
    Rejected,
    NoTariff,
}

/// Result of clearing a tariff.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct ClearTariffsResultType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 60))]
    /// Id of tariff for which _status_ is reported. If no tariffs were found, then this field is absent, and _status_ will be `NoTariff`.
    pub tariff_id: Option<String>,

    /// Status indicating whether the tariff was cleared.
    pub status: TariffClearStatusEnumType,
}
