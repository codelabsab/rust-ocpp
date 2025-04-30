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

impl NetworkConnectionProfileType {
    /// Creates a new `NetworkConnectionProfileType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `ocpp_interface` - Communication interface used by OCPP
    /// * `ocpp_transport` - Transport protocol used by OCPP
    /// * `ocpp_version` - OCPP version used
    /// * `csms_url` - URL of the CSMS that this Charging Station should connect to
    /// * `message_timeout` - Duration in seconds before a message times-out
    /// * `security_profile` - The security profile used when connecting to the CSMS
    /// * `ocpp_csms_port` - OCPP CSMS port
    ///
    /// # Returns
    ///
    /// A new instance of `NetworkConnectionProfileType` with optional fields set to `None`
    pub fn new(
        ocpp_interface: OCPPInterfaceEnumType,
        ocpp_transport: OCPPTransportEnumType,
        ocpp_version: OCPPVersionEnumType,
        csms_url: String,
        message_timeout: i32,
        security_profile: i32,
        ocpp_csms_port: i32,
    ) -> Self {
        Self {
            custom_data: None,
            apn: None,
            vpn: None,
            ocpp_interface,
            ocpp_transport,
            ocpp_version,
            csms_url,
            message_timeout,
            security_profile,
            ocpp_csms_port,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this network connection profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the APN configuration.
    ///
    /// # Arguments
    ///
    /// * `apn` - APN configuration, when using GSM
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_apn(mut self, apn: APNType) -> Self {
        self.apn = Some(apn);
        self
    }

    /// Sets the VPN configuration.
    ///
    /// # Arguments
    ///
    /// * `vpn` - VPN configuration, when using VPN
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_vpn(mut self, vpn: VPNType) -> Self {
        self.vpn = Some(vpn);
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this network connection profile, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the APN configuration.
    ///
    /// # Returns
    ///
    /// An optional reference to the APN configuration
    pub fn apn(&self) -> Option<&APNType> {
        self.apn.as_ref()
    }

    /// Sets the APN configuration.
    ///
    /// # Arguments
    ///
    /// * `apn` - APN configuration, when using GSM, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_apn(&mut self, apn: Option<APNType>) -> &mut Self {
        self.apn = apn;
        self
    }

    /// Gets the VPN configuration.
    ///
    /// # Returns
    ///
    /// An optional reference to the VPN configuration
    pub fn vpn(&self) -> Option<&VPNType> {
        self.vpn.as_ref()
    }

    /// Sets the VPN configuration.
    ///
    /// # Arguments
    ///
    /// * `vpn` - VPN configuration, when using VPN, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_vpn(&mut self, vpn: Option<VPNType>) -> &mut Self {
        self.vpn = vpn;
        self
    }

    /// Gets the OCPP interface.
    ///
    /// # Returns
    ///
    /// The communication interface used by OCPP
    pub fn ocpp_interface(&self) -> OCPPInterfaceEnumType {
        self.ocpp_interface.clone()
    }

    /// Sets the OCPP interface.
    ///
    /// # Arguments
    ///
    /// * `ocpp_interface` - Communication interface used by OCPP
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ocpp_interface(&mut self, ocpp_interface: OCPPInterfaceEnumType) -> &mut Self {
        self.ocpp_interface = ocpp_interface;
        self
    }

    /// Gets the OCPP transport.
    ///
    /// # Returns
    ///
    /// The transport protocol used by OCPP
    pub fn ocpp_transport(&self) -> OCPPTransportEnumType {
        self.ocpp_transport.clone()
    }

    /// Sets the OCPP transport.
    ///
    /// # Arguments
    ///
    /// * `ocpp_transport` - Transport protocol used by OCPP
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ocpp_transport(&mut self, ocpp_transport: OCPPTransportEnumType) -> &mut Self {
        self.ocpp_transport = ocpp_transport;
        self
    }

    /// Gets the OCPP version.
    ///
    /// # Returns
    ///
    /// The OCPP version used
    pub fn ocpp_version(&self) -> OCPPVersionEnumType {
        self.ocpp_version.clone()
    }

    /// Sets the OCPP version.
    ///
    /// # Arguments
    ///
    /// * `ocpp_version` - OCPP version used
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ocpp_version(&mut self, ocpp_version: OCPPVersionEnumType) -> &mut Self {
        self.ocpp_version = ocpp_version;
        self
    }

    /// Gets the CSMS URL.
    ///
    /// # Returns
    ///
    /// The URL of the CSMS that this Charging Station should connect to
    pub fn csms_url(&self) -> &str {
        &self.csms_url
    }

    /// Sets the CSMS URL.
    ///
    /// # Arguments
    ///
    /// * `csms_url` - URL of the CSMS that this Charging Station should connect to
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_csms_url(&mut self, csms_url: String) -> &mut Self {
        self.csms_url = csms_url;
        self
    }

    /// Gets the message timeout.
    ///
    /// # Returns
    ///
    /// Duration in seconds before a message times-out
    pub fn message_timeout(&self) -> i32 {
        self.message_timeout
    }

    /// Sets the message timeout.
    ///
    /// # Arguments
    ///
    /// * `message_timeout` - Duration in seconds before a message times-out
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_message_timeout(&mut self, message_timeout: i32) -> &mut Self {
        self.message_timeout = message_timeout;
        self
    }

    /// Gets the security profile.
    ///
    /// # Returns
    ///
    /// The security profile used when connecting to the CSMS
    pub fn security_profile(&self) -> i32 {
        self.security_profile
    }

    /// Sets the security profile.
    ///
    /// # Arguments
    ///
    /// * `security_profile` - The security profile used when connecting to the CSMS
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_security_profile(&mut self, security_profile: i32) -> &mut Self {
        self.security_profile = security_profile;
        self
    }

    /// Gets the OCPP CSMS port.
    ///
    /// # Returns
    ///
    /// The OCPP CSMS port
    pub fn ocpp_csms_port(&self) -> i32 {
        self.ocpp_csms_port
    }

    /// Sets the OCPP CSMS port.
    ///
    /// # Arguments
    ///
    /// * `ocpp_csms_port` - OCPP CSMS port
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ocpp_csms_port(&mut self, ocpp_csms_port: i32) -> &mut Self {
        self.ocpp_csms_port = ocpp_csms_port;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::APNAuthenticationEnumType;

    #[test]
    fn test_new_network_connection_profile() {
        let ocpp_interface = OCPPInterfaceEnumType::Wired0;
        let ocpp_transport = OCPPTransportEnumType::JSON;
        let ocpp_version = OCPPVersionEnumType::OCPP20;
        let csms_url = "https://example.com/csms".to_string();
        let message_timeout = 30;
        let security_profile = 1;
        let ocpp_csms_port = 443;

        let profile = NetworkConnectionProfileType::new(
            ocpp_interface.clone(),
            ocpp_transport.clone(),
            ocpp_version.clone(),
            csms_url.clone(),
            message_timeout,
            security_profile,
            ocpp_csms_port,
        );

        assert_eq!(profile.ocpp_interface(), ocpp_interface);
        assert_eq!(profile.ocpp_transport(), ocpp_transport);
        assert_eq!(profile.ocpp_version(), ocpp_version);
        assert_eq!(profile.csms_url(), csms_url);
        assert_eq!(profile.message_timeout(), message_timeout);
        assert_eq!(profile.security_profile(), security_profile);
        assert_eq!(profile.ocpp_csms_port(), ocpp_csms_port);
        assert_eq!(profile.custom_data(), None);
        assert_eq!(profile.apn(), None);
        assert_eq!(profile.vpn(), None);
    }

    #[test]
    fn test_with_optional_fields() {
        let ocpp_interface = OCPPInterfaceEnumType::Wired0;
        let ocpp_transport = OCPPTransportEnumType::JSON;
        let ocpp_version = OCPPVersionEnumType::OCPP20;
        let csms_url = "https://example.com/csms".to_string();
        let message_timeout = 30;
        let security_profile = 1;
        let ocpp_csms_port = 443;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };
        let apn = APNType {
            apn: "internet".to_string(),
            apn_user_name: Some("user".to_string()),
            apn_password: Some("password".to_string()),
            sim_pin: Some(1234),
            preferred_network: Some("network".to_string()),
            use_only_preferred_network: Some(true),
            apn_authentication: APNAuthenticationEnumType::CHAP,
            custom_data: None,
        };
        let vpn = VPNType {
            server: "vpn.example.com".to_string(),
            user: "vpnuser".to_string(),
            password: "vpnpass".to_string(),
            key: "vpnkey".to_string(),
            r#type: "OpenVPN".to_string(),
            custom_data: None,
        };

        let profile = NetworkConnectionProfileType::new(
            ocpp_interface.clone(),
            ocpp_transport.clone(),
            ocpp_version.clone(),
            csms_url.clone(),
            message_timeout,
            security_profile,
            ocpp_csms_port,
        )
        .with_custom_data(custom_data.clone())
        .with_apn(apn.clone())
        .with_vpn(vpn.clone());

        assert_eq!(profile.ocpp_interface(), ocpp_interface);
        assert_eq!(profile.ocpp_transport(), ocpp_transport);
        assert_eq!(profile.ocpp_version(), ocpp_version);
        assert_eq!(profile.csms_url(), csms_url);
        assert_eq!(profile.message_timeout(), message_timeout);
        assert_eq!(profile.security_profile(), security_profile);
        assert_eq!(profile.ocpp_csms_port(), ocpp_csms_port);
        assert_eq!(profile.custom_data(), Some(&custom_data));
        assert_eq!(profile.apn(), Some(&apn));
        assert_eq!(profile.vpn(), Some(&vpn));
    }

    #[test]
    fn test_setter_methods() {
        let ocpp_interface1 = OCPPInterfaceEnumType::Wired0;
        let ocpp_interface2 = OCPPInterfaceEnumType::Wired1;
        let ocpp_transport1 = OCPPTransportEnumType::JSON;
        let ocpp_transport2 = OCPPTransportEnumType::SOAP;
        let ocpp_version1 = OCPPVersionEnumType::OCPP20;
        let ocpp_version2 = OCPPVersionEnumType::OCPP16;
        let csms_url1 = "https://example.com/csms".to_string();
        let csms_url2 = "https://example.org/csms".to_string();
        let message_timeout1 = 30;
        let message_timeout2 = 60;
        let security_profile1 = 1;
        let security_profile2 = 2;
        let ocpp_csms_port1 = 443;
        let ocpp_csms_port2 = 8080;

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let apn = APNType {
            apn: "internet".to_string(),
            apn_user_name: Some("user".to_string()),
            apn_password: Some("password".to_string()),
            sim_pin: Some(1234),
            preferred_network: Some("network".to_string()),
            use_only_preferred_network: Some(true),
            apn_authentication: APNAuthenticationEnumType::CHAP,
            custom_data: None,
        };

        let vpn = VPNType {
            server: "vpn.example.com".to_string(),
            user: "vpnuser".to_string(),
            password: "vpnpass".to_string(),
            key: "vpnkey".to_string(),
            r#type: "OpenVPN".to_string(),
            custom_data: None,
        };

        let mut profile = NetworkConnectionProfileType::new(
            ocpp_interface1,
            ocpp_transport1,
            ocpp_version1,
            csms_url1,
            message_timeout1,
            security_profile1,
            ocpp_csms_port1,
        );

        profile
            .set_ocpp_interface(ocpp_interface2.clone())
            .set_ocpp_transport(ocpp_transport2.clone())
            .set_ocpp_version(ocpp_version2.clone())
            .set_csms_url(csms_url2.clone())
            .set_message_timeout(message_timeout2)
            .set_security_profile(security_profile2)
            .set_ocpp_csms_port(ocpp_csms_port2)
            .set_custom_data(Some(custom_data.clone()))
            .set_apn(Some(apn.clone()))
            .set_vpn(Some(vpn.clone()));

        assert_eq!(profile.ocpp_interface(), ocpp_interface2);
        assert_eq!(profile.ocpp_transport(), ocpp_transport2);
        assert_eq!(profile.ocpp_version(), ocpp_version2);
        assert_eq!(profile.csms_url(), csms_url2);
        assert_eq!(profile.message_timeout(), message_timeout2);
        assert_eq!(profile.security_profile(), security_profile2);
        assert_eq!(profile.ocpp_csms_port(), ocpp_csms_port2);
        assert_eq!(profile.custom_data(), Some(&custom_data));
        assert_eq!(profile.apn(), Some(&apn));
        assert_eq!(profile.vpn(), Some(&vpn));

        // Test clearing optional fields
        profile
            .set_custom_data(None)
            .set_apn(None)
            .set_vpn(None);

        assert_eq!(profile.custom_data(), None);
        assert_eq!(profile.apn(), None);
        assert_eq!(profile.vpn(), None);
    }
}
