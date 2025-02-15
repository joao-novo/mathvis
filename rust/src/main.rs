pub mod animation;
use std::{
    fs::{create_dir_all, remove_dir_all},
    process::Command,
};

use animation::{axis::draw_axis, background::fill_background};
use clap::{Parser, ValueEnum};
use imageproc::{
    drawing::draw_filled_circle_mut,
    image::{Rgb, RgbImage},
};

#[derive(ValueEnum, Clone, Debug)]
enum Quality {
    LOW,
    MEDIUM,
    HIGH,
    ULTRA,
}

impl Quality {
    fn resolution(&self) -> (u32, u32) {
        match self {
            Quality::LOW => (854, 480),
            Quality::MEDIUM => (1280, 720),
            Quality::HIGH => (1920, 1080),
            Quality::ULTRA => (3840, 2160),
        }
    }
}

impl ToString for Quality {
    fn to_string(&self) -> String {
        match self {
            Quality::LOW => String::from("low"),
            Quality::MEDIUM => String::from("medium"),
            Quality::HIGH => String::from("high"),
            Quality::ULTRA => String::from("ultra"),
        }
    }
}

#[derive(Parser, Debug, Clone)]
struct Args {
    source: String,

    #[arg(long, default_value_t = 30)]
    fps: u32,

    #[arg(short, long, default_value_t = String::from("../output/output.mp4"))]
    output: String,

    #[arg(long, default_value_t = false)]
    gif: bool,

    #[arg(short, long, default_value_t = Quality::HIGH)]
    quality: Quality,
}

fn main() {
    let args = Args::parse();
    create_dir_all("../output/tmp").unwrap();
    let white = Rgb([255, 255, 255]);
    for i in 0..30 {
        let mut img = RgbImage::new(args.quality.resolution().0, args.quality.resolution().1);
        fill_background(&mut img);
        draw_axis(&mut img, white);
        draw_filled_circle_mut(&mut img, ((500 + i * 10), 200), 50, white);
        img.save(format!("../output/tmp/frame_{:03}.png", i))
            .unwrap();
        println!("Generated frame {}", i);
    }
    let ffmpeg_cmd = Command::new("ffmpeg")
        .args([
            "-framerate",
            "30",
            "-i",
            "../output/tmp/frame_%03d.png",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            &args.output,
        ])
        .status()
        .expect("FFmpeg failed");

    if ffmpeg_cmd.success() {
        println!("Video saved as {}", args.output);
    } else {
        println!("FFmpeg error");
    }
    remove_dir_all("../output/tmp").unwrap();
}
