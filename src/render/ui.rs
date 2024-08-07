use crate::physics::rigidbodies;
use eframe::egui;
use egui::RichText;
use tracing::info;

pub struct Content {
    objects: Vec<Box<dyn rigidbodies::HandleData>>,
    mass: f64,
    radius: f64,
    position_x: f64,
    position_y: f64,
    velocity_x: f64,
    velocity_y: f64,
    angular_velocity: f64,
    selected: rigidbodies::RigidBodySelection,
    sim_button_clicked: bool,
}

impl Default for Content {
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
            selected: rigidbodies::RigidBodySelection::None,
            sim_button_clicked: false,
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select object to add")
                .selected_text(self.selected.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.selected,
                        rigidbodies::RigidBodySelection::Ball,
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
                        rigidbodies::RigidBodySelection::Ball => {
                            self.objects.push(Box::new(rigidbodies::Ball {
                                mass: self.mass,
                                radius: self.radius,
                                position: (self.position_x, self.position_y),
                                velocity: (self.velocity_x, self.velocity_y),
                                acceleration: (0.0, 0.0),
                                angular_velocity: self.angular_velocity,
                            }))
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
                ui.label(RichText::new(format!("{:?}", obj.get_debug_print())));
            }
        });
    }
}
