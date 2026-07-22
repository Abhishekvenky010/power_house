use anchor_lang::prelude::*;

pub struct InitializeMarket<'info> {
    // Market account
    #[account(init, payer = admin, space = 8 + Market::INIT_SPACE)]
    pub market: Account<'info, Market>,

    // Orderbook slabs
    #[account(init, seeds = [b"bids",market.key().as_ref()], payer = admin, space = 8 + Slab::INIT_SPACE,bump)]
    pub bids: Account<'info, Slab>,

    #[account(init, seeds = [b"asks",market.key().as_ref()] , payer = admin, space = 8 + Slab::INIT_SPACE,bump)]
    pub asks: Account<'info, Slab>,

    // Vault token accounts (program-controlled)
    #[account(
        init,
        payer = admin,
        token::mint = base_mint,
        token::authority = vault_signer
    )]
    pub base_vault: Account<'info, AnchorTokenAccount>,

    #[account(
        init,
        payer = admin,
        token::mint = quote_mint,
        token::authority = vault_signer
    )]
    pub quote_vault: Account<'info, AnchorTokenAccount>,

    // PDA that can manage vaults
    /// CHECK:
    /// This is a PDA used only as the authority for the token vaults.
    /// It holds no data, is never read or written, and is only used for signing.
    /// Safe because Anchor verifies the PDA seeds & bump.
    #[account(
        seeds = [b"vault_signer", market.key().as_ref()],
        bump
    )]
    pub vault_signer: UncheckedAccount<'info>,

    // Admin signing the transaction
    #[account(mut)]
    pub admin: Signer<'info>,

    // Token mints
    pub base_mint: Account<'info, AnchorMint>,
    pub quote_mint: Account<'info, AnchorMint>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
pub fn handler(
    ctx: Context<InitializeMarket>,
    base_lot_size: u64,
    quote_lot_size: u64,
    maker_fees_bps: u64,
    taker_fees_bps: u64,
) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let asks = &mut ctx.accounts.asks;
    let bids = &mut ctx.accounts.bids;

    asks.free_list_len = 32;
    bids.free_list_len = 32;
    asks.leaf_count = 0;
    bids.leaf_count = 0;
    asks.head_index = u32::MAX;
    bids.head_index = u32::MAX;

    market.next_order_id = 0;

    market.admin = ctx.accounts.admin.key();
    market.base_mint = ctx.accounts.base_mint.key();
    market.quote_mint = ctx.accounts.quote_mint.key();

    market.bids = bids.key();
    market.asks = asks.key();

    market.base_vault = ctx.accounts.base_vault.key();
    market.quote_vault = ctx.accounts.quote_vault.key();

    market.base_lot_size = base_lot_size;
    market.quote_lot_size = quote_lot_size;

    market.maker_fees_bps = maker_fees_bps;
    market.taker_fees_bps = taker_fees_bps;

    market.vault_signer_nonce = ctx.bumps.vault_signer;

    market.market_status = 1;
    market.max_orders_per_user = 100;
    market.min_order_size = base_lot_size;

    Ok(())
}