#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum IdTokenEnumType {
    #[default]
    Central,
    #[serde(rename = "eMAID")]
    EMAID,
    ISO14443,
    ISO15693,
    KeyCode,
    Local,
    MacAddress,
    NoAuthorization,
}
