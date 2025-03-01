pub mod animation;
pub mod api;
pub mod misc;
use std::{
    error::Error,
    f32::consts::PI,
    fs::{create_dir_all, remove_dir_all},
    process::Command,
    sync::{Arc, Mutex},
};

use animation::{axis::draw_axis, background::fill_background, show::Show2D, vector::Vector2D};
use api::{
    point::PointLike,
    screen::Screen2D,
    util::{Args, Global, Number},
};
use clap::Parser;
use imageproc::image::{Rgb, RgbImage};
use misc::thread_pool::ThreadPool;

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

fn generate_frame(
    args: Arc<Args>,
    mut global: Arc<Mutex<Global>>,
    t: f32,
    i: i32,
) -> Result<(), Box<dyn Error>> {
    let quality = Arc::new(args.quality);
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
    let mut global_lock = global.lock().unwrap();
    global_lock.change_image(&mut img);

    fill_background(&mut img);
    draw_axis(
        &mut img,
        white,
        global_lock.screen.clone(),
        Arc::clone(&quality),
    );
    let mut v = Vector2D::new(t.cos(), t.sin());
    v.add_context(global_lock.to_owned()).unwrap();
    println!("test");
    v.draw(white).unwrap();
    img.save(format!("{}/tmp/frame_{:03}.png", directory, i))?;
    println!("Generated frame {}", i);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let directory = args
        .output
        .parent()
        .ok_or("Invalid output directory")?
        .to_str()
        .ok_or("Invalid directory path")?;

    create_dir_all(format!("{}/tmp", directory))?;

    let completed_frames = Arc::new(Mutex::new(0));
    let global = Arc::new(Mutex::new(Global::new(
        args.clone(),
        (-3.0, 3.0),
        (-3.0, 3.0),
    )));
    let total_frames = 30;
    {
        let thread_pool = ThreadPool::new(30).unwrap();
        for i in 0..total_frames {
            let args_arc = Arc::new(args.clone());
            let global = global.clone();

            let completed_frames = Arc::clone(&completed_frames);
            let t = i as f32 * (2.0 * PI / total_frames as f32);
            thread_pool.execute(move || {
                if let Err(e) = generate_frame(args_arc, global, t, i) {
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
    use crate::api::util::{interpolate, Quality};

    use super::api::screen::Screen2D;

    use super::*;
    #[test]
    fn test_lerp() {
        let screen = Arc::new(Screen2D::new((-10.0, 10.0), (-10.0, 10.0)).unwrap());
        let quality = Arc::new(Quality::HIGH);
        let (x, y) = interpolate(quality, screen, (0.0, 0.0));
        assert!(x == 960.0 && y == 540.0);
    }
}
