# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Add `place_perp_order_with_max_ts`
- Fix precision loss in `ZoDexMarket::lots_to_price`
- Add `EventFillLog` and `OtcFill` events
- BREAKING: Change accounts passed to `cache_oracle`
- BREAKING: Bump `anchor-lang` to `v0.25.0`

## [0.5.0] - 2022-04-25

- Add `taker_rate` and `maker_rate` functions
- Add `place_perp_order_lite`
- Add `ZoDexMarket::{price_to_lots,size_to_lots}`
- Add `From<String>` for `Symbol`
- BREAKING: Refactor `dex::Slab` to avoid copying the buffer, so it can be used in programs
- BREAKING: Upgrade anchor to `0.24.2` (from `0.22.1`)

## [0.4.0] - 2022-03-07

- Added `cancel_all_perp_orders`
- Added `Square` perp type
- Added `FillOrKill` order type ([#4](https://github.com/01protocol/zo-abi/pull/4))
- Added dex `Slab` deserialization and methods to find min or max price
* Added `Order` type to more easily use `Slab` leafs
* Fixed payer not being mutable on `create_margin` and `create_perp_open_orders`
* BREAKING: Removed `cancel_perp_order_by_client_id`, instead changing `cancel_perp_order` to optionally take a `client_id` argument
- BREAKING: Bumped `anchor-lang` to `v0.22.1`
* BREAKING: Restricted `fixed` crate to `>=1.8, <=1.11`

## [0.3.0] - 2022-01-26

- Added `devnet` feature flag to switch IDs
- BREAKING: Updated all IDs, added new `devnet` and `mainnet` instance ones
- BREAKING: Updated structs for new instance

## [0.2.0] - 2022-01-23

- Added `create_perp_open_orders`, `cancel_perp_order`, `cancel_perp_order_by_client_id`, `settle_funds` instructions
- Added `DepositLog`, `WithdrawLog` events
- Added `ReduceOnlyIoc` and `ReduceOnlyLimit` to `OrderType`
- BREAKING: Added `payer` to `create_margin` and `create_perp_order` ([#3](https://github.com/01protocol/zo-abi/pull/3))
- BREAKING: Bumped `anchor-lang` to `v0.20.1`
- BREAKING: Renamed `dex::ID`, `serum::ID`, `state::ID` to `ZO_DEX_PID`, `SERUM_DEX_PID`, `ZO_STATE_ID`

## [0.1.0] - 2022-01-10

- Life to this repo
