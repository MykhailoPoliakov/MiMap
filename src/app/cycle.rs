
use eframe::egui;
use crate::app::{App, board::{Direction::{Down, Left, Right, Up}, Mode}, input::Input};





impl App {
    fn input(&mut self, ctx: &egui::Context, response: &egui::Response) -> () {

        // get input
        let input = Input::get(ctx, &response); 

        // UNIVERAL INPUTS

        // console output
        if input.key_i {
            println!("order: {:?}", self.board.order);
            println!("clpiboard: {:?}", self.clipboard);
        }

        // INPUTS FOR MODE

        // match the mode
        match self.board.mode {
            Mode::Looking => {

                // undo self.board | Crlr + Z
                if input.ctrl_z {
                    if !self.history.is_empty() {
                        self.board = self.history.pop().unwrap();
                    }
                }

                // cut the block | Ctrl + X
                if input.ctrl_x {
                    if let Some(x) = self.board.active {
                        let block = self.board.blocks[&x].clone();
                        self.clipboard = Some(block);
                        self.board.del_block(x);
                    }
                }
                // copy the block | Ctrl + C
                if input.ctrl_c {
                    if let Some(x) = self.board.active {
                        self.clipboard = Some(self.board.blocks[&x].clone());
                    }
                }
                // paste the block | Ctrl + V
                if input.ctrl_v {
                    if let Some(x) = self.clipboard.clone() {
                        let pos = input.mouse_pos.unwrap_or(response.rect.center());
                        self.board.ins_block(&x, [pos.x,pos.y]);   
                    }
                }

                // Arrows
                if input.arrow_left {
                    self.board.select_near_block(Left);
                }
                if input.arrow_right {
                    self.board.select_near_block(Right);
                }
                if input.arrow_up {
                    self.board.select_near_block(Up);
                }
                if input.arrow_down {
                    self.board.select_near_block(Down);
                }

                // Keys
                if input.enter {
                    if let Some(_) = self.board.active {
                        self.board.mode = Mode::Redacting; 
                        self.board.active_on_top();
                    }
                }


                // mouse left
                if input.mouse_left {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.board.add_block([pos.x, pos.y], [200,200,200]);
                    }
                }

                // mouse right
                if input.mouse_right {
                    if let Some(pos) = response.interact_pointer_pos() {
                    }
                }

                // mouse scroll
                if input.mouse_scroll.y != 0. {
                    let factor = if input.mouse_scroll.y > 0. { 1.1 } else { 0.9 };
                    if let Some(mouse_pos) = input.mouse_pos {
                        self.camera.zoom(mouse_pos, factor);
                    }
                    
                }
            },
            Mode::Redacting => {
                // Keys
                if input.esc {
                    self.board.mode = Mode::Looking; 
                }
            },
        }
    }



    fn render(&mut self, painter: &egui::Painter) -> () {

        // render background color
        painter.rect_filled(
            egui::Rect::EVERYTHING,
            0.0,
            egui::Color32::from_rgb(10, 10, 10),
        );

        // render blocks
        for block_id in &self.board.order {
            let block = self.board.blocks.get_mut(block_id).unwrap();
            if self.board.active == Some(*block_id) {
                block.color = [255,255,255];
            } else {
                block.color = [100,100,100];
            }
            block.render_background(&painter,  
                self.camera.convert_pos(block.pos), 
                self.camera.convert_size(block.size));
        }
    }
}




impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            // Input
            self.input(ctx, &response);

            
            // Render
            self.render(&painter);

        });
    }
}