use eframe::egui;

mod board;
mod block;


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("MiMap")
            .with_inner_size([1280.0, 920.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MiMap",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

#[derive(Default)]
struct App {
    shapes: Vec<[f32; 4]>, // x, y, w, h
    drawing: Option<[f32; 2]>,
}





impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            let rect = response.rect;

            // background
            painter.rect_filled(rect, 0.0, egui::Color32::BLACK);

            // mouse input
            if response.drag_started() {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.drawing = Some([pos.x, pos.y]);
                }
            }

            if response.drag_released() {
                if let Some(start) = self.drawing.take() {
                    if let Some(end) = response.interact_pointer_pos() {
                        let x = start[0].min(end.x);
                        let y = start[1].min(end.y);
                        let w = (start[0] - end.x).abs();
                        let h = (start[1] - end.y).abs();

                        self.shapes.push([x, y, w, h]);
                    }
                }
            }

            // draw shapes
            for s in &self.shapes {
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(s[0], s[1]),
                        egui::vec2(s[2], s[3]),
                    ),
                    0.0,
                    egui::Color32::RED,
                );
            }
        });
    }
}