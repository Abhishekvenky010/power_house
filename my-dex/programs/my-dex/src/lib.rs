use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("GGHk4XYCLfv6cpe26drrGKuhV2cdHToWPTDgLZnTfFuz");


#[program]
pub mod my_dex {

    use super::*;

    pub fn initialize_market(
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
}