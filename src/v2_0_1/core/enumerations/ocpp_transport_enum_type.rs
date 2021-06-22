#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum OCPPTransportEnumType {
    JSON,
    SOAP,
}
