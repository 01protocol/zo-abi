use anchor_lang::prelude::Pubkey;
use bytemuck::{Pod, PodCastError, Zeroable};
use std::{mem::size_of, num::NonZeroU64};

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
#[repr(u8)]
pub enum Side {
    Bid = 0,
    Ask = 1,
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

    pub fn lots_to_price(self, n: u64) -> f64 {
        let adj = 10f64.powi(self.coin_decimals as i32 - 6i32);
        let n = n * self.pc_lot_size;
        let (q, r) = (n / self.coin_lot_size, n % self.coin_lot_size);
        (q as f64 + (r as f64 / self.coin_lot_size as f64)) * adj
    }

    pub fn lots_to_size(self, n: u64) -> f64 {
        (n * self.coin_lot_size) as f64 / 10f64.powi(self.coin_decimals as i32)
    }

    pub fn price_to_lots(self, n: f64) -> u64 {
        (n * (10f64.powi(6 - self.coin_decimals as i32)
            * self.coin_lot_size as f64
            / self.pc_lot_size as f64))
            .round() as u64
    }

    pub fn size_to_lots(self, n: f64) -> u64 {
        (n * 10f64.powi(self.coin_decimals as i32)).round() as u64
            / self.coin_lot_size
    }

    pub fn parse_order(self, n: &LeafNode, side: Side) -> Order {
        Order {
            owner_slot: n.owner_slot,
            fee_tier: n.fee_tier,
            control: n.control,
            order_id: n.key,
            client_order_id: n.client_order_id,
            size: self.lots_to_size(n.quantity),
            price: self.lots_to_price((n.key >> 64) as u64),
            side,
        }
    }
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

unsafe impl Zeroable for EventQueueHeader {}
unsafe impl Pod for EventQueueHeader {}

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

impl Event {
    pub fn split(
        buf: &[u8],
    ) -> Result<(&EventQueueHeader, &[Self]), PodCastError> {
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

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct InnerNode {
    _prefix_len: u32,
    _key: u128,
    pub children: [u32; 2],
    _pad: [u8; 40],
}

unsafe impl Zeroable for InnerNode {}
unsafe impl Pod for InnerNode {}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct LeafNode {
    pub owner_slot: u8,
    pub fee_tier: u8,
    _pad: [u8; 2],
    pub key: u128,
    pub control: Pubkey,
    pub quantity: u64,
    pub client_order_id: u64,
}

unsafe impl Zeroable for LeafNode {}
unsafe impl Pod for LeafNode {}

impl LeafNode {
    pub fn price(&self) -> NonZeroU64 {
        NonZeroU64::new((self.key >> 64) as u64).unwrap()
    }
}

enum SlabNodeRef<'a> {
    Inner(&'a InnerNode),
    Leaf(&'a LeafNode),
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct SlabNode {
    tag: u32,
    node: [u8; 68],
}

unsafe impl Zeroable for SlabNode {}
unsafe impl Pod for SlabNode {}

impl SlabNode {
    fn load(&self) -> Option<SlabNodeRef<'_>> {
        match self.tag {
            0 | 3 | 4 => None,
            1 => Some(SlabNodeRef::Inner(bytemuck::from_bytes(&self.node))),
            2 => Some(SlabNodeRef::Leaf(bytemuck::from_bytes(&self.node))),
            i => panic!("Invalid tag {} for slab node", i),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct SlabHeader {
    _head_pad: [u8; 5],
    account_flags: u64,
    _bump_index: u32,
    _pad0: [u8; 4],
    _free_list_len: u32,
    _pad1: [u8; 4],
    _free_list_head: u32,
    root: u32,
    leaf_count: u32,
    _pad2: [u8; 4],
}

unsafe impl Zeroable for SlabHeader {}
unsafe impl Pod for SlabHeader {}

#[derive(Clone, Debug)]
pub struct Slab<'a> {
    head: &'a SlabHeader,
    nodes: &'a [SlabNode],
}

impl<'a> Slab<'a> {
    pub fn deserialize(buf: &'a [u8]) -> Result<Slab<'a>, PodCastError> {
        if buf.len() < size_of::<SlabHeader>() {
            return Err(PodCastError::SizeMismatch);
        }

        let (head, tail) = buf.split_at(size_of::<SlabHeader>());
        let head: &SlabHeader = bytemuck::try_from_bytes(head)?;
        let tail = &tail[..(tail.len() - tail.len() % size_of::<SlabNode>())];

        if buf[..5] != *b"serum"
            || buf[(buf.len() - 7)..] != *b"padding"
            || head.account_flags & AccountFlag::Initialized as u64 == 0
            || (head.account_flags & AccountFlag::Bids as u64 != 0)
                == (head.account_flags & AccountFlag::Asks as u64 != 0)
        {
            panic!("Invalid buffer for slab");
        }

        Ok(Self {
            head,
            nodes: bytemuck::try_cast_slice(tail)?,
        })
    }

    pub fn is_bids(&self) -> bool {
        self.head.account_flags & AccountFlag::Bids as u64 != 0
    }

    pub fn is_asks(&self) -> bool {
        self.head.account_flags & AccountFlag::Asks as u64 != 0
    }

    pub fn side(&self) -> Side {
        match self.is_bids() {
            true => Side::Bid,
            false => Side::Ask,
        }
    }

    pub fn get_min(&self) -> Option<&LeafNode> {
        self.iter_front().next()
    }

    pub fn get_max(&self) -> Option<&LeafNode> {
        self.iter_back().next()
    }

    pub fn get_best(&self) -> Option<&LeafNode> {
        self.iter_best().next()
    }

    pub fn iter_front(&self) -> SlabIter<'_, '_> {
        SlabIter {
            slab: self,
            stack: if self.head.leaf_count > 0 {
                vec![self.head.root]
            } else {
                vec![]
            },
            ascending: true,
        }
    }

    pub fn iter_back(&self) -> SlabIter<'_, '_> {
        SlabIter {
            slab: self,
            stack: if self.head.leaf_count > 0 {
                vec![self.head.root]
            } else {
                vec![]
            },
            ascending: false,
        }
    }

    pub fn iter_best(&self) -> SlabIter<'_, '_> {
        match self.is_bids() {
            true => self.iter_back(),
            false => self.iter_front(),
        }
    }
}

pub struct SlabIter<'a, 'b: 'a> {
    slab: &'a Slab<'b>,
    stack: Vec<u32>,
    ascending: bool,
}

impl<'a, 'b: 'a> Iterator for SlabIter<'a, 'b> {
    type Item = &'a LeafNode;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.slab.nodes[self.stack.pop()? as usize].load()? {
                SlabNodeRef::Inner(x) => self.stack.extend(if self.ascending {
                    [x.children[1], x.children[0]]
                } else {
                    x.children
                }),
                SlabNodeRef::Leaf(x) => return Some(x),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    pub owner_slot: u8,
    pub fee_tier: u8,
    pub control: Pubkey,
    pub order_id: u128,
    pub client_order_id: u64,
    pub size: f64,
    pub price: f64,
    pub side: Side,
}
