#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, Default)]
pub enum EnergyTransferModeEnumType {
    DC,
    #[serde(rename = "AC_single_phase")]
    #[default]
    ACSinglePhase,
    #[serde(rename = "AC_two_phase")]
    ACTwoPhase,
    #[serde(rename = "AC_three_phase")]
    ACThreePhase,
}
