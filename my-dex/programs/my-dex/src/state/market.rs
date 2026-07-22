use anchor_lang::prelude::*;

/// Main configuration account for a trading market.
///
/// Example:
/// base_mint  -> SOL
/// quote_mint -> USDC
#[account]
#[derive(InitSpace)]
pub struct Market {
    // Unique sequence for market events
    pub global_seq: u64,

    // Incrementing order id generator
    pub next_order_id: u64,

    // Token identities
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,

    // Escrow vaults controlled by the program PDA
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,

    // Orderbook accounts
    pub bids: Pubkey,
    pub asks: Pubkey,

    // Event queue for indexers
    pub event_queue: Pubkey,

    // Trading precision
    pub base_lot_size: u64,
    pub quote_lot_size: u64,

    // Fee configuration
    // Basis points (100 bps = 1%)
    pub maker_fees_bps: u64,
    pub taker_fees_bps: u64,

    // Market authority
    pub admin: Pubkey,

    // PDA information
    pub vault_signer_nonce: u8,

    // 0 = inactive
    // 1 = active
    // 2 = paused
    pub market_status: u8,

    // Risk controls
    pub min_order_size: u64,
    pub max_orders_per_user: u16,

    // Future upgrades without reallocating
    pub padding: [u8; 64],
}