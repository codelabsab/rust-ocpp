# OCPP Library

This library implements ocpp and is structured in the Functional Block that is described in the specification.

# Architecture

The 3-tier arch 

![3-tier](docs/3-tier.png "Logo Title Text 1")


# Functional blocks

| Clause | Functional Block Title | Description |
| --- | --- | --- |
|  A  | Security | This Functional Block describes a security specification for the OCPP protocol | 
|  B  | Provisioning | This Functional Block describes all the functionalities that help a CSO provision their Charging Stations, allowing them to be registered and accepted on their network and retrieving basic configuration information from these Charging Stations |
|  C  | Authorization | This Functional Block describes all the authorization related functionality: AuthorizeRequest message handling/behavior and Authorization Cache functionality |
|  D  | Local Authorization List Management | This Functional Block describes functionality for managing the Local Authorization List |
|  E  | Transactions | This Functional Block describes the basic OCPP Transaction related functionality for transactions that are started/stopped on the Charging Station |
|  F  | Remote Control | This Functional Block describes three types of use cases for remote control management from the CSMS: Remote Transaction Control, Unlocking a Connector and Remote Trigger |
|  G  | Availability | This functional Block describes the functionality of sending status notification messages |
|  H  | Reservation | This Functional Block describes the reservation functionality of a Charging Station |
|  I  | Tariff and Cost | This Functional Block provides tariff and cost information to an EV Driver, when a Charging Station is capable of showing this on a display. Before a driver starts charging tariff information needs to be given, detailed prices for all the components that make up the tariff plan applicable to this driver at this Charging Station. During charging the EV Driver needs to be shown the running total cost, updated at a regular, fitting interval. When the EV Driver stops charging the total cost of this transaction needs to be shown |
|  J  | Metering | This Functional Block describes the functionality for sending meter values, on a periodic sampling and/or clock-aligned timing basis |
|  K  | Smart Charging | This Functional Block describes all the functionality that enables the CSO (or indirectly a third party) to influence the charging current/power of a charging session, or set limits to the amount of power/current a Charging Station can offer to an EV |
|  L  | Firmware Management | This Functional Block describes the functionality that enables a CSO to update the firmware of a Charging Station |
|  M  | ISO 15118 Certificate Management | This Functional Block provides the installation and update of ISO 15118 certificates |
|  N  | Diagnostics | This Functional Block describes the functionality that enables a CSO to request and track the upload of a diagnostics file from a Charging Station, and to manage the monitoring of Charging Station data |
|  O  | Display Message | With the DisplayMessage feature OCPP enables a CSO to display a message on a Charging Station, that is not part of the firmware of the Charging Station. The CSO gets control over these messages: the CSO can set, retrieve (get), replace and clear messages |
|  P  | Data Transfer | This Functional Block describes the functionality that enables a party to add custom commands to OCPP, enabling custom extension to OCPP |

# Functional Blocks and use cases

| UC ID | Functional Block | Use case name | Basic Implementation | Implemented |
|  ---  |       ---        |      ---      |          ---         |     ---     |
|  A01  | Security                         | Update Charging Station Password for HTTP Basic Authentication | | o |
|  A02  |                                  | Update Charging Station Certificate by request of CSMS | | o |
|  A03  |                                  | Update Charging Station Certificate initiated by the Charging Station | | o |
|  A04  |                                  | Security Event Notification | | o |
|  B01  | Provisioning                     | Cold Boot Charging Station | x | o |
|  B02  |                                  | Cold Boot Charging Station - Pending | x | o |
|  B03  |                                  | Cold Boot Charging Station - Rejected | x | o |
|  B04  |                                  | Offline Behavior Idle Charging Station | x | o |
|  B05  |                                  | Set Variables | x | o |
|  B06  |                                  | Get Variables | x | o |
|  B07  |                                  | Get Base Report | x | o |
|  B08  |                                  | Get Custom Report | | o |
|  B09  |                                  | Setting a new NetworkConnectionProfile | | o |
|  B10  |                                  | Migrate to new CSMS | | o |
|  B11  |                                  | Reset - Without Ongoing Transaction | x | o |
|  B12  |                                  | Reset - With Ongoing Transaction | x | o |
|  C01  | Authorization                    | EV Driver Authorization using RFID | x | o |
|  C02  |                                  | Authorization using a start button | x | o |
|  C03  |                                  | Authorization using credit/debit card | | o |
|  C04  |                                  | Authorization using PIN-code | x | o |
|  C05  |                                  | Authorization for CSMS initiated transactions | | o |
|  C06  |                                  | Authorization using local id type | | o |
|  C07  |                                  | Authorization using Contract Certificates | | o |
|  C08  |                                  | Authorization at EVSE using ISO 15118 External o Identification Means (EIM) | | o |
|  C09  |                                  | Authorization by GroupId | | o |
|  C10  |                                  | Store Authorization Data in the Authorization o Cache | | o |
|  C11  |                                  | Clear Authorization Data in Authorization Cache | | o |
|  C12  |                                  | Start Transaction - Cached Id | | o |
|  C13  |                                  | Offline Authorization through Local Authorization List | | o |
|  C14  |                                  | Online Authorization through Local Authorization List | | o |
|  C15  |                                  | Offline Authorization of unknown Id | | o |
|  C16  |                                  | Stop Transaction with a Master Pass | | o |
|  D01  | LocalAuthorizationList           | Send Local Authorization List | |o |
|  D02  |                                  | Get Local List Version | | o |
|  E01  | Transactions                     | Start Transaction Options | x | o |
|  E02  |                                  | Start Transaction - Cable Plugin First | x | o |
|  E03  |                                  | Start Transaction - IdToken First | x | o |
|  E04  |                                  | Transaction started while Charging Station is offline | | o |
|  E05  |                                  | Start Transaction - Id not Accepted | x | o |
|  E06  |                                  | Stop Transaction Options | x | o |
|  E07  |                                  | Transaction locally stopped by IdToken | x | o |
|  E08  |                                  | Transaction stopped while Charging Station is offline | x | o |
|  E09  |                                  | When cable disconnected on EV-side: Stop Transaction | x | o |
|  E10  |                                  | When cable disconnected on EV-side: Suspend Transaction | x | o |
|  E11  |                                  | Connection Loss During Transaction | x | o |
|  E12  |                                  | Inform CSMS of an Offline Occurred Transaction | x | o |
|  E13  |                                  | Transaction related message not accepted by CSMS | x | o |
|  E14  |                                  | Check transaction status | | o |
|  E15  |                                  | End of charging process | | o |
|  F01  | RemoteControl                    | Remote Start Transaction - Cable Plugin First | | o |
|  F02  |                                  | Remote Start Transaction - Remote Start First | | o |
|  F03  |                                  | Remote Stop Transaction | | o |
|  F04  |                                  | Remote Stop ISO 15118 charging from CSMS | | o |
|  F05  |                                  | Remotely Unlock Connector | | o |
|  F06  |                                  | Trigger Message | | o |
|  G01  | Availability                     | Status Notification | x | o |
|  G02  |                                  | Heartbeat | | o |
|  G03  |                                  | Change Availability EVSE | x | o |
|  G04  |                                  | Change Availability Charging Station | x | o |
|  G05  |                                  | Lock Failure | x | o |
|  H01  | Reservation                      | Reservation | | o |
|  H02  |                                  | Cancel Reservation | | o |
|  H03  |                                  | Use a reserved EVSE | | o |
|  H04  |                                  | Reservation Ended, not used | | o |
|  I01  | Tariff and Costs                 | Show EV Driver-specific tariff information | | o |
|  I02  |                                  | Show EV Driver running total cost during charging | | o |
|  I03  |                                  | Show EV Driver final total cost after charging | | o |
|  I04  |                                  | Show fallback tariff information | | o |
|  I05  |                                  | Show fallback total cost message | | o |
|  I06  |                                  | Update Tariff Information During Transaction | | o |
|  J01  | Metering                         | Sending Meter Values not related to a transaction | | o |
|  J02  |                                  | Sending transaction related Meter Values | x | o |
|  J03  |                                  | Charging Loop with metering information exchange | | o |
|  K01  | SmartCharging                     | SetChargingProfile | | o |
|  K02  |                                  | Central Smart Charging | | o |
|  K03  |                                  | Local Smart Charging | | o |
|  K04  |                                  | Internal Load Balancing | | o |
|  K05  |                                  | Remote Start Transaction with Charging Profile | | o |
|  K06  |                                  | Offline Behavior Smart Charging During Transaction | | o |
|  K07  |                                  | Offline Behavior Smart Charging at Start of Transaction | | o |
|  K08  |                                  | Get Composite Schedule | | o |
|  K09  |                                  | Get Charging Profiles | | o |
|  K10  |                                  | Clear Charging Profile | | o |
|  K11  |                                  | Set / Update External Charging Limit With Ongoing Transaction | | o |
|  K12  |                                  | Set / Update External Charging Limit Without Ongoing Transaction | | o |
|  K13  |                                  | Reset / release external charging limit | | o |
|  K14  |                                  | External Charging Limit with Local Controller | | o |
|  K15  |                                  | Charging with load leveling based on High Level Communication | | o |
|  K16  |                                  | Optimized charging with scheduling to the CSMS | | o |
|  K17  |                                  | Renegotiating a Charging Schedule | | o |
|  L01  | Firmware Management              | Secure Firmware Update | | o |
|  L02  |                                  | Non-Secure Firmware Update | | o |
|  L03  |                                  | Publish Firmware file on Local Controller | | o |
|  L04  |                                  | Unpublish Firmware file on Local Controller | | o |
|  M01  | ISO 15118 Certificate Management | Certificate Installation EV | | o |
|  M02  |                                  | Certificate Update EV | | o |
|  M03  |                                  | Retrieve list of available certificates from a Charging Station | | o |
|  M04  |                                  | Delete a specific certificate from a Charging Station | | o |
|  M05  |                                  | Install CA certificate in a Charging Station | | o |
|  M06  |                                  | Get Charging Station Certificate status | | o |
|  N01  | Diagnostics                      | Retrieve Log Information | | o |
|  N02  |                                  | Get Monitoring report | | o |
|  N03  |                                  | et Monitoring Base | | o |
|  N04  |                                  | Set Variable Monitoring | | o |
|  N05  |                                  | Set Monitoring Level | | o |
|  N06  |                                  | Clear / Remove Monitoring | | o |
|  N07  |                                  | Alert Event | x | o |
|  N08  |                                  | Periodic Event | | o |
|  N09  |                                  | Get Customer Information | | o |
|  N10  |                                  | Clear Customer Information | | o |
|  O01  | Display Message                  | Set DisplayMessage | | o |
|  O02  |                                  | Set DisplayMessage for Transaction | | o |
|  O03  |                                  | Get All DisplayMessages | | o |
|  O04  |                                  | Get Specific DisplayMessages | | o |
|  O05  |                                  | Clear a DisplayMessage | | o |
|  O06  |                                  | Replace DisplayMessage | | o |
|  P01  | DataTransfer                     | Data Transfer to the Charging Station | x | o |
|  P02  |                                  | Data Transfer to the CSMS | x | o |

# Tests

`cargo run`

In another window run `websocat ws://127.0.0.1:3040/ws`

Paste a BootNotificationRequest:

```json
[2,"19223201","BootNotification",{"reason": "PowerUp","chargingStation": {"model": "SingleSocketCharger","vendorName": "VendorX"}}]
```