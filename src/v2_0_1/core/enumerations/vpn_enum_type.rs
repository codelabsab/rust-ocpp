#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum VPNEnumType {
    IKEv2,
    IPSec,
    L2TP,
    PPTP,
}
