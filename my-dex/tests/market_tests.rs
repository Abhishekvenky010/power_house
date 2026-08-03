use anchor_lang::prelude::*;

use my_dex::state::{
    Market,
    TraderEntry,
    TraderState,
};


#[test]
fn test_get_trader_index_existing() {

    // Create fake wallet addresses
    let trader1 = Pubkey::new_unique();
    let trader2 = Pubkey::new_unique();


    // Create a fake market state
    let market = Market {

        global_seq: 0,
        next_order_id: 0,

        base_mint: Pubkey::default(),
        quote_mint: Pubkey::default(),

        base_vault: Pubkey::default(),
        quote_vault: Pubkey::default(),

        bids: Pubkey::default(),
        asks: Pubkey::default(),
        event_queue: Pubkey::default(),

        base_lot_size: 1,
        quote_lot_size: 1,

        maker_fees_bps: 0,
        taker_fees_bps: 0,

        admin: Pubkey::default(),

        vault_signer_nonce: 0,
        market_status: 1,

        min_order_size: 1,
        max_orders_per_user: 100,

        padding: [0u8; 64],

        trader_entry: vec![

            TraderEntry {
                trader_key: trader1,

                trader_state: TraderState {
                    quote_lots_locked: 0,
                    quote_lots_free: 0,
                    base_lots_free: 0,
                    base_lots_locked: 0,
                },
            },


            TraderEntry {
                trader_key: trader2,

                trader_state: TraderState {
                    quote_lots_locked: 0,
                    quote_lots_free: 0,
                    base_lots_free: 0,
                    base_lots_locked: 0,
                },
            },
        ],
    };


    // Search trader2
    let index = market.get_trader_index(&trader2);


    // trader2 should be at index 1
    assert_eq!(index, Some(1));
}



#[test]
fn test_get_trader_index_missing() {

    let trader1 = Pubkey::new_unique();
    let unknown_trader = Pubkey::new_unique();


    let market = Market {

        global_seq: 0,
        next_order_id: 0,

        base_mint: Pubkey::default(),
        quote_mint: Pubkey::default(),

        base_vault: Pubkey::default(),
        quote_vault: Pubkey::default(),

        bids: Pubkey::default(),
        asks: Pubkey::default(),
        event_queue: Pubkey::default(),

        base_lot_size: 1,
        quote_lot_size: 1,

        maker_fees_bps: 0,
        taker_fees_bps: 0,

        admin: Pubkey::default(),

        vault_signer_nonce: 0,
        market_status: 1,

        min_order_size: 1,
        max_orders_per_user: 100,

        padding: [0u8; 64],

        trader_entry: vec![

            TraderEntry {
                trader_key: trader1,

                trader_state: TraderState {
                    quote_lots_locked: 0,
                    quote_lots_free: 0,
                    base_lots_free: 0,
                    base_lots_locked: 0,
                },
            }
        ],
    };


    let index = market.get_trader_index(&unknown_trader);


    // trader does not exist
    assert_eq!(index, None);
}