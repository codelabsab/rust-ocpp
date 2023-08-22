/// Represent a signed version of the meter value.
/// SignedMeterValueType is used by: Common:SampledValueType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType<'a> {
    pub signed_meter_data: &'a str,
    pub signing_method: &'a str,
    pub encoding_method: &'a str,
    pub public_key: &'a str,
}
