use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub global_seq: u64,
    pub next_order_id: u64,

    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,

    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,

    pub bids: Pubkey,
    pub asks: Pubkey,

    pub event_queue: Pubkey,

    pub base_lot_size: u64,
    pub quote_lot_size: u64,

    pub maker_fees_bps: u64,
    pub taker_fees_bps: u64,

    pub admin: Pubkey,

    pub vault_signer_nonce: u8,
    pub market_status: u8,

    pub min_order_size: u64,
    pub max_orders_per_user: u16,

    pub padding: [u8; 64],
}