use crate::types::{FeeTier, PerpType};

/// Returns taker rate x/100_000.
pub fn taker_rate(perp_type: PerpType, fee_tier: FeeTier) -> u16 {
    match perp_type {
        PerpType::Square => match fee_tier {
            FeeTier::Base => 200,
            FeeTier::Zo2 => 190,
            FeeTier::Zo3 => 180,
            FeeTier::Zo4 => 170,
            FeeTier::Zo5 => 160,
            FeeTier::Zo6 => 150,
            FeeTier::Msrm => 100,
        },
        _ => match fee_tier {
            FeeTier::Base => 100,
            FeeTier::Zo2 => 90,
            FeeTier::Zo3 => 80,
            FeeTier::Zo4 => 70,
            FeeTier::Zo5 => 60,
            FeeTier::Zo6 => 50,
            FeeTier::Msrm => 42,
        },
    }
}

/// Returns maker rate x/100_000.
pub fn maker_rate(_: PerpType, _: FeeTier) -> u16 {
    0u16
}
