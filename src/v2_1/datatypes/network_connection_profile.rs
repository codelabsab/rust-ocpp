use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{apn::APNType, custom_data::CustomDataType, vpn::VPNType};
use crate::v2_1::enumerations::{
    OCPPInterfaceEnumType, OCPPTransportEnumType, OCPPVersionEnumType,
};

/// The NetworkConnectionProfile defines the functional and technical parameters of a communication link.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConnectionProfileType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. APN configuration, when using GSM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn: Option<APNType>,

    /// Required. VPN configuration, when using VPN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VPNType>,

    /// Required. Communication interface used by OCPP.
    pub ocpp_interface: OCPPInterfaceEnumType,

    /// Required. Transport protocol used by OCPP.
    pub ocpp_transport: OCPPTransportEnumType,

    /// Required. OCPP version used.
    pub ocpp_version: OCPPVersionEnumType,

    /// Required. URL of the CSMS that this Charging Station should connect to.
    #[validate(length(max = 512))]
    pub csms_url: String,

    /// Required. The security profile used when connecting to the CSMS with this NetworkConnectionProfile.
    #[validate(range(min = 0, max = 65535))]
    pub message_timeout: i32,

    /// Required. The security profile used when connecting to the CSMS with this NetworkConnectionProfile.
    #[validate(range(min = 0, max = 65535))]
    pub security_profile: i32,

    /// Required. Duration in seconds before a message send by the Charging Station via this network connection times-out.
    /// The best setting depends on the underlying network connection and the OCPP operations used.
    #[validate(range(min = 1, max = 65535))]
    pub ocpp_csms_port: i32,
}
