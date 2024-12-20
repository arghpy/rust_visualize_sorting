use crate::visualize::algorithms::{
    bubble_sort::bubble_sort_visualization, insertion_sort::insertion_sort_visualization,
};
use rand::{thread_rng, Rng};
use std::thread;

mod visualize;

// 720p, 64 elements to sort
pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
pub const BAR_WIDTH: usize = 20;

// Colors
pub const FOREGROUND: u32 = 0xFFFFFF; // White
pub const BACKGROUND: u32 = 0x000000; // Black

fn main() {
    let mut rng = thread_rng();
    //let mut bubble_sort_arr = Vec::new();
    let mut insertion_sort_arr = Vec::new();

    for _ in 0..(WIDTH / BAR_WIDTH) {
        let nr = rng.gen_range(1..(HEIGHT - 5));
        // In order to not fill all screen height
        //bubble_sort_arr.push(nr);
        insertion_sort_arr.push(nr);
    }

    //let handle_1 = thread::spawn(move || {
    //    bubble_sort_visualization(&mut bubble_sort_arr);
    //});

    let handle_2 = thread::spawn(move || {
        insertion_sort_visualization(&mut insertion_sort_arr);
    });

    //handle_1.join().unwrap();
    handle_2.join().unwrap();
}
