use eframe::egui;
use serde::{Serialize, Deserialize};

mod file;
mod text;
mod image;

use file::File;
use text::Text;
use image::Image;


// Block on the whiteboard
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub images: Vec<Image>,
    pub files: Vec<File>,
    pub text: Text,

    pub pos: [f32;2],
    pub size: [f32;2],
    pub color: [u8;3],

    pub children: Vec<u64>,
    pub parents: Vec<u64>,

}


impl Block {
    // create empty Block
    pub fn new(pos: [f32;2], color: [u8;3]) -> Self {
        Block {
            images: Vec::new(),
            files: Vec::new(),
            text: Text::new(),

            pos,
            size: [0.0,0.0],
            color,

            children: Vec::new(),
            parents: Vec::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> () {
        let painter = ui.painter();

        let mut offset = 0;

        // background draw

        // image draw

        // file draw

        // text draw

    }
}