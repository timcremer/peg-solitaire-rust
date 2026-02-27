use crate::board::Board;
use eframe::egui;

pub struct OmasApp {
    board: Board,
    selected: Option<(usize, usize)>,
}

impl Default for OmasApp {
    fn default() -> Self {
        Self {
            board: Board::new(),
            selected: None,
        }
    }
}

impl eframe::App for OmasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Omas Spiel");
            ui.label(format!("Pegs remaining: {}", self.board.pegs_remaining()));

            let button_size = egui::vec2(40.0, 40.0);

            for r in 0..self.board.size() {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.spacing_mut().item_spacing.y = 0.0;
                    
                    for c in 0..self.board.size() {
                        match self.board.cell(r, c) {
                            None => {
                                ui.add_enabled(false, egui::Button::new("").min_size(button_size));
                            }
                            Some(true) => {
                                let mut btn = egui::Button::new("🟢").min_size(button_size);
                                if self.selected == Some((r, c)) {
                                    btn = btn.fill(egui::Color32::LIGHT_BLUE);
                                }
                                if ui.add(btn).clicked() {
                                    if let Some((sr, sc)) = self.selected {
                                        if self.board.move_peg(sr, sc, r, c) {}
                                        self.selected = None;
                                    } else {
                                        self.selected = Some((r, c));
                                    }
                                }
                            }
                            Some(false) => {
                                if ui
                                    .add(egui::Button::new("○").min_size(button_size))
                                    .clicked()
                                {
                                    if let Some((sr, sc)) = self.selected {
                                        if self.board.move_peg(sr, sc, r, c) {}
                                        self.selected = None;
                                    }
                                }
                            }
                        }
                    }
                });
            }
        });
    }
}
