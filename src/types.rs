use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone)]
pub struct Symbol {
    data: [u8; 24],
}

impl Symbol {
    pub fn is_nil(&self) -> bool {
        self.data.iter().all(|x| *x == 0)
    }
}

impl From<Symbol> for String {
    fn from(sym: Symbol) -> String {
        String::from(&sym)
    }
}

impl From<&Symbol> for String {
    fn from(sym: &Symbol) -> String {
        let mut end = 0;
        while end < sym.data.len() && sym.data[end] != 0 {
            end += 1;
        }
        String::from_utf8_lossy(&sym.data[0..end]).into()
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone)]
pub struct WrappedI80F48 {
    pub data: i128,
}

#[zero_copy]
pub struct CollateralInfo {
    pub mint: Pubkey,
    pub oracle_symbol: Symbol,
    pub decimals: u8,
    pub weight: u16,  //  in permil
    pub liq_fee: u16, // in permil
    pub is_borrowable: bool,
    pub optimal_util: u16, // in permil
    pub optimal_rate: u16, // in permil
    pub max_rate: u16,     // in permil
    pub og_fee: u16,       // in bps
    pub is_swappable: bool,
    pub serum_open_orders: Pubkey,
}

#[zero_copy]
pub struct PerpMarketInfo {
    pub symbol: Symbol, // Convention ex: "BTC-EVER-C" or "BTC-PERP"
    pub oracle_symbol: Symbol,
    pub perp_type: PerpType,
    pub asset_decimals: u8,
    pub asset_lot_size: u64,
    pub quote_lot_size: u64,
    pub strike: u64,   // in smolUSD per bigAsset
    pub base_imf: u16, // in permil (i.e. 1% <=> 10 permil)
    pub liq_fee: u16,  // in permil
    pub dex_market: Pubkey,
}

#[derive(Copy, Clone)]
pub enum PerpType {
    Future = 0,
    CallOption = 1,
    PutOption = 2,
}

#[zero_copy]
pub struct OpenOrdersInfo {
    pub key: Pubkey,
    pub native_pc_total: i64,
    pub pos_size: i64,
    pub realized_pnl: i64,
    pub coin_on_bids: u64,
    pub coin_on_asks: u64,
    pub order_count: u8,
    pub funding_index: i128,
}

#[zero_copy]
pub struct OracleCache {
    pub symbol: Symbol,
    pub sources: [OracleSource; 2],
    pub last_updated: u64,
    pub price: WrappedI80F48, // smol quote per smol asset
    pub twap: WrappedI80F48,
    pub base_decimals: u8, // actual decimal of the mint
    pub quote_decimals: u8,
}

#[derive(Copy, Clone)]
pub enum OracleType {
    Nil = 0,
    Pyth,
    Switchboard,
}

#[zero_copy]
pub struct OracleSource {
    pub ty: OracleType,
    pub key: Pubkey,
}

#[zero_copy]
pub struct MarkCache {
    pub price: WrappedI80F48, // smol usd per smol asset
    // pub twap: [Olhc; 12],
    /// Hourly twap sampled every 5min.
    pub twap: TwapInfo,
}

#[zero_copy]
pub struct TwapInfo {
    pub cumul_avg: WrappedI80F48,
    pub open: WrappedI80F48,
    pub high: WrappedI80F48,
    pub low: WrappedI80F48,
    pub close: WrappedI80F48,
    pub last_sample_start_time: u64,
}

#[zero_copy]
pub struct BorrowCache {
    pub supply: WrappedI80F48,
    pub borrows: WrappedI80F48,
    pub supply_multiplier: WrappedI80F48, // earned interest per asset supplied
    pub borrow_multiplier: WrappedI80F48, // earned interest per asset borrowed
    pub last_updated: u64,
}

#[account(zero_copy)]
pub struct State {
    pub signer_nonce: u8,
    pub admin: Pubkey,
    pub cache: Pubkey,
    pub swap_fee_vault: Pubkey,
    pub insurance: u64,          // in smol usd
    pub fees_accrued: [u64; 25], // in smol usd
    pub vaults: [Pubkey; 25],
    pub collaterals: [CollateralInfo; 25],
    pub perp_markets: [PerpMarketInfo; 50],
    pub total_collaterals: u16,
    pub total_markets: u16,
}

#[account(zero_copy)]
pub struct Margin {
    pub nonce: u8,
    pub authority: Pubkey,
    pub collateral: [WrappedI80F48; 25], // mapped to the state collaterals array, divided by entry ir_index
    pub control: Pubkey,
}

#[account(zero_copy)]
pub struct Cache {
    pub oracles: [OracleCache; 25],
    /// Mapped to `State.perp_markets`
    pub marks: [MarkCache; 50],
    pub funding_cache: [i128; 50], // long to short
    /// Mapped to 'State.collaterals'
    pub borrow_cache: [BorrowCache; 25],
}

#[account(zero_copy)]
pub struct Control {
    pub authority: Pubkey,
    pub open_orders_agg: [OpenOrdersInfo; 50], // index mapped to perp markets on state
}
