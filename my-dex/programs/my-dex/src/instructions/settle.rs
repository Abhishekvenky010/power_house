use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::errors::SettlementError;
use crate::state::Market;

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub base_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub trader_base_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub trader_quote_ata: Account<'info, TokenAccount>,

    /// CHECK: PDA signer for vault transfers, validated by seeds and bump
    #[account(
        seeds = [b"vault_signer", market.key().as_ref()],
        bump
    )]
    pub vault_signer: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Settle>) -> Result<()> {
    let market = &ctx.accounts.market;
    let base_lot_size = market.base_lot_size;
    let quote_lot_size = market.quote_lot_size;
    let base_mint = market.base_mint;
    let quote_mint = market.quote_mint;
    let market_key = market.key();
    let trader_key = ctx.accounts.authority.key();

    let market_account = &mut ctx.accounts.market;
    let trader_entry = market_account
        .get_trader_entry(&trader_key)
        .ok_or(SettlementError::NonSettleableEvent)?;

    require_keys_eq!(
        ctx.accounts.trader_base_ata.mint,
        base_mint,
        SettlementError::InvalidBaseAta
    );
    require_keys_eq!(
        ctx.accounts.trader_quote_ata.mint,
        quote_mint,
        SettlementError::InvalidQuoteAta
    );
    require_keys_eq!(
        ctx.accounts.trader_base_ata.owner,
        trader_key,
        SettlementError::InvalidOwnerBaseAta
    );
    require_keys_eq!(
        ctx.accounts.trader_quote_ata.owner,
        trader_key,
        SettlementError::InvalidOwnerQuoteAta
    );

    let base_amount = trader_entry
        .trader_state
        .base_lots_free
        .checked_mul(base_lot_size)
        .ok_or(SettlementError::NonSettleableEvent)?;

    let quote_amount = trader_entry
        .trader_state
        .quote_lots_free
        .checked_mul(quote_lot_size)
        .ok_or(SettlementError::NonSettleableEvent)?;

    if base_amount == 0 && quote_amount == 0 {
        return err!(SettlementError::NonSettleableEvent);
    }

    let signer_seeds = &[
        b"vault_signer",
        market_key.as_ref(),
        &[ctx.bumps.vault_signer],
    ];

    if base_amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.base_vault.to_account_info(),
                    to: ctx.accounts.trader_base_ata.to_account_info(),
                    authority: ctx.accounts.vault_signer.to_account_info(),
                },
                &[signer_seeds],
            ),
            base_amount,
        )?;
    }

    if quote_amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.quote_vault.to_account_info(),
                    to: ctx.accounts.trader_quote_ata.to_account_info(),
                    authority: ctx.accounts.vault_signer.to_account_info(),
                },
                &[signer_seeds],
            ),
            quote_amount,
        )?;
    }

    trader_entry.trader_state.base_lots_free = 0;
    trader_entry.trader_state.quote_lots_free = 0;

    Ok(())
}
