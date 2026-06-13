use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{block::Block};


#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub enum Mode {
    #[default]
    Looking,
    Redacting,
}


pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Board {
    // blocks
    pub blocks: HashMap<u64, Block>,
    pub order: Vec<u64>,
    pub active: Option<u64>,
    pub next_id: u64,

    pub mode: Mode,
}




impl Board {

    // move active block to top layer
    pub fn active_on_top(&mut self) {
        if let Some(target_id) = self.active {
            let index = self.order.iter().position(|id| *id == target_id).unwrap();
            let id = self.order.remove(index);
            self.order.push(id);
        }
        
    }

    // create new Block
    pub fn add_block(&mut self, pos: [f32;2], color: [u8;3]) -> u64 {
        // make new id for new block and create empty block
        self.next_id += 1;
        self.blocks.insert(self.next_id, Block::new( pos, color));
        self.order.push(self.next_id);
        self.active = Some(self.next_id);

        self.next_id
    }


    // insert existing Block
    pub fn ins_block(&mut self, clipboard_block: &Block, pos: [f32;2]) -> u64 {
        self.next_id += 1;
        // copy the block
        let mut block  = clipboard_block.clone();
        block.pos = pos; 
        // insert the block
        self.blocks.insert(self.next_id, block);
        self.order.push(self.next_id);
        self.active = Some(self.next_id);
        // return id
        self.next_id
    }


    // delete existing Block
    pub fn del_block(&mut self, id: u64 ) -> () {
        // deleting block by deleting it`s data and deleting it from the order
        self.blocks.remove(&id);
        self.order.retain(|i| *i != id);
        
        let order_len = self.order.len(); 
        if order_len > 0 {
            self.active = Some(self.order[order_len-1]);
        } else {
            self.active = None;
        }
    }


    // make nearest block in the chousen direction active 
    pub fn select_near_block(&mut self, direction: Direction) -> () {
        // if no active block
        let Some(current_id) = self.active else { return; };

        let mut next: Option<(u64, f32)> = None;

        let current_pos = self.blocks[&current_id].pos;
        for id in &self.order {
            let pos = self.blocks[&id].pos;
            if id == &current_id { continue; }

            let straight_distance = {
                match direction {
                    Direction::Right => { pos[0] - current_pos[0] },
                    Direction::Left  => { current_pos[0] - pos[0] },
                    Direction::Up    => { current_pos[1] - pos[1] },
                    Direction::Down  => { pos[1] - current_pos[1] },
                }
            };
            let offset = {
                match direction {
                    Direction::Right | Direction::Left => { (pos[1] - current_pos[1]).abs() },
                    Direction::Up    | Direction::Down => { (pos[0] - current_pos[0]).abs() },
                }
            };

            if offset/2. >= straight_distance { continue; };
            let distance = straight_distance + offset/2.;

            match next {
                Some(value) => {
                    if distance < value.1 {
                        next = Some((*id, distance));
                    }
                },
                None => {
                    next = Some((*id, distance));
                },
            }
        }

        // make it active
        if let Some(value) = next {
            self.active = Some(value.0);
        }
    }
}