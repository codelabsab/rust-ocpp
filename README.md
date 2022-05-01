# rust-ocpp

[![crates.io](https://img.shields.io/crates/v/rust-ocpp.svg)](https://crates.io/crates/rust-ocpp)
[![workflow](https://img.shields.io/github/workflow/status/codelabsab/rust-ocpp/Rust)](https://github.com/codelabsab/rust-ocpp/actions)
[![codecov](https://codecov.io/gh/codelabsab/rust-ocpp/branch/main/graph/badge.svg?token=23C458RC3S)](https://codecov.io/gh/codelabsab/rust-ocpp)

The `rust-ocpp` libs implements the Open Charge Point Protocol
used in charging stations. You can read more on the official [Open Charge Alliance](https://www.openchargealliance.org/) website.

Both OCPP v1.6 and v2.0.1 are implemented and validated using the official json schemas from Open Charge Alliance.

You can find the tests in `schema_validation.rs` for both `v.1.6` and `v2.0.1`

## repo structure

`src/` : library files for v1.6 and v2.0.1

`docs/` : official ocpp specification

## contribute

Use `rustfmt` before you PR.

pre-commit config is available. You can read more about it at [pre-commits](https://pre-commit.com) website and checkout their repo on [github](https://github.com/pre-commit/pre-commit)
