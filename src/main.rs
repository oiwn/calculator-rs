#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{FontFamily, FontId, TextStyle};

mod calculator;

use calculator::{Calculator, Events};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(280.0, 380.0)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::new(Calculator::default())),
    );
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(18.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Monospace)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(5.0);
            configure_text_styles(ctx);

            if ui
                .add_enabled(false, egui::Button::new(self.display()))
                .clicked()
            {
                unreachable!();
            }

            ui.horizontal(|ui| {
                if ui.button("C").clicked() {
                    self.dispatch(Events::Reset);
                }
                if ui.button("±").clicked() {
                    self.dispatch(Events::Neg);
                }
                let _ = ui.button("(");
                let _ = ui.button(")");
            });
            ui.horizontal(|ui| {
                for num in 1..4 {
                    if ui.button(num.to_string()).clicked() {
                        self.dispatch(Events::Number(num));
                    }
                }
                if ui.button("+".to_string()).clicked() {
                    self.dispatch(Events::Add);
                }
            });
            ui.horizontal(|ui| {
                for num in 4..7 {
                    if ui.button(num.to_string()).clicked() {
                        self.dispatch(Events::Number(num));
                    }
                }
                if ui.button("-".to_string()).clicked() {
                    self.dispatch(Events::Sub);
                }
            });
            ui.horizontal(|ui| {
                for num in 7..10 {
                    if ui.button(num.to_string()).clicked() {
                        self.dispatch(Events::Number(num));
                    }
                }
                if ui.button("*".to_string()).clicked() {
                    self.dispatch(Events::Mul);
                }
            });
            ui.horizontal(|ui| {
                if ui.button("0".to_string()).clicked() {
                    self.dispatch(Events::Number(0));
                }
                if ui.button(".".to_string()).clicked() {
                    // float number ops
                }
                if ui.button("=".to_string()).clicked() {
                    self.dispatch(Events::Eq);
                }
                if ui.button("/".to_string()).clicked() {
                    self.dispatch(Events::Div);
                }
            });
        });
    }
}
