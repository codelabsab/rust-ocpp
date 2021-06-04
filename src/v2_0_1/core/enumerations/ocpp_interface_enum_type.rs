#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum OCPPInterfaceEnumType {
    Wired0,
    Wired1,
    Wired2,
    Wired3,
    Wireless0,
    Wireless1,
    Wireless2,
    Wireless3,
}
