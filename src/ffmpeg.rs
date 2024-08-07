#![allow(dead_code)]

use std::fs;
use std::process::{Command, ExitStatus};

fn create_directory() -> std::io::Result<()> {
    fs::create_dir_all("outputs/images")
}

fn compile_images() -> std::io::Result<ExitStatus> {
    let command_args = [
        "-framerate 30",
        "-pattern_type glob",
        "-i '*.png'",
        "-c:v libx264",
        "-pix_fmt yuv420p",
        "simulation.mp4",
        "outputs/images/",
    ];
    let mut ffmpeg_command = Command::new("ffmpeg");
    ffmpeg_command.args(command_args);

    ffmpeg_command.status()
}
