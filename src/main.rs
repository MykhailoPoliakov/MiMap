use eframe::egui;


mod block;
mod app;

use app::App;


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