#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum OCPPTransportEnumType {
    JSON,
    SOAP,
}
