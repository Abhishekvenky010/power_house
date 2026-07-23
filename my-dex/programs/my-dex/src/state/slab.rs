use anchor_lang::prelude::*;

use super::Node;

#[account]
#[derive(InitSpace, Debug)]
pub struct Slab {

    // first free node index
    pub head_index: u32,

    // available slots
    pub free_list_len: u32,

    // number of active orders
    pub leaf_count: u32,

    #[max_len(32)]
    pub nodes: Vec<Node>,
}