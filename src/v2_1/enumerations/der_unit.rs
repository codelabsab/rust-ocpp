use serde::{Deserialize, Serialize};

/// Unit of the setpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DERUnitEnumType {
    #[serde(rename = "Not_Applicable")]
    NotApplicable,
    #[serde(rename = "PctMaxW")]
    PctMaxW,
    #[serde(rename = "PctMaxVar")]
    PctMaxVar,
    #[serde(rename = "PctWAvail")]
    PctWAvail,
    #[serde(rename = "PctVarAvail")]
    PctVarAvail,
    #[serde(rename = "PctEffectiveV")]
    PctEffectiveV,
}

impl Default for DERUnitEnumType {
    fn default() -> Self {
        Self::NotApplicable
    }
}
