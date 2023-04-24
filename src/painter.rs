// use eframe::*;
// use egui::*;

// /// Incomplete function to draw each frame.
// pub fn draw_frames() {
//     egui::Painter::add(&self, CircleShape)
// }

// /// What would display the simulation while it was running.
// fn main() -> Result<(), eframe::Error> {
//     let options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(1000.0, 1000.0)),
//         multisampling: 4,
//         renderer: eframe::Renderer::Glow,
//         ..Default::default()
//     };

//     eframe::run_native(
//         "Simulation images",
//         options,
//         Box::new(|cc| Box::new(Data::new(cc))),
//     )
// }

// /// Containing all the objects that would be rendered. Obtained from `ui.rs`
// struct Data {
//     objects: Box<dyn std::fmt::Display>
// }

// /// Obligatory default state for start-up.
// impl Default for Data {
//     fn default() -> Self {
//         Self {
//             objects: Box::from(Vec::new())
//         }
//     }
// }

// /// Incomplete: implementing the App (ui) for Data.
// impl eframe::App for Data {
//     fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {        
//         egui::Painter::add(&self, CircleShape)
//     }
// }
