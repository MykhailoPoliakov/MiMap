use eframe::egui;



pub struct Camera {
    pub offset: [f32;2],
    pub zoom: f32,
}


impl Default for Camera {
    fn default() -> Self {
        Self {
            offset: [0.,0.],
            zoom: 1.0,
        }
    }
}


impl Camera {

    pub fn convert_pos(&self, pos: [f32;2]) -> egui::Pos2 {
        let conv_x = (pos[0] + self.offset[0]) * self.zoom;
        let conv_y = (pos[1] + self.offset[1]) * self.zoom;
        return egui::Pos2 { x: conv_x, y: conv_y}
    }

    pub fn convert_size(&self, size: [f32;2]) -> egui::Vec2 {
        let conv_w = size[0] * self.zoom; 
        let conv_h = size[1] * self.zoom; 
        return egui::Vec2 { x: conv_w, y: conv_h}
    }

    pub fn reverse_pos(&self, pos: egui::Pos2) -> [f32;2] {
        let rev_x = (pos[0] / self.zoom) - self.offset[0];
        let rev_y = (pos[1] / self.zoom) - self.offset[0];
        return [rev_x, rev_y]
    }

    pub fn pan(&mut self, delta: [f32;2]) -> () {}

    pub fn zoom(&mut self, mouse_pos: egui::Pos2, factor: f64) -> () {
        
    }



}
