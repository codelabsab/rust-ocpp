#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum IslandingDetectionEnumType {
    #[default]
    NoAntiIslandingSupport,
    RoCoF,
    UVP_OVP,
    UFP_OFP,
    VoltageVectorShift,
    ZeroCrossingDetection,
    OtherPassive,
    ImpedanceMeasurement,
    ImpedanceAtFrequency,
    SlipModeFrequencyShift,
    SandiaFrequencyShift,
    SandiaVoltageShift,
    FrequencyJump,
    RCLQFactor,
    OtherActive,
}
