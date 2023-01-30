use std::fs::File;
use std::ops::Deref;
use eframe::egui;
use toml::Value;
use crate::config::CURRENT_FIELD;
use crate::fields::{change_field, FIELD_VEC, FieldType, start_field};

pub fn start_manager() {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "GatherThat Manager #232901",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    current_field: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_field: (&*CURRENT_FIELD.to_string()).parse().unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Current Field: "));
                ui.text_edit_singleline(&mut self.current_field);
                if ui.button("CHANGE_FIELD").clicked() {
                    change_field(self.current_field.clone());
                }
            });

            ui.vertical(|ui| {
                if ui.button("FIELD_TYPES").hovered() {
                    ui.label(format!("Allowed Fields: \n{:?}", &*FIELD_VEC));
                }
            });

            ui.label(" ");

            ui.label("H")


        });
    }
}

// ui.heading("My egui Application");
// ui.horizontal(|ui| {
// let name_label = ui.label("Your name: ");
// ui.text_edit_singleline(&mut self.current_field)
// .labelled_by(name_label.id);
// });
// ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
// if ui.button("Click each year").clicked() {
// self.age += 1;
// }
// ui.label(format!("Hello '{}', age {}", self.current_field, self.age));
