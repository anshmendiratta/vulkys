// use crate::type_traits::*;
// use crate::painter;
use crate::rigidbodies;
use eframe::egui;

/// Where all data is stored.
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
}

/// Obligatory default for start-up.
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
            selected: rigidbodies::RigidBodySelection::None(0),
        }
    }
}

/// Implementing the App.
impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            /// Adding the box, which when clicked on grants you the options of rigid bodies to add.
            egui::ComboBox::from_label("Select object to add")
                .selected_text(String::from(self.selected.to_string()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.selected,
                        rigidbodies::RigidBodySelection::Ball,
                        "Ball",
                    )
                });

            /// Adding the sliders for the position and velocity.
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

            /// Displaying a button to add the rigid body to the simulation.
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
                                parent: rigidbodies::RigidBody::default(),
                            }))
                        }
                        _ => (),
                    }

                    // Defaulting the chosen values to zero when the button is clicked.
                    self.position_x = 0.0;
                    self.position_y = 0.0;
                    self.velocity_x = 0.0;
                    self.velocity_y = 0.0;
                }
            });

            /// Displaying a button to run the simulation, which would primarily call the first line.
            ui.horizontal(|ui| {
                if ui.button("Run simulation").clicked() {
                    // painter::draw_frames();

                    ui.label("Building");
                    ui.add(egui::ProgressBar::new(0.0)
                    .animate(true)
                    .show_percentage());                    
                }
            })
        });
    }
}
