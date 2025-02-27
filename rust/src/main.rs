pub mod animation;
pub mod api;
pub mod misc;
use std::{
    error::Error,
    f32::consts::PI,
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
};

use animation::{axis::draw_axis, background::fill_background, vector::draw_vector};
use api::{point::PointLike, screen::Screen2D, util::Quality, vector::Vector};
use clap::Parser;
use imageproc::image::{Rgb, RgbImage};
use misc::thread_pool::ThreadPool;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
    source: String,

    #[arg(long, default_value_t = 30)]
    fps: u32,

    #[arg(short, long, default_value_os = "../output/output.mp4")]
    output: PathBuf,

    #[arg(long, default_value_t = false)]
    gif: bool,

    #[arg(short, long, default_value_t = Quality::HIGH)]
    quality: Quality,
}

pub fn join_frames(args: &Args, directory: &str) -> Result<(), Box<dyn Error>> {
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

fn generate_frame(
    args: &Arc<Args>,
    screen: &Arc<Screen2D>,
    t: f32,
    i: i32,
) -> Result<(), Box<dyn Error>> {
    let white = Rgb([255, 255, 255]);
    let directory = args
        .output
        .parent()
        .ok_or("Invalid output directory")?
        .to_str()
        .ok_or("Invalid directory path")?;
    let mut img = RgbImage::new(
        args.quality.resolution().values()[0] as u32,
        args.quality.resolution().values()[1] as u32,
    );
    let w = t + PI / 2.0;
    fill_background(&mut img);
    draw_axis(&mut img, white, &screen, &args.quality);
    draw_vector(
        &Vector::new(vec![t.cos(), t.sin()]).unwrap(),
        &mut img,
        white,
        &screen,
        &args.quality,
    );
    draw_vector(
        &Vector::new(vec![w.cos(), w.sin()]).unwrap(),
        &mut img,
        white,
        &screen,
        &args.quality,
    );
    img.save(format!("{}/tmp/frame_{:03}.png", directory, i))?;
    println!("Generated frame {}", i);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arc::new(Args::parse());
    let directory = args
        .output
        .parent()
        .ok_or("Invalid output directory")?
        .to_str()
        .ok_or("Invalid directory path")?;

    create_dir_all(format!("{}/tmp", directory))?;

    let screen =
        Arc::new(Screen2D::new((-3.0, 3.0), (-3.0, 3.0)).ok_or("Invalid screen dimensions")?);
    let completed_frames = Arc::new(Mutex::new(0));
    let total_frames = 120;
    {
        let thread_pool = ThreadPool::new(30).unwrap();
        for i in 0..total_frames {
            let args = Arc::clone(&args);
            let screen = Arc::clone(&screen);
            let completed_frames = Arc::clone(&completed_frames);
            let t = i as f32 * (2.0 * PI / total_frames as f32);
            thread_pool.execute(move || {
                if let Err(e) = generate_frame(&args, &screen, t, i) {
                    eprintln!("Error generating frame {}: {}", i, e);
                } else {
                    let mut completed = completed_frames.lock().unwrap();
                    *completed += 1;
                }
            });
        }
    }

    let completed = *completed_frames.lock().unwrap();
    if completed != total_frames {
        return Err(format!(
            "Only {} of {} frames were generated",
            completed, total_frames
        )
        .into());
    }

    join_frames(&args, directory)?;

    remove_dir_all(format!("{}/tmp", directory)).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::util::interpolate;

    use super::api::screen::Screen2D;

    use super::*;
    #[test]
    fn test_lerp() {
        let screen = Screen2D::new((-10.0, 10.0), (-10.0, 10.0)).unwrap();
        let quality = Quality::HIGH;
        let (x, y) = interpolate(&quality, &screen, (0.0, 0.0));
        assert!(x == 960.0 && y == 540.0);
    }
}
