// use std::{path::Path, process::id};

// use eframe::*;
// use egui::{*, epaint::CircleShape};

// use crate::rigidbodies::Updateable;

// /// Incomplete function to draw each frame.
// pub fn draw_frames() {
//     // egui::Painter::add(&self, CircleShape)
// }

// /// What would display the simulation while it was running.
// fn main() where T: Updateable {
//     let options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(1000.0, 1000.0)),
//         multisampling: 4,
//         renderer: eframe::Renderer::Glow,
//         ..Default::default()
//     };

//     eframe::run_native(
//         "Simulation images",
//         options,
//         Box::new(|cc| Box::new(Data::<T>::default())),
//     );
// }

// /// Containing all the objects that would be rendered. Obtained from `ui.rs`
// struct Data<T> where T: Updateable {
//     objects: Vec<T>
// }

// /// Obligatory default state for start-up.
// impl<T> Default for Data<T> where T: Updateable {
//     fn default() -> Self {
//         Self {
//             objects: Vec::new()
//         }
//     }
// }

// /// Incomplete: implementing the App (ui) for Data.
// impl<T> eframe::App for Data<T> where T: Updateable {
//     fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//         let p = egui::Painter::new(*ctx, LayerId {
//             order: Order::Background,
//             id: Id::new(1)
//         }, Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 2000.0, y: 2000.0 } });

//         for object in self.objects {
//             // let radius = object.get_radius();
//             egui::Painter::add(&p, CircleShape {
//                 center: Pos2 { x: 1000.0, y: 1000.0 },
//                 radius: 1.0,
//                 fill: Color32::RED,
//                 stroke: Stroke { width: 0.0, color: Color32::RED },
//             });
//         }
//     }
// }

// fn save_frames() {
//     // std::fs::write(Path::new("outputs/images"), b"e")
// }
