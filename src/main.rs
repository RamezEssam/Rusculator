#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release



use std::collections::VecDeque;
use regex::Regex;
use eframe::egui;
use egui::{Button, Label, RichText, TextEdit, Vec2};
use egui::Visuals;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::Layout;
use egui::Align;
use egui::TextStyle::{Monospace, Body, Small, Heading};


#[derive(Debug)]
enum Token {
    Number(f64),
    Operator(char),
    Function(String),
    OpenParen,
    CloseParen,
}


fn tokenize(expression: &str) -> Vec<Token> {
    let re = Regex::new(r"\d+\.?\d*|[+\-^*/()]|cos|sin|tan").unwrap();
    let mut tokens: Vec<Token> = Vec::new();

    for cap in re.captures_iter(expression) {
        let part = cap.get(0).unwrap().as_str();
        tokens.push(
            match part {
                "(" => Token::OpenParen,
                ")" => Token::CloseParen,
                "+" | "-" | "*" | "/" | "^" => Token::Operator(part.chars().next().unwrap()),
                "cos"| "sin" | "tan" => Token::Function(part.to_string()),
                _ => Token::Number(part.parse().unwrap()),
            }
        );
    }

    tokens
}


fn shunting_yard(tokens: Vec<Token>) -> VecDeque<Token> {
    let mut output = VecDeque::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push_back(token),
            Token::Function(_) => operators.push(token),
            Token::Operator(op) => {
                while let Some(Token::Operator(top_op)) = operators.last() {
                    if precedence(op) <= precedence(*top_op) {
                        output.push_back(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::OpenParen => operators.push(token),
            Token::CloseParen => {
                while let Some(top) = operators.pop() {
                    if let Token::OpenParen = top {
                        break;
                    } else {
                        output.push_back(top);
                    }
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push_back(op);
    }

    output
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}


fn evaluate_rpn(queue: VecDeque<Token>) -> f64 {
    let mut stack = Vec::new();

    for token in queue {
        match token {
            Token::Number(num) => stack.push(num),
            Token::Operator(op) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    '^' => a.powf(b),
                    _ => panic!("Unsupported operator"),
                };
                stack.push(result);
            }
            Token::Function(func) => {
                let value = stack.pop().unwrap();
                let result = match func.as_str() {
                    "cos" => value.to_radians().cos(),
                    "sin" => value.to_radians().sin(),
                    "tan" => value.to_radians().tan(),
                    _ => panic!("Unsupported function"),
                };
                stack.push(result);
            }
            _ => {}
        }
    }

    stack.pop().unwrap()
}


fn calculate(expr: &str) -> String {
    let tokens = tokenize(expr);
    let rpn = shunting_yard(tokens);
    let result = evaluate_rpn(rpn);
    format!("{}", result)
}

#[derive(Default)]
struct Content {
    text: String,
    answer: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Setting Dark theme
        ctx.set_visuals(Visuals::dark());

        // Setting Font
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (egui::TextStyle::Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);
        

        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.add(
                TextEdit::multiline(&mut self.text)
                        .desired_rows(10)
                        .desired_width(ctx.available_rect().width())
                        
            );

            let calc_button = ui.add_sized(
                Vec2::new(ui.available_width(), 55.0),
                Button::new("Calculate")
            );


            let display = ui.add_sized(
                Vec2::new(ui.available_width(), ui.available_height()),
                Label::new(self.answer.clone())
            );
                

            if  calc_button.clicked(){
                if self.text.len() > 0 {
                    self.answer = calculate(&self.text)
                }
                
            }
        });
    }
}


fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusculator",
        options,
        Box::new(|_cc| Ok(Box::<Content>::default())),
    )
}
