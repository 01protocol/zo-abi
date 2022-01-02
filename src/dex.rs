use anchor_lang::prelude::{declare_id, Pubkey};
use bytemuck::{Pod, PodCastError, Zeroable};
use std::mem::size_of;

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

unsafe impl Zeroable for ZoDexMarket {}
unsafe impl Pod for ZoDexMarket {}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct EventQueueHeader {
    _head_pad: [u8; 5],

    pub account_flags: u64,
    pub head: u64,
    pub count: u64,
    pub seq_num: u64,
}

unsafe impl Zeroable for EventQueueHeader {}
unsafe impl Pod for EventQueueHeader {}

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

unsafe impl Zeroable for Event {}
unsafe impl Pod for Event {}

impl ZoDexMarket {
    pub fn deserialize(buf: &[u8]) -> Result<&Self, PodCastError> {
        const FLAGS: u64 = (AccountFlag::Initialized as u64)
            | (AccountFlag::Market as u64)
            | (AccountFlag::Permissioned as u64);

        let r: &Self = bytemuck::try_from_bytes(buf)?;

        if r._head_pad[..] != *"serum".as_bytes()
            || r._tail_pad[..] != *"padding".as_bytes()
            || r.account_flags != FLAGS
        {
            panic!("Invalid buffer for market");
        }

        Ok(r)
    }
}

impl EventQueueHeader {
    pub fn deserialize(buf: &[u8]) -> Result<&Self, PodCastError> {
        const FLAGS: u64 = (AccountFlag::Initialized as u64)
            | (AccountFlag::EventQueue as u64);

        let r: &Self = bytemuck::try_from_bytes(buf)?;

        if r._head_pad[..] != *"serum".as_bytes() || r.account_flags != FLAGS {
            panic!("Invalid buffer for event queue header");
        }

        Ok(r)
    }
}

impl Event {
    pub fn split(buf: &[u8]) -> Result<(&EventQueueHeader, &[Self]), PodCastError> {
        let (header, body) = buf.split_at(size_of::<EventQueueHeader>());

        if body[(body.len() - 7)..] != *"padding".as_bytes() {
            panic!("Invalid buffer for event queue");
        }

        let header = EventQueueHeader::deserialize(header)?;

        // Omit slop and padding at the end.
        let body = &body[..(body.len() - body.len() % size_of::<Self>())];
        let body: &[Self] = bytemuck::try_cast_slice(body)?;

        Ok((header, body))
    }

    pub fn deserialize_queue(
        buf: &[u8],
    ) -> Result<
        (&EventQueueHeader, impl Iterator<Item = &Self> + '_),
        PodCastError,
    > {
        let (header, body) = Self::split(buf)?;

        let (tail, head) = body.split_at(header.head as usize);
        let head_len = head.len().min(header.count as usize);
        let tail_len = header.count as usize - head_len;

        let head = &head[..head_len];
        let tail = &tail[..tail_len];

        Ok((header, head.iter().chain(tail.iter())))
    }

    /// Iterator over sequence number and Events. Also
    /// return the new seq_num.
    pub fn deserialize_since(
        buf: &[u8],
        last_seq_num: u64,
    ) -> Result<(impl Iterator<Item = (u64, &Self)> + '_, u64), PodCastError>
    {
        let (header, body) = Self::split(buf)?;
        let len = body.len() as u64;

        const MOD32: u64 = 1u64 << 32;

        let mut missed = (MOD32 + header.seq_num - last_seq_num) % MOD32;
        if missed > len {
            missed = len - 1;
        }

        let start_seq = (MOD32 + header.seq_num - missed) % MOD32;
        let end = (header.head + header.count) % len;
        let start = (len + end - missed) % len;

        Ok((
            (0u64..missed).map(move |i| {
                let seq = (start_seq + i) % MOD32;
                let j = ((start + i) % len) as usize;

                (seq, &body[j])
            }),
            header.seq_num % MOD32,
        ))
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
