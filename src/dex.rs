use anchor_lang::prelude::{declare_id, Pubkey};
use std::mem::{size_of, transmute};

declare_id!("CX8xiCu9uBrLX5v3DSeHX5SEvGT36PSExES2LmzVcyJd");

#[derive(Copy, Clone, Debug)]
#[repr(u64)]
pub enum AccountFlag {
    Initialized = 1u64 << 0,
    Market = 1u64 << 1,
    OpenOrders = 1u64 << 2,
    RequestQueue = 1u64 << 3,
    EventQueue = 1u64 << 4,
    Bids = 1u64 << 5,
    Asks = 1u64 << 6,
    Disabled = 1u64 << 7,
    Closed = 1u64 << 8,
    Permissioned = 1u64 << 9,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct ZoDexMarket {
    _head_pad: [u8; 5],

    pub account_flags: u64,
    pub own_address: Pubkey,
    pub pc_fees_accrued: u64,
    pub req_q: Pubkey,
    pub event_q: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub fee_rate_bps: u64,
    pub referrer_rebates_accrued: u64,
    pub funding_index: i128,
    pub last_updated: u64,
    pub strike: u64,
    pub perp_type: u64,
    pub coin_decimals: u64,
    pub open_interest: u64,

    pub open_orders_authority: Pubkey,
    pub prune_authority: Pubkey,

    _pad: [u8; 1032],
    _tail_pad: [u8; 7],
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum EventFlag {
    Fill = 0x1,
    Out = 0x2,
    Bid = 0x4,
    Maker = 0x8,
    ReleaseFunds = 0x10,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct EventQueueHeader {
    _head_pad: [u8; 5],

    pub account_flags: u64,
    pub head: u64,
    pub count: u64,
    pub seq_num: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Event {
    pub event_flags: u8,
    pub owner_slot: u8,
    pub fee_tier: u8,

    _pad: [u8; 5],

    pub native_qty_released: u64,
    pub native_qty_paid: u64,
    pub native_fee_or_rebate: u64,
    pub order_id: u128,
    pub control: Pubkey,
    pub client_order_id: u64,
}

impl ZoDexMarket {
    const SIZE: usize = size_of::<Self>();
    const FLAGS: u64 = (AccountFlag::Initialized as u64)
        | (AccountFlag::Market as u64)
        | (AccountFlag::Permissioned as u64);

    pub fn deserialize(buf: &[u8]) -> Self {
        let buf: [u8; Self::SIZE] =
            buf.try_into().expect("Invalid buffer length");

        unsafe {
            let r: Self = transmute(buf);

            // SAFETY: Ensure the decoded account is valid.
            if r._head_pad[..] != *"serum".as_bytes()
                || r._tail_pad[..] != *"padding".as_bytes()
                || r.account_flags != Self::FLAGS
            {
                panic!("Invalid buffer for market");
            }

            r
        }
    }
}

impl EventQueueHeader {
    const SIZE: usize = size_of::<Self>();
    const FLAGS: u64 =
        (AccountFlag::Initialized as u64) | (AccountFlag::EventQueue as u64);

    pub fn deserialize(buf: &[u8]) -> Self {
        let buf: [u8; Self::SIZE] =
            buf.try_into().expect("Invalid buffer length");

        unsafe {
            let r: Self = transmute(buf);

            // SAFETY: Ensure decoded account is valid.
            if r._head_pad[..] != *"serum".as_bytes()
                || r.account_flags != Self::FLAGS
            {
                panic!("Invalid buffer for market");
            }

            r
        }
    }
}

impl Event {
    const SIZE: usize = size_of::<Self>();

    /// Iterator over sequence number and Events. Also
    /// return the new seq_num.
    pub fn deserialize_since<'a>(
        buf: &'a [u8],
        last_seq_num: u64,
    ) -> (impl Iterator<Item = (u64, Self)> + 'a, u64) {
        let head =
            EventQueueHeader::deserialize(&buf[..EventQueueHeader::SIZE]);
        let buf: &'a _ = &buf[EventQueueHeader::SIZE..];
        let len = (buf.len() / Self::SIZE) as u64;

        const MOD32: u64 = 1u64 << 32;

        let mut missed = (MOD32 + head.seq_num - last_seq_num) % MOD32;
        if missed > len {
            missed = len - 1;
        }

        let start_seq = (MOD32 + head.seq_num - missed) % MOD32;
        let end = (head.head + head.count) % len;
        let start = (len + end - missed) % len;

        (
            (0u64..missed).map(move |i| {
                let j = ((start + i) % len) as usize;
                let j = j * Self::SIZE;

                let seq = (start_seq + i) % MOD32;

                let ev: [u8; Self::SIZE] = (&buf[j..(j + Self::SIZE)])
                    .try_into()
                    .expect("Invalid buffer length");

                unsafe {
                    let ev: Self = transmute(ev);
                    (seq, ev)
                }
            }),
            head.seq_num % MOD32,
        )
    }

    pub fn is_fill(&self) -> bool {
        self.event_flags & (EventFlag::Fill as u8) != 0
    }

    pub fn is_bid(&self) -> bool {
        self.event_flags & (EventFlag::Bid as u8) != 0
    }

    pub fn is_maker(&self) -> bool {
        self.event_flags & (EventFlag::Maker as u8) != 0
    }
}
