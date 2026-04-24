//! Qaren GUI — Desktop configuration comparison tool.
//!
//! Launches the eframe/egui native window with the `QarenApp`.

mod app;
mod masking;
mod settings;
mod theme;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_min_inner_size([800.0, 600.0])
            .with_title("Qaren — Configuration Comparator"),
        ..Default::default()
    };

    eframe::run_native(
        "Qaren",
        options,
        Box::new(|cc| Ok(Box::new(app::QarenApp::new(cc)))),
    )
}
