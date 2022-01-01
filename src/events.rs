use anchor_lang::prelude::*;

#[event]
pub struct RealizedPnlLog {
    pub market_key: Pubkey,
    pub margin: Pubkey,
    pub is_long: bool,
    pub pnl: i64,
    pub qty_paid: i64,
    pub qty_received: i64,
}
