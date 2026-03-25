use eframe::egui;
use egui::{RichText, ScrollArea};

#[derive(Default)]
struct Rubsidian {
    text: String,
}

impl eframe::App for Rubsidian {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let width = ui.available_width() * 0.5;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Editor pane");
                    let editor = egui::TextEdit::multiline(&mut self.text)
                        .desired_rows(50)
                        .lock_focus(true)
                        .frame(true);
                    ui.add_sized([width, ui.available_height()], editor);
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label("Preview Pane");
                    ScrollArea::vertical().show(ui, |ui| {
                        for line in self.text.lines() {
                            let sep_test = line.trim();
                            if sep_test == "---" {
                                ui.separator();
                                continue;
                            }

                            let trimmings = line.trim_start();
                            let heading_level = trimmings.chars().take_while(|c| *c == '#').count();
                            if heading_level >= 1 && heading_level <= 6 {
                                let rest = &trimmings[heading_level..];
                                if rest.starts_with(' ') {
                                    let content = rest[1..].trim();
                                    match heading_level {
                                        1 => {
                                            ui.label(RichText::new(content).size(32.0).strong());
                                        }
                                        2 => {
                                            ui.label(RichText::new(content).size(28.0).strong());
                                        }
                                        3 => {
                                            ui.label(RichText::new(content).size(24.0).strong());
                                        }
                                        4 => {
                                            ui.label(RichText::new(content).size(22.0).strong());
                                        }
                                        5 => {
                                            ui.label(RichText::new(content).size(20.0).strong());
                                        }
                                        _ => {
                                            ui.label(RichText::new(content).size(18.0).strong());
                                        }
                                    }
                                    continue;
                                }
                            }

                            if line.trim().is_empty() {
                                ui.add_space(7.0);
                            } else {
                                ui.label(line);
                            }
                        }
                    })
                })
            })
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rubsidian text editor",
        native_options,
        Box::new(|_cc| Ok(Box::new(Rubsidian::default()))),
    )
}
