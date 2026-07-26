#[derive(Accounts)]
pub struct CancelOrder<'info> {

    // Market
    #[account(mut)]
    pub market: Account<'info, Market>,


    // Orderbooks
    #[account(mut)]
    pub bids: Account<'info, Slab>,

    #[account(mut)]
    pub asks: Account<'info, Slab>,


    // User cancelling order
    #[account(mut)]
    pub owner: Signer<'info>,


    // User receives funds back
    #[account(mut)]
    pub user_base_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_quote_ata: Account<'info, TokenAccount>,


    // Market escrow
    #[account(mut)]
    pub base_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,


    pub token_program: Program<'info, Token>,
}
pub fn handler(
    ctx: Context<CancelOrder>,
    order_id: u64,
) -> Result<()> {
    let bids = &mut ctx.accounts.bids;
    let asks = &mut ctx.accounts.asks;
    let owner = &ctx.accounts.owner;
    let market = &mut ctx.accounts.market;
    let market_key = market.key();
      // Select correct orderbook
    let slab = match side {
        Side::Ask => &mut ctx.accounts.asks,
        Side::Bid => &mut ctx.accounts.bids,
    };


    // Remove order and get deleted order details
    let deleted_order = slab.remove_order(&order_id)?;


    // Return locked funds back to user
    match side {

        // Seller cancelling SELL order
        Side::Ask => {

            unlock_ask_funds(
                market,
                deleted_order.quantity,
                &owner.key(),
                &ctx.accounts.user_base_vault,
            )?;

        }


        // Buyer cancelling BUY order
        Side::Bid => {

            unlock_bid_funds(
                market,
                deleted_order.price,
                &owner.key(),
                deleted_order.quantity,
                &ctx.accounts.user_quote_vault,
            )?;

        }
    }


    // Emit cancellation event
    dispatch_event(
        market,
        EventParams {

            event_type: EventType::Cancel,

            order_id,

            owner: owner.key(),

            counterparty: Pubkey::default(),

            side,

            price: deleted_order.price,

            base_quantity: deleted_order.quantity,

            client_order_id: deleted_order.client_order_id,

            market_pubkey: market_key,

            maker_order_id: 0,

            maker_remaining_qty: 0,

            taker_remaining_qty: 0,
        },
    )?;


    Ok(())
}