#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum LogEnumType {
    DiagnosticsLog,
    SecurityLog,
}
