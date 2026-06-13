use eframe::egui::{self, Response};


// get input logs
fn _input_logs(ctx: &egui::Context) {
    ctx.input(|i| {
        for event in &i.events {
            println!("{:?}", event);
        }
    });
}

// all inputs
pub struct Input {
    pub ctrl_z: bool,
    pub ctrl_x: bool,
    pub ctrl_c: bool,
    pub ctrl_v: bool,

    pub arrow_left: bool,
    pub arrow_right: bool,
    pub arrow_up: bool,
    pub arrow_down: bool,

    pub key_i: bool,
    pub key_e: bool,

    pub enter: bool,
    pub esc: bool,

    pub mouse_pos: Option<egui::Pos2>,
    pub mouse_scroll: egui::Vec2,
    pub mouse_left: bool,
    pub mouse_right: bool,
}


impl Input {
    pub fn get(ctx: &egui::Context, response: &Response) -> Self {

        // UNCOMMENT for input logs
        // _input_logs(ctx);

        // short cuts
        let (ctrl_z, ctrl_x, ctrl_c, ctrl_v) = ctx.input_mut(|i| {
            let undo = i.key_pressed(egui::Key::Z) && i.modifiers.ctrl;
            let cut   = i.events.iter().any(|e| matches!(e, egui::Event::Cut));
            let copy  = i.events.iter().any(|e| matches!(e, egui::Event::Copy));
            let paste_released = i.events.iter().any(|e| matches!(e,
                egui::Event::Key { key: egui::Key::V, pressed: false, modifiers, .. }
                if modifiers.ctrl
            ));
            (undo, cut, copy, paste_released)
        });
        // arrows and keys
        let (arrow_left, arrow_right, arrow_up, arrow_down) = ctx.input(|i| (
            i.key_pressed(egui::Key::ArrowLeft),
            i.key_pressed(egui::Key::ArrowRight),
            i.key_pressed(egui::Key::ArrowUp),
            i.key_pressed(egui::Key::ArrowDown),
        ));
        // Keys
        let (key_e, key_i, enter, esc) = ctx.input(|i| (
            i.key_pressed(egui::Key::E),
            i.key_pressed(egui::Key::I),
            i.key_pressed(egui::Key::Enter),
            i.key_pressed(egui::Key::Escape),
        ));
        // mouse pos
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
        let mouse_scroll = ctx.input(|i| i.smooth_scroll_delta);
        let mouse_left = response.clicked_by(egui::PointerButton::Primary);
        let mouse_right = response.clicked_by(egui::PointerButton::Secondary);


        return Self {
            ctrl_z,
            ctrl_x,
            ctrl_c,
            ctrl_v,

            arrow_left,
            arrow_right,
            arrow_up,
            arrow_down,

            key_i,
            key_e,
            enter,
            esc,

            mouse_pos,
            mouse_scroll,
            mouse_left,
            mouse_right,
        };
    }
}


