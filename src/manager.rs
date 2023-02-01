use crate::config::{CONFIG, CURRENT_FIELD, CURRENT_HIVE_SLOT};
use crate::fields::{
    change_bee_amount, change_field, change_hiveslot, start_field, FieldType, FIELD_VEC,
};
use crate::screen::SYS;
use eframe::egui;
use sysinfo::SystemExt;

pub fn start_manager() {
    unsafe {
        SYS.refresh_all();
    }
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(670.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "GatherThat Manager #232901",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    current_field: String,
    hive_slot: String,
    bees: String,
    used_memory: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_field: (*CURRENT_FIELD.to_string()).parse().unwrap(),
            hive_slot: (*CURRENT_HIVE_SLOT.to_string()).parse().unwrap(),
            bees: (&*CONFIG.bees).parse().unwrap(),
            used_memory: unsafe { SYS.used_memory().to_string() },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Current Field: ".to_string());
                ui.text_edit_singleline(&mut self.current_field);
                if ui.button("CHANGE_FIELD").clicked() {
                    let result = change_field(self.current_field.clone());

                    match result {
                        Ok(_value) => {
                            ui.label("WRITTEN");
                        }
                        Err(_err) => {
                            ui.label("INVALID");
                        }
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Current Hive Slot: ".to_string());
                ui.text_edit_singleline(&mut self.hive_slot);
                if ui.button("CHANGE_HIVE_SLOT").clicked() {
                    let result = change_hiveslot(self.hive_slot.clone());

                    match result {
                        Ok(_value) => {
                            ui.label("WRITTEN");
                        }
                        Err(_err) => {
                            ui.label("INVALID");
                        }
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Current Bee Amount: ".to_string());
                ui.text_edit_singleline(&mut self.bees);
                if ui.button("CHANGE_BEE_AMOUNT").clicked() {
                    change_bee_amount(self.bees.clone());
                    ui.label("WRITTEN");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("FIELD_TYPES").hovered() {
                    ui.label(format!("Allowed Fields: \n{:#?}", &*FIELD_VEC));
                }

                ui.add_space(5.0);
                if ui.button("HIVE_SLOTS").hovered() {
                    ui.label("From left(next to cannon) to right: 6,5,4,3,2,1".to_string());
                }
            });

            ui.horizontal_top(|ui| {
                if ui.button("START").clicked() {
                    let s: FieldType = self.current_field.clone().parse().expect("Parse Err");
                    start_field(&s, &CONFIG).expect("START_FIELD FUNCTION CALL ERROR");
                }

                ui.label(format!("{}: Bytes of RAM used", self.used_memory.clone()));
            });
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
