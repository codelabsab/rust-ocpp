#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum APNAuthenticationEnumType {
    CHAP,
    NONE,
    PAP,
    AUTO,
}
