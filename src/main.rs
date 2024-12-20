use rand::{thread_rng, Rng};

mod algorithms;

// 720p, 64 elements to sort
pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
pub const BAR_WIDTH: usize = 20;

// Colors
pub const FOREGROUND: u32 = 0xFFFFFF; // White
pub const BACKGROUND: u32 = 0x000000; // Black

fn main() {
    let mut rng = thread_rng();
    let mut arr = vec![];

    for _ in 0..(WIDTH / BAR_WIDTH) {
        // In order to not fill all screen height
        arr.push(rng.gen_range(1..(HEIGHT - 5)));
    }
    algorithms::bubble_sort_visualization(&mut arr.clone());
    algorithms::insertion_sort_visualization(&mut arr.clone());
}
