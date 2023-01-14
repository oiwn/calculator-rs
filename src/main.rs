#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{FontFamily, FontId, TextStyle};

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

enum Events {
    Add,
    Sub,
    Mul,
    Div,
    Number(i64),
    Eq,
    Reset,
    Idle,
}

#[derive(Debug, PartialEq, Clone)]
enum Tokens {
    Add,
    Sub,
    Mul,
    Div,
    Number(i64),
}

struct Calculator {
    ops: Vec<Tokens>,
    accumulator: i64,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            ops: vec![],
            accumulator: 0,
        }
    }
}

fn shunting_yard(tokens: Vec<Tokens>) -> Vec<Tokens> {
    let mut output_queue = vec![];
    let mut operator_stack = vec![];

    for token in tokens {
        match token {
            Tokens::Number(n) => output_queue.push(Tokens::Number(n)),
            Tokens::Add | Tokens::Sub => {
                while let Some(top) = operator_stack.last() {
                    if *top == Tokens::Add || *top == Tokens::Sub {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token);
            }
            Tokens::Mul | Tokens::Div => {
                while let Some(top) = operator_stack.last() {
                    if *top == Tokens::Mul || *top == Tokens::Div {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token);
            }
        }
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    output_queue
}

impl Calculator {
    fn calculate(&mut self) -> i64 {
        println!("Ops: {:?}", self.ops.clone());
        println!("Algo: {:?}", shunting_yard(self.ops.clone()));
        let mut stack = vec![];

        for token in shunting_yard(self.ops.clone()) {
            match token {
                Tokens::Number(n) => stack.push(n),
                Tokens::Add => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();
                    stack.push(x + y);
                }
                Tokens::Sub => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();
                    stack.push(x - y);
                }
                Tokens::Mul => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();
                    stack.push(x * y);
                }
                Tokens::Div => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();
                    stack.push(x / y);
                }
            }
        }
        stack.pop().unwrap()
    }

    fn dispatch(&mut self, event: Events) {
        match event {
            Events::Idle => {}
            Events::Eq => {
                // here will be complex logic
                self.ops.push(Tokens::Number(self.accumulator));
                self.accumulator = self.calculate();
                self.ops.clear();
            }
            Events::Reset => {
                self.ops.clear();
                self.accumulator = 0;
            }
            Events::Number(num) => {
                if self.accumulator <= 999_999_999_9 {
                    self.accumulator *= 10;
                    self.accumulator += num as i64;
                }
            }
            op @ (Events::Add | Events::Sub | Events::Mul | Events::Div) => {
                // operation first
                let op_token: Option<Tokens> = match op {
                    Events::Add => Some(Tokens::Add),
                    Events::Sub => Some(Tokens::Sub),
                    Events::Mul => Some(Tokens::Mul),
                    Events::Div => Some(Tokens::Div),
                    _ => None,
                };

                self.ops.push(Tokens::Number(self.accumulator));
                self.ops.push(op_token.unwrap());
                self.accumulator = 0
            }
        }
    }
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
                .add_enabled(false, egui::Button::new(self.accumulator.to_string()))
                .clicked()
            {
                unreachable!();
            }

            ui.horizontal(|ui| {
                if ui.button("C").clicked() {
                    self.dispatch(Events::Reset);
                }
                if ui.button("±").clicked() {
                    self.dispatch(Events::Idle);
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
