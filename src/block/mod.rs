use eframe::egui;
use serde::{Serialize, Deserialize};

mod file;
mod text;
mod img;

use file::File;
use text::Text;
use img::Img;



// get image proportions
pub fn get_ratio(path: &String) -> Option<[f32; 2]> {
    let data = std::fs::read(path).ok()?;
    let img = image::load_from_memory(&data).ok()?;
    let w = img.width();
    let h = img.height();
    let x = (w+h) as f32 / 2.0;
    Some([w as f32 / x, h as f32 / x])
}




// Block
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub img: Option<Img>,
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
            img: None,
            files: Vec::new(),
            text: Text::new(),

            pos,
            size: [350.,100.0],
            color,

            children: Vec::new(),
            parents: Vec::new(),
        }
    }


    // add new file
    pub fn add_file(&mut self, path: String) -> () {
        let ext = std::path::Path::new( &path )
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_lowercase();

        let is_image = matches!(
            ext.as_str(), 
            "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp");
            
        let size_mb = std::fs::metadata(&path)
            .map(|m| m.len() as f64 / 1024.0 / 1024.0)
            .unwrap_or(0.0);


        // if file can be displayed as an image    
        if is_image && let x = get_ratio(&path) && self.img.is_none() {
            self.img = Some(Img {
                path,
                size_mb,
                size: x.unwrap(),
            });
        // add a basic file
        } else {
            self.files.push(File {path, size_mb: 0} );
        }
    }


    // delete old file
    pub fn del_file(&mut self, path: String) -> () {
        self.files.retain(|f| f.path != path);
    }



    // render background
    pub fn render_background(&mut self, painter: &egui::Painter, pos: egui::Pos2, size: egui::Vec2) -> () {
        let rect= egui::Rect::from_min_size(pos, size);
        painter.rect_filled(rect, 0.0, egui::Color32::BLACK);
        painter.rect_stroke(rect, 0.0, 
            egui::Stroke::new(1.0, 
            egui::Color32::from_rgb(self.color[0], self.color[1], self.color[2])));
    }   


    pub fn render_files(&mut self, painter: &egui::Painter, pos: egui::Pos2, size: egui::Vec2) -> () {}


    pub fn render_text(&mut self, painter: &egui::Painter, pos: egui::Pos2, size: egui::Vec2) -> () {}



    // render block
    pub fn render(&mut self, painter: &egui::Painter, pos: egui::Pos2, size: egui::Vec2) -> () {
        let mut offset: u32 = 0;

        for file in &self.files {

        }


        // background draw

        // image draw

        // file draw

        // text draw

    }
}