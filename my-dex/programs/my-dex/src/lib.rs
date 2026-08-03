use crate::states::order_schema::enums::Side;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint as AnchorMint, Token, TokenAccount as AnchorTokenAccount};

declare_id!("n5Q5ZKqMkBoWen1QAb8yr8SkD7QGHp6RXLQNmkPqtMn");

pub mod assets;
pub mod calculate;
pub mod errors;
pub mod events;
pub mod helpers;
pub mod instructions;
pub mod state;
pub mod states;
use state::*;
use instructions::*;

#[program]
pub mod orderbook {
    use super::*;

    pub fn initialise_market(
        ctx: Context<InitializeMarket>,
        base_lot_size: u64,
        quote_lot_size: u64,
        maker_fees_bps: u64,
        taker_fees_bps: u64,
    ) -> Result<()> {
        instructions::initialize_market::handler(
            ctx,
            base_lot_size,
            quote_lot_size,
            maker_fees_bps,
            taker_fees_bps,
        )
    }

    pub fn place_limit_order(
        ctx: Context<PlaceLimitOrder>,
        max_base_size: u64,
        client_order_id: u64,
        price: u64,
        order_type: OrderType,
        side: Side,
    ) -> Result<()> {
        instructions::place_limit_order::handler(
            ctx,
            max_base_size,
            client_order_id,
            price,
            order_type,
            side,
        )
    }

    pub fn place_ioc_order(
        ctx: Context<PlaceIOCOrder>,
        base_qty: u64,
        price_in_raw_units: u64,
        order_type: OrderType,
        side: Side,
    ) -> Result<()> {
        instructions::place_ioc::handler(
            ctx,
            base_qty,
            price_in_raw_units,
            order_type,
            side,
        )
    }

    pub fn place_post_only_order(
        ctx: Context<PlacePostOnlyOrder>,
        base_qty: u64,
        price_in_raw_units: u64,
        order_type: OrderType,
        client_order_id: u64,
        side: Side,
    ) -> Result<()> {
        instructions::place_post_only::handler(
            ctx,
            base_qty,
            price_in_raw_units,
            order_type,
            client_order_id,
            side,
        )
    }

    pub fn cancel_order(ctx: Context<CancelOrder>, order_id: u64, side: Side) -> Result<()> {
        instructions::cancel_order::handler(ctx, order_id, side)
    }
}