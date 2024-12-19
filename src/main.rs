use rand::{thread_rng, Rng};
use std::fs::{create_dir, remove_dir_all, File};
use std::io::{self, BufWriter, Write};
use std::process::{Command, Stdio};

// 720p, 64 elements to sort
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const BAR_WIDTH: usize = 40;

// Colors
const FOREGROUND: u32 = 0xFFFFFF; // White
const BACKGROUND: u32 = 0x000000; // Black

fn bubble_sort_visualization(arr: &mut [usize]) {
    let dir_name = "bubble_sort";
    let _ = create_dir(dir_name);
    let mut pixels = [0u32; WIDTH * HEIGHT];
    let mut nr = 0;

    // Set by default pixels to a bright green color
    // because we want to color all of them and this
    // green will indicate if we covered all pixels or not
    pixels.fill(0x00FF00);
    bars_array(&mut pixels, arr);
    save_as_ppm(
        format!("{}/round-{:0>4}.ppm", dir_name, nr).as_str(),
        &pixels,
    )
    .unwrap();
    for _ in 0..arr.len() {
        for i in 0..(arr.len() - 1) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);

                bars_array(&mut pixels, arr);
                nr += 1;
                save_as_ppm(
                    format!("{}/round-{:0>4}.ppm", dir_name, nr).as_str(),
                    &pixels,
                )
                .unwrap();
            }
        }
    }
    convert_to_whatsapp_compatible_video(dir_name);
    convert_video_to_gif(dir_name);
    let _ = remove_dir_all(dir_name);
}

fn convert_to_whatsapp_compatible_video(path: &str) {
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

fn convert_video_to_gif(path: &str) {
    let pngs_to_gif = Command::new("gifski")
        .arg("-o")
        .arg(format!("{}.gif", path).as_str())
        .arg("--fps")
        .arg("50")
        .arg(format!("{}.mp4", path).as_str())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("process failed to execute");

    if pngs_to_gif.success() {
        println!("Video {}.gif saved.", path);
    }
}

fn save_as_ppm(file_path: &str, pixels: &[u32]) -> io::Result<()> {
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

fn bars_array(pixels: &mut [u32], arr: &[usize]) {
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

fn main() {
    let mut rng = thread_rng();
    let mut arr = vec![];

    for _ in 0..(WIDTH / BAR_WIDTH) {
        // In order to not fill all screen
        arr.push(rng.gen_range(1..(HEIGHT - 5)));
    }
    bubble_sort_visualization(&mut arr.clone());
}
