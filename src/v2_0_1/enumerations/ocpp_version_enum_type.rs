#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum OCPPVersionEnumType {
    OCPP12,
    OCPP15,
    OCPP16,
    #[default]
    OCPP20,
}
