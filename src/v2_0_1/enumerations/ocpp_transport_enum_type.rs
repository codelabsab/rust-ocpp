#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum OCPPTransportEnumType {
    #[default]
    JSON,
    SOAP,
}
