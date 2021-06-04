# Security

This Functional Block describes the security requirements for the OCPP protocol. The security part was developed to strengthen
and mature the future development and standardization of OCPP. It is based amongst others on the end-to-end security design by
LaQuSo. Security requirements are included on security measures at Charging Station and CSMS, to support users of the
OCPP.

## Use cases & Requirements

| No. | Description | Implemented |
|---  |---          |---          |
| A01 | Update Charging Station Password for HTTP Basic Authentication | o |
| A02 | Update Charging Station Certificate by request of CSMS | o |
| A03 | Update Charging Station Certificate initiated by the Charging Station | o |
| A04 | Security Event Notification | o |
| A05 | Upgrade Charging Station Security Profile | o |

## Requirements

The CSMS SHALL support at least the following four cipher suites:

- TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
- TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
- TLS_RSA_WITH_AES_128_GCM_SHA256
- TLS_RSA_WITH_AES_256_GCM_SHA384

## Profiles

This section defines the different OCPP security profiles and their requirement. OCPP 2.0.1 supports three security profiles:
The table below shows which security measures are used by which profile.

Table 11. Overview of OCPP security profiles

| Profile | Charging Station Authentication | CSMS  Authentication | Communication Security |
| ---     | ---                             | ---                  | ---                    |
| 1. Unsecured Transport with Basic Authentication | HTTP Basic Authentication | - | - |
| 2. TLS with Basic Authentication | HTTP Basic Authentication | TLS authentication using certificate | Transport Layer Security (TLS) |
| 3. TLS with Client Side Certificates | TLS authentication using certificate | TLS authentication using certificate | Transport Layer Security (TLS) |
