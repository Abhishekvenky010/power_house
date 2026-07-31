use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::Market;
use crate::state::Slab;


#[derive(Accounts)]
pub struct InitializeMarket<'info> {

    #[account(
        init,
        payer = admin,
        space = 8 + Market::INIT_SPACE
    )]
    pub market: Account<'info, Market>,


    #[account(
        init,
        payer = admin,
        space = 8 + Slab::INIT_SPACE,
        seeds = [b"bids", market.key().as_ref()],
        bump
    )]
    pub bids: Account<'info, Slab>,


    #[account(
        init,
        payer = admin,
        space = 8 + Slab::INIT_SPACE,
        seeds = [b"asks", market.key().as_ref()],
        bump
    )]
    pub asks: Account<'info, Slab>,


    #[account(
        init,
        payer = admin,
        token::mint = base_mint,
        token::authority = vault_signer
    )]
    pub base_vault: Account<'info, TokenAccount>,


    #[account(
        init,
        payer = admin,
        token::mint = quote_mint,
        token::authority = vault_signer
    )]
    pub quote_vault: Account<'info, TokenAccount>,


    /// CHECK:
    /// PDA authority for vaults
    #[account(
        seeds = [b"vault_signer", market.key().as_ref()],
        bump
    )]
    pub vault_signer: UncheckedAccount<'info>,


    #[account(mut)]
    pub admin: Signer<'info>,


    pub base_mint: Account<'info, Mint>,

    pub quote_mint: Account<'info, Mint>,


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

    market.next_order_id = 0;

    market.admin = ctx.accounts.admin.key();

    market.base_mint = ctx.accounts.base_mint.key();
    market.quote_mint = ctx.accounts.quote_mint.key();

    market.bids = ctx.accounts.bids.key();
    market.asks = ctx.accounts.asks.key();

    market.base_lot_size = base_lot_size;
    market.quote_lot_size = quote_lot_size;

    market.maker_fees_bps = maker_fees_bps;
    market.taker_fees_bps = taker_fees_bps;

    market.market_status = 1;

    market.min_order_size = base_lot_size;
    market.max_orders_per_user = 100;

    market.base_vault = ctx.accounts.base_vault.key();

    market.quote_vault = ctx.accounts.quote_vault.key();

    market.vault_signer_nonce = ctx.bumps.vault_signer;

    Ok(())
}