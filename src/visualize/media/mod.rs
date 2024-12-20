use std::io::{self, BufWriter, Write};
use std::process::{Command, Stdio};
use std::fs::File;

use crate::WIDTH;
use crate::HEIGHT;
use crate::BAR_WIDTH;

// Colors
use crate::FOREGROUND;
use crate::BACKGROUND;

pub fn save_as_ppm(file_path: &str, pixels: &[u32]) -> io::Result<()> {
    let stdout = std::io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut file = BufWriter::with_capacity(WIDTH * HEIGHT * 3, File::create(file_path)?);
    write!(file, "P6\n{} {} 255\n", WIDTH, HEIGHT)?;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = pixels[y * WIDTH + x];
            // 0xRRGGBB
            let color = [
                ((pixel >> (8 * 2)) & 0xFF) as u8, // 0xRR
                ((pixel >> (8 * 1)) & 0xFF) as u8, // 0xGG
                ((pixel >> (8 * 0)) & 0xFF) as u8, // 0xBB
            ];
            file.write_all(&color)?;
        }
    }
    write!(stdout, "\rGenerated {}", file_path)?;
    Ok(())
}

pub fn bars_array(pixels: &mut [u32], arr: &[usize]) {
    for y in 0..HEIGHT {
        for x in (0..WIDTH).step_by(BAR_WIDTH) {
            pixels[y * WIDTH + x] = if y < HEIGHT - arr[x / BAR_WIDTH] {
                BACKGROUND
            } else {
                FOREGROUND
            };
            for i in 0..BAR_WIDTH {
                pixels[y * WIDTH + x + i] = pixels[y * WIDTH + x];
            }
        }
    }
}

pub fn convert_to_whatsapp_compatible_video(path: &str) {
    let cmd = Command::new("ffmpeg")
        .arg("-y")
        .arg("-framerate")
        .arg("120")
        .arg("-i")
        .arg(format!("{}/round-%04d.ppm", path).as_str())
        .arg("-c:v")
        .arg("libx264")
        .arg("-profile:v")
        .arg("baseline")
        .arg("-level")
        .arg("3.0")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg(format!("{}.mp4", path).as_str())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("process failed to execute");

    if cmd.success() {
        println!("\nVideo {}.mp4 saved.", path);
    }
}

pub fn convert_video_to_gif(path: &str) {
    let mp4_to_gif = Command::new("gifski")
        .arg("-o")
        .arg(format!("{}.gif", path).as_str())
        .arg("--fps")
        .arg("50")
        .arg(format!("{}.mp4", path).as_str())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("process failed to execute");

    if mp4_to_gif.success() {
        println!("Video {}.gif saved.", path);
    }
}
