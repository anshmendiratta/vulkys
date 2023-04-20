use eframe::egui;
use crate::type_traits::*;
use crate::rigidbodies;

#[derive(Debug)]
pub struct Content<T> where T: rigidbodies::Updateable + rigidbodies::HandleData<T> + std::fmt::Debug {
    objects: Vec<T>,
    mass: f64,
    radius: f64,
    position_x: f64,
    position_y: f64,
    velocity_x: f64,
    velocity_y: f64,
    angular_velocity: f64,
    selected: rigidbodies::RigidBodySelection,
}

// impl Default for RigidBody {}

impl<T> Default for Content<T> where T: rigidbodies::Updateable + rigidbodies::HandleData<T> + std::fmt::Debug {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            mass: 1.0,
            radius: 1.0,
            position_x: 0.0,
            position_y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            angular_velocity: 0.0,
            selected: rigidbodies::RigidBodySelection::None(0),
        }
    }
}

impl<T> eframe::App for Content<T> where T: rigidbodies::Updateable + rigidbodies::HandleData<T> + std::fmt::Debug {
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

            egui::ComboBox::from_label("Select object to add")
                .selected_text(String::from(self.selected.to_string()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected, rigidbodies::RigidBodySelection::Ball, "Ball")
                });

            ui.horizontal(|ui| {
                // let mut position_x: f64 = 0.0;
                ui.add(egui::Slider::new(&mut self.position_x, 0.0..=5.0));
                ui.label("STARTING X-COORDINATE");

                // let mut position_y: f64 = 0.0;
                ui.add(egui::Slider::new(&mut self.position_y, 0.0..=5.0));
                ui.label("STARTING Y-COORDINATE");
            });

            ui.horizontal(|ui| {
                // let mut velocity_x: f64 = 0.0;
                ui.add(egui::Slider::new(&mut self.velocity_x, 0.0..=5.0));
                ui.label("STARTING X-VELOCITY");

                // let mut velocity_y: f64 = 0.0;
                ui.add(egui::Slider::new(&mut self.velocity_y, 0.0..=5.0));
                ui.label("STARTING Y-VELOCITY");
            });

            ui.horizontal(|ui| {
                if ui.button("Add object").clicked() {
                    match self.selected {
                        rigidbodies::RigidBodySelection::Ball => self.objects.push(rigidbodies::Ball {
                            mass: self.mass,
                            radius: self.radius,
                            position: (self.position_x, self.position_y),
                            velocity: (self.velocity_x, self.velocity_y),
                            acceleration: (0.0, 0.0),
                            angular_velocity: self.angular_velocity,
                            parent: rigidbodies::RigidBody::default(),
                        }),
                        _ => (),
                    }

                    self.position_x = 0.0;
                    self.position_y = 0.0;
                    self.velocity_x = 0.0;
                    self.velocity_y = 0.0;
                }
            });

            ui.label(format!("{:?}", self.objects));

            ui.horizontal(|ui| {
                if ui.button("Run simulation").clicked() {
                    egui::Window::new("Simulation")
                } else {
                    egui::Window::new("Simulation")
                }
            })
        });
    }
}


