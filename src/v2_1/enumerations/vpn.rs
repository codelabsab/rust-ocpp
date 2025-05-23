use serde::{Deserialize, Serialize};

/// Type of VPN protocol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VPNEnumType {
    /// Internet Key Exchange version 2 protocol.
    #[serde(rename = "IKEv2")]
    IKEv2,

    /// Internet Protocol Security protocol.
    #[serde(rename = "IPSec")]
    IPSec,

    /// Layer 2 Tunneling Protocol.
    #[serde(rename = "L2TP")]
    L2TP,

    /// Point-to-Point Tunneling Protocol.
    #[serde(rename = "PPTP")]
    PPTP,
}
