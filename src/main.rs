use rand::{thread_rng, Rng};
use std::fs::{create_dir, File};
use std::io;
use std::io::{BufWriter, Write};

// 720p, 64 elements to sort
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const BAR_WIDTH: usize = 20;

// Colors
const FOREGROUND: u32 = 0xFFFFFF; // White
const BACKGROUND: u32 = 0x000000; // Black

fn bubble_sort_visualization(arr: &mut [usize]) {
    let _ = create_dir("bubble_sort");
    let mut pixels = [0u32; WIDTH * HEIGHT];
    let mut nr = 0;

    // Set by default pixels to a bright green color
    // because we want to color all of them and this
    // green will indicate if we covered all pixels or not
    pixels.fill(0x00FF00);
    bars_array(&mut pixels, &arr);
    save_as_ppm(format!("bubble_sort/round-{}.ppm", nr).as_str(), &pixels).unwrap();
    for _ in 0..arr.len() {
        for i in 0..(arr.len() - 1) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);

                bars_array(&mut pixels, &arr);
                nr += 1;
                save_as_ppm(format!("bubble_sort/round-{}.ppm", nr).as_str(), &pixels).unwrap();
            }
        }
    }
}

fn save_as_ppm(file_path: &str, pixels: &[u32]) -> io::Result<()> {
    let mut file = BufWriter::with_capacity(WIDTH * HEIGHT * 3, File::create(file_path)?);
    write!(file, "P6\n{} {} 255\n", WIDTH, HEIGHT)?;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = pixels[y * WIDTH + x];
            // 0xRRGGBB
            let color = [
                ((pixel >> 8 * 2) & 0xFF) as u8, // 0xRR
                ((pixel >> 8 * 1) & 0xFF) as u8, // 0xGG
                ((pixel >> 8 * 0) & 0xFF) as u8, // 0xBB
            ];
            file.write(&color)?;
        }
    }
    println!("Saved {}", file_path);
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
