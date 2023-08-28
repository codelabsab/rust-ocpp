//! # rust-ocpp
//!
//! `rust-ocpp` is a collection of data types and enums specified by the Open Charge Point Protocol
//! by Open Charge Alliance
//!
//! More Info on Open Charge Alliance can be found here: [Open Charge Alliance](https://www.openchargealliance.org/)
//!
//! It aims to implement the `ocpp 1.6` and `ocpp 2.0.1` protocols.
//!
//! # v1_6
//! The [v1_6](v1_6) module contains the `ocpp 1.6` implementation
//!
//! # v2_0_1
//! The [v2_0_1](v2_0_1) module contains the `ocpp 2.0.1` implementation

#![cfg_attr(not(feature = "std"), no_std)]

#![feature(generic_arg_infer)]

/// tests
#[cfg(all(test, feature = "std"))]
pub mod tests;
#[cfg(feature = "v1_6")]
pub mod v1_6;
#[cfg(feature = "v2_0_1")]
pub mod v2_0_1;

#[cfg(feature="std")]
#[allow(type_alias_bounds)]
pub type Vec<T: Sized, const N: usize> = std::vec::Vec<T>;

#[cfg(feature="no_std")]
pub use heapless::Vec;

/// defaults
/// TODO: Adjust to match real-life usage
/// TODO: These constants are used as size defaults on generic structs,
/// TODO: but those structs can not be constructed without
/// TODO: size arguments passed in. (compiler bug?)
pub const N_VARIABLES: usize = 20;
pub const N_CHARGING_SCHEDULE_PERIODS: usize = 20;
pub const N_KEYS: usize = 20;
pub const N_CONF_KEYS: usize = 20;
pub const N_UNKNOWN_KEYS: usize = 20;
pub const N_METER_VALUES: usize = 20;
pub const N_SAMPLED_VALUES_PER_METER: usize = 20;
pub const CHARGING_SCHEDULE_SIZE: usize = 20;
pub const AUTH_LIST_SIZE: usize = 100;
pub const N_TXN_DATA: usize = 20;
pub const N_METER_VALUES_PER_TXN: usize = 20;
pub const N_SAMPLED_VALUES: usize = 20;
pub const N_EVSE_IDS: usize = 100;
pub const TOKEN_N_ADDITIONAL_INFOS: usize = 20;
pub const ID_TOKEN_TYPE_N_ADDITIONAL_INFOS: usize = 20;
pub const N_CERTIFICATE_HASHES: usize = 20;
pub const N_CHARGING_PROFILE_IDS: usize = 20;
pub const N_CHARGING_LIMIT_SOURCES: usize = 20;
pub const N_SALES_TARIFF_ENTRIES: usize = 20;
pub const N_TARIFF_CONSUMPTION_COSTS: usize = 20;
pub const N_COSTS_PER_TARIFF_CONS_COST: usize = 20;
pub const N_CHARGING_SCHEDULES: usize = 20;
pub const N_COSTS: usize = 20;
pub const N_VARIABLE_MONITORINGS: usize = 20;
pub const N_VARIABLE_ATTRIBUTES: usize = 20;
pub const N_CONSUMPTION_COSTS: usize = 20;
pub const N_COSTS_PER_CONS_COST: usize = 20;