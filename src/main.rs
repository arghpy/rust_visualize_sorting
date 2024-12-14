use rand::{Rng, thread_rng};
use std::fs::{File, create_dir};
use std::io;
use std::io::{Write, BufWriter};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

// Colors
const FOREGROUND: u32 = 0xFFFFFF; // White
const BACKGROUND: u32 = 0x000000; // Black

fn bubble_sort_visualization() {
    let _ = create_dir("bubble_sort");
    let mut arr = construct_random_array();
    let mut pixels = [0u32; WIDTH * HEIGHT];
    let mut nr = 0;

    // Set by default pixels to a bright green color
    // because we want to color all of them and this
    // green will indicate if we covered all pixels or not
    pixels.fill(0x00FF00);
    bars_array(&mut pixels, &arr);
    save_as_ppm(format!("bubble_sort/round-{}.ppm", nr).as_str(), &pixels).unwrap();
    for _ in 0..(arr.len() - 1) {
        for i in 0..(arr.len() - 1) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);

                pixels.fill(0x00FF00);
                bars_array(&mut pixels, &arr);
                nr += 1;
                save_as_ppm(format!("bubble_sort/round-{}.ppm", nr).as_str(), &pixels).unwrap();
            }
        }
    }
}

fn construct_random_array() -> Vec<usize> {
    let mut rng = thread_rng();
    let mut arr = vec![];

    for _ in 0..WIDTH {
        arr.push(rng.gen_range(1..HEIGHT));
    }
    arr
}

fn save_as_ppm(file_path: &str, pixels: &[u32]) -> io::Result<()> {
    let mut file = BufWriter::with_capacity(WIDTH * HEIGHT * 3, File::create(file_path)?);
    write!(file, "P6\n{} {} 255\n", WIDTH, HEIGHT)?;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = pixels[y * WIDTH + x];
            // 0xRRGGBB
            let color = [((pixel >> 8 * 2) & 0xFF) as u8,  // 0xRR
                         ((pixel >> 8 * 1) & 0xFF) as u8,  // 0xGG
                         ((pixel >> 8 * 0) & 0xFF) as u8]; // 0xBB
            file.write(&color)?;
        }
    }
    println!("Saved {}", file_path);
    Ok(())
}

fn bars_array(pixels: &mut [u32], arr: &[usize]) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            pixels[x * HEIGHT + y] = if x >= arr[y] {
                FOREGROUND
            } else {
                BACKGROUND
            };
        }
    }
}

fn main() {
    bubble_sort_visualization();
}
