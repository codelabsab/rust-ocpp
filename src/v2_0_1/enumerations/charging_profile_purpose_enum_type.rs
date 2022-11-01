#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ChargingProfilePurposeEnumType {
    ChargingStationExternalConstraints,
    ChargingStationMaxProfile,
    #[default]
    TxDefaultProfile,
    TxProfile,
}
