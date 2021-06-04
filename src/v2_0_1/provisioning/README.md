# 1. Introduction

This Functional Block describes all the functionalities that help a CSO provision their Charging Stations, allowing them on their network and retrieving configuration information from these Charging Stations. 

Additionally, it consists of the ability to retrieve information about the configuration of Charging Stations, make changes to the configuration etc. This chapter also covers resetting a Charging Station and migrating to a new NetworkConnectionProfile.

# 1.1. Transactions before being accepted by a CSMS
A Charging Station Operator MAY choose to configure a Charging Station to accept transactions before the Charging Station is accepted by a CSMS. Parties who want to implement this such behavior should realize that it is uncertain if those transactions can ever be delivered to the CSMS.

After a restart (for instance due to a remote reset command, power outage, firmware update, software error etc.) the Charging Station MUST again contact the CSMS and SHALL send a BootNotification request. If the Charging Station fails to receive a BootNotificationResponse from the CSMS, and has no in-built non-volatile real-time clock hardware that has been correctly preset, the Charging Station may not have a valid date and time setting, making it difficult or even impossible to later determine the date and time of transactions.

It might also be the case (e.g. due to configuration error) that the CSMS indicates a status other than Accepted for an extended period of time, or indefinitely.

It is usually advisable to deny all charging services at a Charging Station if the Charging Station has never before been Accepted by the CSMS (using the current connection settings, URL, etc.) since users cannot be authenticated and running transactions could conflict with provisioning processes.

If this is supported, this behaviour can be configured via the Configuration Variable: TxBeforeAcceptedEnabled.