mod cycle;
mod input;
mod board;
mod camera;

use board::Board;
use camera::Camera;
use crate::{block::Block};


#[derive(Default)]
pub struct App {
    board: Board,
    history: Vec<Board>,

    clipboard: Option<Block>,

    camera: Camera,
}

impl App {

    // undo to last saved board
    pub fn undo(&mut self) -> () {
        if let Some(board) = self.history.pop() {
            self.board = board;
        }
    }

    // save the board in history
    pub fn save(&mut self) -> () {
        self.history.push(self.board.clone());
    }

    pub fn global_save() -> () {
        
    }
}