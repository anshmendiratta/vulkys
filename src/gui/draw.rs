use crate::FVec2;
use std::path::Path;

use crate::physics::circle::Circle;
use crate::physics::rigidbody::GenericObject;
use crate::physics::rigidbody::RigidBody;
use crate::physics::rigidbody::RigidBodySelection;
use eframe::egui;
use egui::RichText;
use serde::Serialize;
use tracing::info;

#[derive(Serialize, Clone)]
pub struct Content {
    #[serde(skip_serializing)]
    pub sim_button_clicked: bool,
    #[serde(skip_serializing)]
    selected: RigidBodySelection,
    #[serde(skip_serializing)]
    next_object_id: u8,
    objects: Vec<RigidBody>,
    #[serde(skip_serializing)]
    radius: f32,
    #[serde(skip_serializing)]
    position_x: f32,
    #[serde(skip_serializing)]
    position_y: f32,
    #[serde(skip_serializing)]
    velocity_x: f32,
    #[serde(skip_serializing)]
    velocity_y: f32,
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
            selected: RigidBodySelection::None,
            sim_button_clicked: false,
            next_object_id: 0,
        }
    }
}

struct ObjectInfo {
    obj_type: RigidBodySelection,
    radius: f32,
    position_x: f32,
    position_y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl Content {
    fn add_object(&mut self, obj_params: ObjectInfo) {
        match obj_params.obj_type {
            RigidBodySelection::Circle_ => {
                let complete_circle: Circle = Circle {
                    radius: obj_params.radius,
                    position: FVec2::new(obj_params.position_x, obj_params.position_y),
                    velocity: FVec2::new(obj_params.velocity_x, obj_params.velocity_y),
                };
                self.objects
                    .push(RigidBody::Circle_(complete_circle, self.next_object_id))
            }
            RigidBodySelection::None => (),
        };

        self.next_object_id += 1;
    }
    fn reset_values(&mut self) {
        self.radius = 0.;
        self.position_x = 0.;
        self.position_y = 0.;
        self.velocity_x = 0.;
        self.velocity_y = 0.;
    }
    fn flush_to_csv(&mut self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self)?;
        std::fs::write(Path::new("objects.json"), json)?;
        self.reset_values();

        Ok(())
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.sim_button_clicked {
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select object to add")
                .selected_text(self.selected.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected, RigidBodySelection::Circle_, "Circle")
                });

            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.position_x, -1.0..=1.0));
                ui.label("STARTING X-COORDINATE");

                ui.add(egui::Slider::new(&mut self.position_y, -0.0..=1.0));
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
                    if self.selected == RigidBodySelection::None {
                        return;
                    };
                    let obj_params: ObjectInfo = ObjectInfo {
                        obj_type: self.selected,
                        radius: self.radius,
                        position_x: self.position_x,
                        position_y: self.position_y,
                        velocity_x: self.velocity_x,
                        velocity_y: self.velocity_y,
                    };
                    self.add_object(obj_params);
                    self.reset_values();
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Run simulation").clicked() {
                    self.sim_button_clicked = true;
                    info!("Run simulation button pressed");
                    let _ = self.flush_to_csv();
                }
            });
        });

        egui::SidePanel::right("Objects added").show(ctx, |ui| {
            ui.heading("Objects added");
            ui.label(RichText::new(""));
            for obj in &mut *self.objects {
                ui.label(RichText::new(format!("{:?}", obj.get_object().get_debug())));
            }
        });
    }
}
