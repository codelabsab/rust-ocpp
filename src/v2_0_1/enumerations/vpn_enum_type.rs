#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum VPNEnumType {
    #[default]
    IKEv2,
    IPSec,
    L2TP,
    PPTP,
}
