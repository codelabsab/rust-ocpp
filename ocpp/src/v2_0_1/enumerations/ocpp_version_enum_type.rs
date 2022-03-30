#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum OCPPVersionEnumType {
    OCPP12,
    OCPP15,
    OCPP16,
    OCPP20,
}
