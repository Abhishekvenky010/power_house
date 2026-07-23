use anchor_lang::prelude::*;


#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    Copy,
    InitSpace,
    Debug,
    PartialEq
)]
pub enum OrderType {

    // Remains in orderbook if not completely filled
    Limit,

    // Execute immediately, cancel remaining quantity
    ImmediateOrCancel,

    // Add liquidity only, never take existing orders
    PostOnly,
}



#[repr(u8)]
#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    Copy,
    InitSpace,
    Debug,
    PartialEq,
    Eq
)]
pub enum OrderStatus {

    // Fully executed
    Fill = 1,

    // Partially executed
    PartialFill = 2,

    // Waiting in orderbook
    Open = 3,

    // Removed by user
    Cancel = 4,
}