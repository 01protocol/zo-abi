# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Added `Square` perp type
- Added `FillOrKill` order type ([#4](https://github.com/01protocol/zo-abi/pull/4))
- Added dex `Slab` deserialization and methods to find min or max price
* Added `Order` type to more easily use `Slab` leafs
- BREAKING: Bumped `anchor-lang` to `v0.21.0`
* BREAKING: Restricted `fixed` crate to `<=1.11`

## [0.3.0] - 2021-01-26

- Added `devnet` feature flag to switch IDs
- BREAKING: Updated all IDs, added new `devnet` and `mainnet` instance ones
- BREAKING: Updated structs for new instance

## [0.2.0] - 2021-01-23

- Added `create_perp_open_orders`, `cancel_perp_order`, `cancel_perp_order_by_client_id`, `settle_funds` instructions
- Added `DepositLog`, `WithdrawLog` events
- Added `ReduceOnlyIoc` and `ReduceOnlyLimit` to `OrderType`
- BREAKING: Added `payer` to `create_margin` and `create_perp_order` ([#3](https://github.com/01protocol/zo-abi/pull/3))
- BREAKING: Bumped `anchor-lang` to `v0.20.1`
- BREAKING: Renamed `dex::ID`, `serum::ID`, `state::ID` to `ZO_DEX_PID`, `SERUM_DEX_PID`, `ZO_STATE_ID`

## [0.1.0] - 2021-01-10

- Life to this repo
