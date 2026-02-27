mod board;
mod gui;

use gui::OmasApp;

fn main() {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Omas Spiel",
        options,
        Box::new(|_cc| Box::new(OmasApp::default())),
    );
}
