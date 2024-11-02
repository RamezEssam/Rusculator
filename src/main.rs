#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod calculator;

use std::collections::VecDeque;
use std::sync::Arc;
use eframe::egui::{self};
use egui::{Button, IconData, Label, TextEdit, Vec2, ViewportBuilder};
use egui::Visuals;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::{Monospace, Body, Small, Heading};
use std::io::Cursor;
use image::io::Reader as ImageReader;



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


            let _ = ui.add_sized(
                Vec2::new(ui.available_width(), ui.available_height()),
                Label::new(self.answer.clone())
            );
                

            if  calc_button.clicked(){
                if self.text.len() > 0 {
                    self.answer = calculator::calculate(&self.text)
                }
                
            }
        });
    }
}


fn main() -> eframe::Result<()> {

    let logo_data = include_bytes!(r"..\assets\ruscalculator-logo.jpg");
    let logo_image = ImageReader::new(Cursor::new(logo_data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();

    let (width, height) = logo_image.dimensions();
    let pixels = logo_image.into_raw();



    let options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            icon: Some(Arc::new(
                IconData {
                    rgba: pixels,
                    width: width,
                    height: height,
                }
            )),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Rusculator",
        options,
        Box::new(|_cc| Box::new(Content::default())),
    )
}
