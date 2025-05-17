pub mod app;
use std::sync::Arc;

use eframe::egui::{
    self, FontData, FontDefinitions,
    FontFamily::{Monospace, Proportional},
    FontId, TextStyle,
};

pub fn set_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::empty();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    let font_buf = include_bytes!("../../example/MiSans-Demibold.ttf");
    let font = FontData::from_static(font_buf);
    fonts.font_data.insert("my_font".to_owned(), Arc::new(font));

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(66.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Proportional)),
        (TextStyle::Button, FontId::new(16.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
