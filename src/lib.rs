#![doc = include_str!("../README.md")]

mod types;
use anchor_lang::prelude::*;
use solana_program::pubkey;

pub mod dex;
pub mod events;
pub use crate::types::*;

#[cfg(feature = "devnet")]
declare_id!("Zo1ThtSHMh9tZGECwBDL81WJRL6s3QTHf733Tyko7KQ");

#[cfg(not(feature = "devnet"))]
declare_id!("Zo1ggzTUKMY5bYnDvT5mtVeZxzf2FaLTbKkmvGUhUQk");

pub static ZO_DEX_PID: Pubkey = match cfg!(feature = "devnet") {
    true => pubkey!("ZDxUi178LkcuwdxcEqsSo2E7KATH99LAAXN5LcSVMBC"),
    false => pubkey!("ZDx8a8jBqGmJyxi1whFxxCo5vG6Q9t4hTzW2GSixMKK"),
};

pub static SERUM_DEX_PID: Pubkey = match cfg!(feature = "devnet") {
    true => pubkey!("DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY"),
    false => pubkey!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"),
};

pub static ZO_STATE_ID: Pubkey = match cfg!(feature = "devnet") {
    true => pubkey!("KwcWW7WvgSXLJcyjKZJBHLbfriErggzYHpjS9qjVD5F"),
    false => pubkey!("71yykwxq1zQqy99PgRsgZJXi2HHK2UDx9G4va7pH6qRv"),
};

/// Returns taker rate x/100_000
pub fn taker_rate(perp_type: PerpType, fee_tier: FeeTier) -> u16 {
    if perp_type == PerpType::Square {
        match fee_tier {
            FeeTier::Base => 200,
            FeeTier::ZO2 => 190,
            FeeTier::ZO3 => 180,
            FeeTier::ZO4 => 170,
            FeeTier::ZO5 => 160,
            FeeTier::ZO6 => 150,
            FeeTier::MSRM => 100,
        }
    } else {
        match fee_tier {
            FeeTier::Base => 100,
            FeeTier::ZO2 => 90,
            FeeTier::ZO3 => 80,
            FeeTier::ZO4 => 70,
            FeeTier::ZO5 => 60,
            FeeTier::ZO6 => 50,
            FeeTier::MSRM => 42,
        }
    }
}

/// Returns maker rate x/100_000
pub fn maker_rate(_perp_type: PerpType, _fee_tier: FeeTier) -> u16 {
    0u16
}

#[program]
mod zo_abi {
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(clippy::too_many_arguments)]

    use super::*;

    // ========== MARGIN ==========

    pub(crate) fn create_margin(
        cx: Context<CreateMargin>,
        margin_nonce: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn deposit(
        cx: Context<Deposit>,
        repay_only: bool,
        amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn withdraw(
        cx: Context<Withdraw>,
        allow_borrow: bool,
        amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    // ========== TRADING ==========

    /// Creates a trader's open orders account for a given market
    pub(crate) fn create_perp_open_orders(
        cx: Context<CreatePerpOpenOrders>,
    ) -> Result<()> {
        Ok(())
    }

    /// Places a new order
    pub(crate) fn place_perp_order(
        cx: Context<PlacePerpOrder>,
        is_long: bool,
        limit_price: u64,
        max_base_quantity: u64,
        max_quote_quantity: u64,
        order_type: OrderType,
        limit: u16,
        client_id: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Places a new order (lite version uses less compute, does not settle funds automatically)
    /// Currently only available on devnet
    pub(crate) fn place_perp_order_lite(
        cx: Context<PlacePerpOrder>,
        is_long: bool,
        limit_price: u64,
        max_base_quantity: u64,
        max_quote_quantity: u64,
        order_type: OrderType,
        limit: u16,
        client_id: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Cancels an order on the book, using either `order_id` and `is_long` or only `client_id`.
    pub(crate) fn cancel_perp_order(
        cx: Context<CancelPerpOrder>,
        order_id: Option<u128>,
        is_long: Option<bool>,
        client_id: Option<u64>,
    ) -> Result<()> {
        Ok(())
    }

    /// Cancels all orders on the book
    pub(crate) fn cancel_all_perp_orders(
        cx: Context<CancelAllPerpOrders>,
        limit: u16,
    ) -> Result<()> {
        Ok(())
    }

    /// Settles unrealized funding and realized pnl into the margin account
    pub(crate) fn settle_funds(cx: Context<SettleFunds>) -> Result<()> {
        Ok(())
    }

    /// Swaps two tokens on a single A/B market, where A is the base currency
    /// and B is the quote currency. This is just a direct IOC trade that
    /// instantly settles.
    ///
    /// When side is "bid", then swaps B for A. When side is "ask", then swaps
    /// A for B.
    pub(crate) fn swap(
        cx: Context<Swap>,
        buy: bool,
        allow_borrow: bool, // whether the withdraw currency can go below 0
        amount: u64,        // smol amount to swap *from*
        min_rate: u64, // number of smol tokens received from a single big token given
    ) -> Result<()> {
        Ok(())
    }

    // ========== KEEPERS ==========

    pub(crate) fn update_perp_funding(
        cx: Context<UpdatePerpFunding>,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cache_oracle(
        cx: Context<CacheOracle>,
        symbols: Vec<String>,
        mock_prices: Option<Vec<Option<u64>>>,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cache_interest_rates(
        cx: Context<CacheInterestRates>,
        start: u8,
        end: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn consume_events(
        cx: Context<ConsumeEvents>,
        limit: u16,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn crank_pnl(cx: Context<CrankPnl>) -> Result<()> {
        Ok(())
    }

    // ========== LIQUIDATION ==========

    /// Force cancels all orders of an account under liquidation
    pub(crate) fn force_cancel_all_perp_orders(
        cx: Context<ForceCancelAllPerpOrders>,
        limit: u16,
    ) -> Result<()> {
        Ok(())
    }

    /// Liquidates a perp position by transferring it from the liqee to the liqor
    pub(crate) fn liquidate_perp_position(
        cx: Context<LiquidatePerpPosition>,
        asset_transfer_lots: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Liquidates a spot position by transferring it from the liqee to the liqor
    pub(crate) fn liquidate_spot_position(
        cx: Context<LiquidateSpotPosition>,
        asset_transfer_amount: i64,
    ) -> Result<()> {
        Ok(())
    }

    /// Transfer negative borrows from liqee to liqor, and subsidize through insurance fund
    pub(crate) fn settle_bankruptcy(
        cx: Context<SettleBankruptcy>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
struct SettleFunds<'info> {
    pub authority: Signer<'info>,
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
struct CancelPerpOrder<'info> {
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub event_q: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
struct CancelAllPerpOrders<'info> {
    pub authority: Signer<'info>,
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    #[account(mut)]
    pub req_q: UncheckedAccount<'info>,
    #[account(mut)]
    pub event_q: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
struct CreatePerpOpenOrders<'info> {
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    // if authority is a pda, use a non-pda as payer
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub margin: AccountLoader<'info, Margin>,
    #[account(mut)]
    pub control: AccountLoader<'info, Control>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub dex_market: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
struct CreateMargin<'info> {
    pub state: AccountInfo<'info>,
    pub authority: Signer<'info>,
    // if authority is a pda, use a non-pda as payer
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Must be an uninitialized Keypair with
    /// `seeds = [authority.key.as_ref(), state.key().as_ref(), b\"marginv1\".as_ref()]`
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    /// The control account must be created as a pre-instruction, with the correct size, and with
    /// the zo program as the owner. Current size is 8 + 4482
    #[account(zero)]
    pub control: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
struct Deposit<'info> {
    pub state: AccountInfo<'info>,
    /// ` seeds = [state.key().as_ref()] `
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    /// Vault pubkey can be found from the State account
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
struct Withdraw<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    /// ` seeds = [state.key().as_ref()] `
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub control: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    /// Vault pubkey can be found from the State account
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
struct UpdatePerpFunding<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = ZO_DEX_PID)]
    pub dex_program: AccountInfo<'info>,
}

/// Price info accounts are passed in remaining
/// accounts array.
#[derive(Accounts)]
struct CacheOracle<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
}

#[derive(Accounts)]
struct CacheInterestRates<'info> {
    pub signer: Signer<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
}

#[derive(Accounts)]
struct ConsumeEvents<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    // RA: [alice_control, bob_control, ..., alice_oo, bob_oo, ...]
}

#[derive(Accounts)]
struct CrankPnl<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    // RA: [alice_control, bob_control, ..., alice_oo, bob_oo, ..., alice_margin, bob_margin, ...]
}

#[derive(Accounts)]
struct ForceCancelAllPerpOrders<'info> {
    pub pruner: Signer<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_oo: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub req_q: AccountInfo<'info>,
    #[account(mut)]
    pub event_q: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = ZO_DEX_PID)]
    pub dex_program: AccountInfo<'info>,
}

#[derive(Accounts)]
struct LiquidatePerpPosition<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    pub liqor: Signer<'info>,
    #[account(mut)]
    pub liqor_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqor_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqor_oo: AccountInfo<'info>,
    pub liqee: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_oo: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub req_q: AccountInfo<'info>,
    #[account(mut)]
    pub event_q: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = ZO_DEX_PID)]
    pub dex_program: AccountInfo<'info>,
}

#[derive(Accounts)]
struct LiquidateSpotPosition<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub liqor: Signer<'info>,
    #[account(mut)]
    pub liqor_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqor_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_control: AccountInfo<'info>,
    pub asset_mint: AccountInfo<'info>,
    pub quote_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
struct PlacePerpOrder<'info> {
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub control: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub dex_market: AccountInfo<'info>,
    #[account(mut)]
    pub req_q: AccountInfo<'info>,
    #[account(mut)]
    pub event_q: AccountInfo<'info>,
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    #[account(address = ZO_DEX_PID)]
    pub dex_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
struct SettleBankruptcy<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    pub liqor: Signer<'info>,
    #[account(mut)]
    pub liqor_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqor_control: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_margin: AccountInfo<'info>,
    #[account(mut)]
    pub liqee_control: AccountInfo<'info>,
    pub asset_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
struct Swap<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub state: AccountInfo<'info>,
    pub state_signer: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
    #[account(mut)]
    pub margin: AccountInfo<'info>,
    #[account(mut)]
    pub control: AccountInfo<'info>,
    /// For the dex, this is the _quote_ mint. However, for zo, quote
    /// refers to the collateral at index 0, normally USDC.
    pub quote_mint: AccountInfo<'info>,
    #[account(mut)]
    pub quote_vault: AccountInfo<'info>,
    pub asset_mint: AccountInfo<'info>,
    #[account(mut)]
    pub asset_vault: AccountInfo<'info>,
    #[account(mut)]
    pub swap_fee_vault: AccountInfo<'info>,
    #[account(mut)]
    pub serum_open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut)]
    pub serum_request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    #[account(mut)]
    pub serum_coin_vault: AccountInfo<'info>,
    #[account(mut)]
    pub serum_pc_vault: AccountInfo<'info>,
    pub serum_vault_signer: AccountInfo<'info>,
    #[account(address = SERUM_DEX_PID)]
    pub srm_spot_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}
