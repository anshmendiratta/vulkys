use std::fs;
use std::process::Command;

/// Creates the directory`outputs/images/`if it does not already exist. If it does, do nothing (the error is handled by Rust and the program will not halt). 
/// 
/// The directory is created in the current directory. 
fn create_directory() -> std::io::Result<()> {
    fs::create_dir_all("outputs/images")
}

/// Creates an ffmpeg command to be ran that compiles all images in `outputs/images` and produces a 30fps video with one image per frame.
fn compile_images() -> Command {
    let mut ffmpeg_command = Command::new("ffmpeg");
    ffmpeg_command.arg("-framerate 30");
    ffmpeg_command.arg("-pattern_type glob");
    ffmpeg_command.arg("-i '*.png'");
    ffmpeg_command.arg("-c:v libx264");
    ffmpeg_command.arg("-pix_fmt yuv420p");
    ffmpeg_command.arg("simulation.mp4");

    ffmpeg_command.args(["outputs/images/"]);

    ffmpeg
}

/// Runs both commands.`status()`executes a command and awaits for its status.
fn main() {
    create_directory();
    compile_images().status();
}