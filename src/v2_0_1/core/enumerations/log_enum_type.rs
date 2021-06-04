#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum LogEnumType {
    DiagnosticsLog,
    SecurityLog,
}
