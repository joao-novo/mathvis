pub mod animation;
pub mod api;
mod misc;
use std::{
    error::Error,
    fs::{create_dir_all, remove_dir_all},
    process::Command,
    sync::{Arc, Mutex},
};

use animation::show::Show2D;
use animation::vector::Vector2D;
use api::{matrix::Matrix, point::PointLike, screen::Screen2D, util::Args};
use clap::Parser;
use imageproc::image::Rgb;

pub(crate) fn join_frames(args: &Args, directory: String) -> Result<(), Box<dyn Error>> {
    let codec = if args.gif {
        vec!["-f", "gif"]
    } else {
        vec!["-c:v", "libx264", "-pix_fmt", "yuv420p"]
    };
    let ffmpeg_cmd = Command::new("ffmpeg")
        .args([
            "-framerate",
            &args.fps.to_string(),
            "-i",
            &format!("{}/tmp/frame_%03d.png", directory),
            "-nostats",
            "-loglevel",
            "0",
            "-y",
        ])
        .args(&codec)
        .arg(args.output.to_str().ok_or("Invalid output path")?)
        .status()?;

    if ffmpeg_cmd.success() {
        println!("Video saved as {}", args.output.display());
        Ok({})
    } else {
        Err("FFmpeg error".into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let directory = args
        .output
        .parent()
        .ok_or("Invalid output directory")?
        .to_str()
        .ok_or("Invalid directory path")?
        .to_string();

    create_dir_all(format!("{}/tmp", directory))?;

    let white = Rgb([255, 255, 255]);
    let screen = Arc::new(Mutex::new(
        Screen2D::new(
            (-3.0, 3.0),
            (-3.0, 3.0),
            directory.clone(),
            args.fps,
            args.quality.resolution().values()[0] as u32,
            args.quality.resolution().values()[1] as u32,
        )
        .unwrap(),
    ));
    let mut v = Vector2D::new(0.0, 1.0, white);
    v.add_context(screen.clone())?;
    v.rotate_then_scale(
        2.0,
        Matrix::new(vec![vec![1.0, 0.0], vec![1.0, 1.0]]).unwrap(),
    )?;
    join_frames(&args, directory.clone())?;

    remove_dir_all(format!("{}/tmp", directory)).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::util::{interpolate, Quality};

    use super::api::screen::Screen2D;

    use super::*;
    #[test]
    fn test_lerp() {
        let screen = Arc::new(
            Screen2D::new((-10.0, 10.0), (-10.0, 10.0), String::new(), 30, 1920, 1080).unwrap(),
        );
        let quality = Quality::HIGH;
        let (x, y) = interpolate(quality, screen, (0.0, 0.0));
        assert!(x == 960.0 && y == 540.0);
    }
}
