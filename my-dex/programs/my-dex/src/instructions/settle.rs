use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::errors::SettlementError;
use crate::state::Market;
use crate::helpers::transfer_if_needed;


#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        address = market.base_vault
    )]
    pub base_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = market.quote_vault
    )]
    pub quote_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = trader_base_ata.owner == authority.key(),
        constraint = trader_base_ata.mint == market.base_mint,
    )]
    pub trader_base_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = trader_quote_ata.owner == authority.key(),
        constraint = trader_quote_ata.mint == market.quote_mint,
    )]
    pub trader_quote_ata: Account<'info, TokenAccount>,

    /// CHECK: PDA authority for vaults
    #[account(
        seeds = [b"vault_signer", market.key().as_ref()],
        bump = market.vault_signer_nonce,
    )]
    pub vault_signer: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
pub fn handler(ctx: Context<Settle>) -> Result<()> {
    let trader_key = ctx.accounts.authority.key();
    let market = &mut ctx.accounts.market;

    // Copy immutable values first
    let market_key = market.key();
    let vault_signer_nonce = market.vault_signer_nonce;
    let base_lot_size = market.base_lot_size;
    let quote_lot_size = market.quote_lot_size;

    // ------------------------------------------
    // Borrow trader only to calculate amounts
    // ------------------------------------------
    let (base_amount, quote_amount) = {
        let trader_entry = market
            .get_trader_entry(&trader_key)
            .ok_or(SettlementError::TraderNotFound)?;

        let base_amount = trader_entry
            .trader_state
            .base_lots_free
            .checked_mul(base_lot_size)
            .ok_or(SettlementError::MathOverflow)?;

        let quote_amount = trader_entry
            .trader_state
            .quote_lots_free
            .checked_mul(quote_lot_size)
            .ok_or(SettlementError::MathOverflow)?;

        (base_amount, quote_amount)
    };

    require!(
        base_amount > 0 || quote_amount > 0,
        SettlementError::NothingToSettle
    );

    let signer_seeds: &[&[u8]] = &[
        b"vault_signer",
        market_key.as_ref(),
        &[vault_signer_nonce],
    ];

    // ------------------------------------------
    // Transfer Base
    // ------------------------------------------
    transfer_if_needed(
        &ctx.accounts.token_program,
        &ctx.accounts.base_vault,
        &ctx.accounts.trader_base_ata,
        &ctx.accounts.vault_signer,
        signer_seeds,
        base_amount,
    )?;

    // ------------------------------------------
    // Transfer Quote
    // ------------------------------------------
    transfer_if_needed(
        &ctx.accounts.token_program,
        &ctx.accounts.quote_vault,
        &ctx.accounts.trader_quote_ata,
        &ctx.accounts.vault_signer,
        signer_seeds,
        quote_amount,
    )?;

    // ------------------------------------------
    // Borrow trader AGAIN to clear balances
    // ------------------------------------------
    {
        let trader_entry = market
            .get_trader_entry(&trader_key)
            .ok_or(SettlementError::TraderNotFound)?;

        trader_entry.trader_state.base_lots_free = 0;
        trader_entry.trader_state.quote_lots_free = 0;
    }

    Ok(())
}