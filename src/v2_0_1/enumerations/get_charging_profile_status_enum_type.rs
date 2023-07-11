#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum GetChargingProfileStatusEnumType {
    #[default]
    Accepted,
    NoProfiles,
}
