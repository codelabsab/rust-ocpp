use validator::Validate;
/// DCChargingParametersType is used by: Common:ChargingNeedsType
// TODO: Does bulk_soc and full_soc really rename correctly?
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct DCChargingParametersType {
    pub ev_max_current: i64,
    pub ev_max_voltage: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_power: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub state_of_charge: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_capacity: Option<i64>,
    #[serde(rename = "fullSoC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub full_soc: Option<u8>,
    #[serde(rename = "bulkSoC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub bulk_soc: Option<u8>,
}
