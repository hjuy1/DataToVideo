#![allow(unused)]
use super::set_fonts;
use crate::{
    color::Color,
    init::Operation,
    slide::{Position, Slide},
    video::{Video, VideoConfigBuilder},
    Result,
};
use ab_glyph::FontArc;
use eframe::egui;
use std::{fs, path::PathBuf};

pub struct MyApp {
    pub operation: Vec<Operation>,
    pub selected_var: String,
    pub input_fields: Vec<String>,
    pub screen: (String, String),
    pub fps: String,
    pub work_dir: String,
    pub back_color: String,
    pub cover_sec: String,
    pub ending_sec: String,
    pub swip_pixels_per_sec: String,
    pub width_slides: String,
    pub save_path: String,
    pub step: String,
    pub font: String,
    pub frame: u32,
    pub preview: bool,
    pub output: Vec<String>,
}

#[derive(PartialEq, Debug)]
enum Operation1 {
    Image(i32),
    Text,
    Color,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        set_fonts(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {
            operation: vec![],
            selected_var: "".to_string(),
            input_fields: vec![],
            screen: ("1920".to_string(), "1080".to_string()),
            fps: "60".to_string(),
            work_dir: "".to_string(),
            back_color: "white".to_string(),
            cover_sec: "4".to_string(),
            ending_sec: "4".to_string(),
            swip_pixels_per_sec: "160".to_string(),
            width_slides: "480".to_string(),
            save_path: "".to_string(),
            step: "20".to_string(),
            font: "".to_string(),
            frame: 0,
            preview: false,
            output: vec![],
        }
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("screen: ");
            ui.add(egui::TextEdit::singleline(&mut self.screen.0).desired_width(50.0));
            ui.add(egui::TextEdit::singleline(&mut self.screen.1).desired_width(50.0));
        });
        ui.horizontal(|ui| {
            let fps_label = ui.label("fps: ");
            ui.add(egui::TextEdit::singleline(&mut self.fps).desired_width(50.0))
                .labelled_by(fps_label.id);
        });
        ui.horizontal(|ui| {
            ui.label("work_dir: ");
            ui.add(egui::TextEdit::singleline(&mut self.work_dir).hint_text("default: ./work"));
        });
        ui.horizontal(|ui| {
            ui.label("back_color: ");
            ui.add(egui::TextEdit::singleline(&mut self.back_color).desired_width(100.0));
        });
        ui.horizontal(|ui| {
            ui.label("cover_sec: ");
            ui.add(egui::TextEdit::singleline(&mut self.cover_sec).desired_width(50.0));
        });
        ui.horizontal(|ui| {
            ui.label("ending_sec: ");
            ui.add(egui::TextEdit::singleline(&mut self.ending_sec).desired_width(50.0));
        });
        ui.horizontal(|ui| {
            ui.label("swip_pixels_per_sec: ");
            ui.add(egui::TextEdit::singleline(&mut self.swip_pixels_per_sec).desired_width(50.0));
        });
        ui.horizontal(|ui| {
            ui.label("width_slides: ");
            ui.add(egui::TextEdit::singleline(&mut self.width_slides).desired_width(50.0));
        });
        ui.horizontal(|ui| {
            ui.label("save_path: ");
            ui.add(
                egui::TextEdit::singleline(&mut self.save_path)
                    .hint_text("default: work_dir/output.mp4"),
            );
        });
        ui.horizontal(|ui| {
            ui.label("step: ");
            ui.add(egui::TextEdit::singleline(&mut self.step).desired_width(50.0));
        });
        ui.horizontal(|ui| {
            ui.label("font: ");
            ui.add(egui::TextEdit::singleline(&mut self.font));
        });
    }

    pub fn to_config(&self) -> Result<VideoConfigBuilder> {
        let work_dir = if self.work_dir.is_empty() {
            None
        } else {
            Some(PathBuf::from(&self.work_dir))
        };
        let save_path = if self.save_path.is_empty() {
            None
        } else {
            Some(PathBuf::from(&self.save_path))
        };
        let font = if self.font.is_empty() {
            None
        } else {
            Some(PathBuf::from(&self.font))
        };
        Ok(VideoConfigBuilder {
            screen: (self.screen.0.parse()?, self.screen.1.parse()?),
            fps: self.fps.parse()?,
            work_dir,
            back_color: self.back_color.clone(),
            cover_sec: self.cover_sec.parse()?,
            ending_sec: self.ending_sec.parse()?,
            swip_pixels_per_sec: self.swip_pixels_per_sec.parse()?,
            width_slides: self.width_slides.parse()?,
            save_path,
            step: self.step.parse()?,
            font,
        })
    }

    pub fn preview(&self) -> Result<()> {
        let font_buf = fs::read(&self.font).unwrap_or_else(|_| {
            let font = std::env::current_dir()
                .unwrap()
                .join("example")
                .join("MiSans-Demibold.ttf");
            fs::read(font).unwrap()
        });
        let font = FontArc::try_from_vec(font_buf)?;
        Slide::new_default(1)
            .render(self.width_slides.parse()?, self.screen.1.parse()?, &font)?
            .save("preview.png")?;
        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        let font_buf = fs::read(&self.font)?;
        let font = FontArc::try_from_vec(font_buf)?;
        Video::new(
            vec![
                Slide::new_default(1),
                Slide::new_default(1),
                Slide::new_default(1),
                Slide::new_default(1),
                Slide::new_default(1),
                Slide::new_default(1),
                Slide::new_default(1),
                Slide::new_default(1),
            ],
            self.to_config()?.build(),
        )
        .run()?;
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        let mut myenum = Operation1::Text;
        egui::SidePanel::left("left").show(ctx, |ui| {
            ui.heading("Slide元素");
            ui.add_space(50.0);
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("选择操作:")
                    .selected_text(&self.selected_var)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_var, "图片".to_string(), "图片");
                        ui.selectable_value(&mut self.selected_var, "文字".to_string(), "文字");
                        ui.selectable_value(&mut self.selected_var, "背景色".to_string(), "背景色");
                    });
                let num_field = match self.selected_var.as_str() {
                    "图片" => 3,
                    "文字" => 7,
                    "背景色" => 6,
                    _ => 1,
                };
                if self.input_fields.len() != num_field {
                    self.input_fields.resize(num_field, "".to_string());
                }
                for field in self.input_fields.iter_mut() {
                    ui.add(egui::TextEdit::singleline(field).desired_width(50.0));
                }
                if ui.button("添加").clicked() {
                    let inputs: Vec<&str> = self.input_fields.iter().map(|s| s.trim()).collect();

                    match self.selected_var.as_str() {
                        "图片" if inputs.len() == 3 => {
                            self.operation.push(Operation::Image(Position::new(
                                inputs[0].parse().unwrap(),
                                inputs[1].parse().unwrap(),
                                inputs[2].parse().unwrap(),
                            )));
                        }
                        "文字" if inputs.len() == 7 => {
                            self.operation.push(Operation::Text(
                                inputs[0].parse().unwrap(),
                                Color([
                                    inputs[1].parse().unwrap(),
                                    inputs[2].parse().unwrap(),
                                    inputs[3].parse().unwrap(),
                                ]),
                                Position::new(
                                    inputs[4].parse().unwrap(),
                                    inputs[5].parse().unwrap(),
                                    inputs[6].parse().unwrap(),
                                ),
                            ));
                        }
                        "背景色" if inputs.len() == 6 => {
                            self.operation.push(Operation::Color(
                                Color([
                                    inputs[0].parse().unwrap(),
                                    inputs[1].parse().unwrap(),
                                    inputs[2].parse().unwrap(),
                                ]),
                                Position::new(
                                    inputs[3].parse().unwrap(),
                                    inputs[4].parse().unwrap(),
                                    inputs[5].parse().unwrap(),
                                ),
                            ));
                        }
                        _ => {}
                    }
                }
            });
            ui.add_space(50.0);
            let mut del = -1;
            for (index, op) in self.operation.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{:?}", op));
                    if ui.button("Delet").clicked() {
                        del = index as i32;
                    };
                });
            }
            if del != -1 {
                self.operation.remove(del as usize);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("视频参数");

            self.show_content(ui);

            ui.add_space(50.0);

            ui.horizontal(|ui| ui.label(self.frame.to_string()));

            ui.add_space(50.0);

            ui.horizontal(|ui| {
                if ui.button("测试").clicked() {
                    match self.preview() {
                        Ok(_) => {
                            self.preview = true;
                        }
                        Err(e) => {
                            self.output.push(e.to_string());
                        }
                    };
                }

                if ui.button("生成").clicked() {
                    if let Err(e) = self.run() {
                        self.output.push(e.to_string());
                    };
                }
            });

            ui.add_space(50.0);

            for i in self.output.iter() {
                ui.label(i);
            }
        });
        egui::SidePanel::right("right").show(ctx, |ui| {
            ui.heading("预览: ");
            if self.preview {
                ui.image("file://preview.png");
            }
        });
        self.frame += 1;
    }
}
