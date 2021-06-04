/*
    This Functional Block describes all the functionalities that help a CSO provision their Charging Stations, allowing them on their
    network and retrieving configuration information from these Charging Stations. Additionally, it consists of the ability to retrieve
    information about the configuration of Charging Stations, make changes to the configuration etc. This chapter also covers resetting
    a Charging Station and migrating to a new NetworkConnectionProfile.
*/

pub mod ocpp_ws;
pub mod service_tests;
pub mod unit_tests;
