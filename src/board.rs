use std::collections::HashMap;
use eframe::egui;

use crate::{block::Block};


#[derive(Default)]
enum Mode {
    #[default]
    Looking,
    Redacting,
}

#[derive(Default)]
pub struct Board {
    // blocks
    blocks: HashMap<u64, Block>,
    order: Vec<u64>,
    active: Option<u64>,
    next_id: u64,

    clipboard: Option<Block>,

    mode: Mode,
}


impl Board {
    // create Board
    pub fn new() -> Self {
        Board {
            // blocks
            blocks: HashMap::new(),
            order: Vec::new(),
            active: None,
            next_id: 0,

            clipboard: None,

            mode: Mode::Looking,
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


    // select block
    pub fn select_block(&mut self, id: u64) -> () {
        self.active = Some(id); 
    }

    // deselect block
    pub fn deselect_block(&mut self) -> () {
        self.active = None;
    }
}

// get input logs
fn input_logs(ctx: &egui::Context) {
    ctx.input(|i| {
        for event in &i.events {
            println!("{:?}", event);
        }
    });
}


impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        input_logs(ctx);

        let (ctrl_x, ctrl_c, ctrl_v) = ctx.input_mut(|i| {
            let cut   = i.events.iter().any(|e| matches!(e, egui::Event::Cut));
            let copy  = i.events.iter().any(|e| matches!(e, egui::Event::Copy));
            let paste = i.events.iter().any(|e| matches!(e, egui::Event::Paste(_)));
            let v_released = i.events.iter().any(|e| matches!(e,
                egui::Event::Key { key: egui::Key::V, pressed: false, .. }));
            i.events.retain(|e| !matches!(e, egui::Event::Paste(_)));
            (cut, copy, paste || v_released)
        });

        let (arrow_left, arrow_right, arrow_up, arrow_down, key_i) = ctx.input(|i| (
            i.key_pressed(egui::Key::ArrowLeft),
            i.key_pressed(egui::Key::ArrowRight),
            i.key_pressed(egui::Key::ArrowUp),
            i.key_pressed(egui::Key::ArrowDown),
            i.key_pressed(egui::Key::I),
        ));

        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());


        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());


            // INPUT

            // match the mode
            match self.mode {
                Mode::Looking => {
                    ctx.input(|i| {
                        // cut the block | Ctrl + X
                        if ctrl_x {
                            if let Some(x) = self.active {
                                let block = self.blocks[&x].clone();
                                self.clipboard = Some(block);
                                self.del_block(x);
                            }
                        }
                        // copy the block | Ctrl + C
                        if ctrl_c {
                            if let Some(x) = self.active {
                                self.clipboard = Some(self.blocks[&x].clone());
                            }
                        }
                        // paste the block | Ctrl + V
                        if ctrl_v {
                            println!("ctrl_v");
                            if let Some(x) = self.clipboard.clone() {
                                println!("ctrl_v");
                                let pos = mouse_pos.unwrap_or(response.rect.center());
                                self.ins_block(&x, [pos.x,pos.y]);
                                
                            }
                        }

                        if arrow_left {
                            println!("left");
                        }
                        if arrow_right {
                            println!("right");
                        }
                        if arrow_up {
                            println!("up");
                        }
                        if arrow_down {
                            println!("down");
                        }
                    })
                },
                Mode::Redacting => {
                    ctx.input(|i| {
                        
                    })
                },
            }


        
            // console output
            if key_i {
                println!("order: {:?}", self.order);
                println!("clpiboard: {:?}", self.clipboard);
            }


            // if left mouse clicked
            if response.clicked_by(egui::PointerButton::Primary) {
                if let Some(pos) = response.interact_pointer_pos() {
                    let block_id = self.add_block([pos.x, pos.y], [200,200,200]);
                }
            }


            // if right mouse clicked
            if response.clicked_by(egui::PointerButton::Secondary) {
                if let Some(pos) = response.interact_pointer_pos() {
                }
            }



            // Render



            // render background color
            painter.rect_filled(
                egui::Rect::EVERYTHING,
                0.0,
                egui::Color32::from_rgb(10, 10, 10),
            );



            // render blocks
            for block_id in &self.order {
                let block = self.blocks.get_mut(block_id).unwrap();
                if self.active == Some(*block_id) {
                    block.color = [255,255,255];
                } else {
                    block.color = [155,155,155];
                }
                block.render_background(&painter,  egui::Pos2 { x:block.pos[0], y:block.pos[1]}, egui::Vec2{x:block.size[0],y:block.size[1]});
            }

        });
    }
}
