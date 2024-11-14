use crate::physics::circle::Circle;
use crate::physics::rigidbody::GenericObject;
use crate::physics::rigidbody::RigidBody;
use crate::physics::rigidbody::RigidBodySelection;
use eframe::egui;
use egui::{RichText, Vec2};
use tracing::info;

pub struct Content {
    objects: Vec<RigidBody>,
    radius: f32,
    position_x: f32,
    position_y: f32,
    velocity_x: f32,
    velocity_y: f32,
    selected: Option<RigidBodySelection>,
    sim_button_clicked: bool,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            radius: 1.0,
            position_x: 0.0,
            position_y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            selected: None,
            sim_button_clicked: false,
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select object to add")
                .selected_text(
                    self.selected
                        .as_ref()
                        .expect("no selected body")
                        .to_string(),
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.selected.as_ref().unwrap(),
                        &RigidBodySelection::Circle_,
                        "Ball",
                    )
                });

            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.position_x, 0.0..=5.0));
                ui.label("STARTING X-COORDINATE");

                ui.add(egui::Slider::new(&mut self.position_y, 0.0..=5.0));
                ui.label("STARTING Y-COORDINATE");
            });

            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.velocity_x, 0.0..=5.0));
                ui.label("STARTING X-VELOCITY");

                ui.add(egui::Slider::new(&mut self.velocity_y, 0.0..=5.0));
                ui.label("STARTING Y-VELOCITY");
            });

            ui.horizontal(|ui| {
                if ui.button("Add object").clicked() {
                    match self.selected {
                        Some(RigidBodySelection::Circle_) => {
                            let complete_object = Circle {
                                radius: self.radius,
                                position: Vec2::new(self.position_x, self.position_y),
                                velocity: Vec2::new(self.velocity_x, self.velocity_y),
                            };
                            self.objects.push(RigidBody::Circle_(complete_object));
                        }
                        _ => (),
                    }

                    self.position_x = 0.0;
                    self.position_y = 0.0;
                    self.velocity_x = 0.0;
                    self.velocity_y = 0.0;
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Run simulation").clicked() {
                    self.sim_button_clicked = true;
                    info!("run simulation button pressed");
                }
            })
        });

        egui::SidePanel::right("Objects added").show(ctx, |ui| {
            ui.heading("Objects added");
            ui.label(RichText::new(""));
            for obj in &mut *self.objects {
                ui.label(RichText::new(format!("{:?}", obj.get_inner().get_debug())));
            }
        });
    }
}
