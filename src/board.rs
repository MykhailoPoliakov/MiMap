use std::collections::HashMap;

use crate::block::Block;








#[derive(Default)]
struct Board {
    // blocks
    blocks: HashMap<u64, Block>,
    order: Vec<u64>,
    active: Option<u64>,
    next_id: u64,
}


impl Board {
    // create Board
    pub fn new() -> Self {
        Board {
            blocks: HashMap::new(),
            order: Vec::new(),
            active: None,

            next_id: 0,
        }
    }


    // create new Block
    pub fn add_block(&mut self, pos: [f32;2], color: [u8;3]) -> () {
        // make new id for new block and create empty block
        self.next_id += 1;

        self.blocks.insert(self.next_id, Block::new( pos, color));

        self.order.push(self.next_id);
    }


    // delete existing Block
    pub fn del_block(&mut self, id: u64 ) -> () {
        // deleting block by deleting it`s data and deleting it from the order
        self.blocks.remove(&id);
        self.order.retain(|i| *i != id);
    }
}









#[derive(Default)]
struct Camera {
    pos: [f32;2],
    zoom: [f32;2],
}

