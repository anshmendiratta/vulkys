use eframe::egui;
use rigidbodies::RigidBody;
use std::str::FromStr;
use strum_macros::EnumString;

mod boundary;
mod rigidbodies;
use crate::rigidbodies::*;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "HELLO!",
        options,
        Box::new(|_cc| Box::new(Content::default())),
    )
}

// enum SelectedRigidBody {
//     RigidBody::Ball,
//     RigidBody::Plane,
// }
#[derive()]
struct Content {
    objects: Vec<RigidBody>,
    position_x: f32,
    position_y: f32,
    velocity_x: f32,
    velocity_y: f32,
    selected: RigidBody,
}

// impl Default for RigidBody {}

impl Default for Content {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            position_x: 0.0,
            position_y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            selected: RigidBody::Ball,
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.horizontal(|ui| {
            //     ui.add(
            //         egui::ComboBox::from_label("Select object type to add to simulation")
            //             .selected_text(format!("{:?}", &self.selected))
            //             .show_ui(ui, |ui| {
            //                 ui.selectable_value(&mut self.selected, RigidBody::Ball, "Ball")
            //             }),
            //     )
            // });

            egui::ComboBox::from_label("Select object to add").show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected, RigidBody::Ball, "Ball")
            });

            ui.horizontal(|ui| {
                // let mut position_x: f32 = 0.0;
                ui.add(egui::Slider::new(&mut self.position_x, 0.0..=5.0));
                ui.label("STARTING X-COORDINATE");

                // let mut position_y: f32 = 0.0;
                ui.add(egui::Slider::new(&mut self.position_y, 0.0..=5.0));
                ui.label("STARTING Y-COORDINATE");
            });

            ui.horizontal(|ui| {
                // let mut velocity_x: f32 = 0.0;
                ui.add(egui::Slider::new(&mut self.velocity_x, 0.0..=5.0));
                ui.label("STARTING X-VELOCITY");

                // let mut velocity_y: f32 = 0.0;
                ui.add(egui::Slider::new(&mut self.velocity_y, 0.0..=5.0));
                ui.label("STARTING Y-VELOCITY");
            });

            ui.horizontal(|ui| {
                if ui.button("ADD OBJECT").clicked() {
                    self.objects.push(self.selected);

                    self.position_x = 0.0;
                    self.position_y = 0.0;
                    self.velocity_x = 0.0;
                    self.velocity_y = 0.0;
                }
                ui.label(format!("{:?}", self.objects))
            });
        });
    }
}
