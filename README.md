# rust-ocpp

[![crates.io](https://img.shields.io/crates/v/rust-ocpp.svg)](https://img.shields.io/crates/v/rust-ocpp)
[![workflow](https://img.shields.io/github/workflow/status/codelabsab/rust-ocpp/Rust)](https://img.shields.io/github/workflow/status/codelabsab/rust-ocpp/Rust)


this repo contains implementations of `ocpp` which is the Open Charge Point Protocol
used in charging stations. 

## repo structure

`ocpp/` : contains libs for 1.6 and 2.0.1

`ws/` : contains basic websockets server for v2.0.1

`docs/` : official ocpp specification

`schemas/` : jsonschemas draft 6 which is used to validate the implementation

## ws

basic implementation of ws server with v2.0.1
