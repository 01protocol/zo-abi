#![allow(unused_variables)]

pub mod dex;
mod types;

pub use crate::types::*;
use anchor_lang::prelude::*;

declare_id!("DuSPvazsfthvWRuJ8TUs984VXCeUfJ1qbzd8NwkRLEpd");

#[program]
pub mod zo_abi {
    use super::*;

    pub fn update_perp_funding(
        cx: Context<UpdatePerpFunding>,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn cache_oracle(
        cx: Context<CacheOracle>,
        symbols: Vec<String>,
        mock_prices: Option<Vec<Option<u64>>>,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn cache_interest_rates(
        cx: Context<CacheInterestRates>,
        start: u8,
        end: u8,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn consume_events(
        cx: Context<ConsumeEvents>,
        limit: u16,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn crank_pnl(cx: Context<CrankPnl>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdatePerpFunding<'info> {
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
    #[account(address = dex::ID)]
    pub dex_program: AccountInfo<'info>,
}

/// Price info accounts are passed in remaining
/// accounts array.
#[derive(Accounts)]
pub struct CacheOracle<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CacheInterestRates<'info> {
    pub signer: Signer<'info>,
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub cache: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ConsumeEvents<'info> {
    #[account(mut)]
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    pub dex_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    #[account(mut)]
    pub event_queue: UncheckedAccount<'info>,
    // RA: [alice_control, bob_control, ..., alice_oo, bob_oo, ...]
}

#[derive(Accounts)]
pub struct CrankPnl<'info> {
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub state_signer: UncheckedAccount<'info>,
    #[account(mut)]
    pub cache: AccountLoader<'info, Cache>,
    pub dex_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    // RA: [alice_control, bob_control, ..., alice_oo, bob_oo, ..., alice_margin, bob_margin, ...]
}
