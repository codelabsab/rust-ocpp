#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
pub enum EnergyTransferModeEnumType {
    DC,
    #[serde(rename = "AC_single_phase")]
    ACSinglePhase,
    #[serde(rename = "AC_two_phase")]
    ACTwoPhase,
    #[serde(rename = "AC_three_phase")]
    ACThreePhase,
}
