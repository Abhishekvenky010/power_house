 use anchor_lang::prelude::*;

use super::order::OrderStatus;

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    InitSpace,
    Debug
)]
pub struct Node {

    pub price: u64,

    pub quantity: u64,

    pub owner: Pubkey,

    pub client_order_id: u64,

    pub timestamp: i64,

    pub order_id: u64,

    pub order_status: OrderStatus,

    pub next: u32,

    pub prev: u32,
}

