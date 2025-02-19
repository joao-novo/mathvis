pub mod animation;
pub mod api;
use std::{
    error::Error,
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use animation::{axis::draw_axis, background::fill_background};
use api::{
    screen::Screen2D,
    util::{interpolate, Quality},
};
use clap::Parser;
use imageproc::{
    drawing::draw_filled_circle_mut,
    image::{Rgb, RgbImage},
};

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

fn generate_frame(args: &Arc<Args>, screen: &Arc<Screen2D>, i: i32) -> Result<(), Box<dyn Error>> {
    let white = Rgb([255, 255, 255]);
    let directory = args
        .output
        .parent()
        .ok_or("Invalid output directory")?
        .to_str()
        .ok_or("Invalid directory path")?;
    let mut img = RgbImage::new(
        args.quality.resolution().0 as u32,
        args.quality.resolution().1 as u32,
    );
    fill_background(&mut img);
    draw_axis(&mut img, white, &screen, &args.quality);
    let (x, y) = interpolate(&args.quality, &screen, (-3.0, -5.0));
    draw_filled_circle_mut(&mut img, ((x as i32 + i * 10), y as i32), 50, white);
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
        Arc::new(Screen2D::new((-10.0, 5.0), (-10.0, 5.0)).ok_or("Invalid screen dimensions")?);
    let completed_frames = Arc::new(Mutex::new(0));
    let total_frames = 30;
    let mut threads = vec![];
    for i in 0..total_frames {
        let args = Arc::clone(&args);
        let screen = Arc::clone(&screen);
        let completed_frames = Arc::clone(&completed_frames);
        let thread = thread::spawn(move || {
            if let Err(e) = generate_frame(&args, &screen, i) {
                eprintln!("Error generating frame {}: {}", i, e);
            } else {
                let mut completed = completed_frames.lock().unwrap();
                *completed += 1;
            }
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().map_err(|_| "Thread panicked")?;
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
