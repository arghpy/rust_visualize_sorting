use crate::visualize::media::{
    bars_array, convert_to_whatsapp_compatible_video, convert_video_to_gif, save_as_ppm,
};
use std::fs::{create_dir, remove_dir_all};

pub use crate::HEIGHT;
pub use crate::WIDTH;

pub fn bubble_sort_visualization(arr: &mut Vec<usize>) {
    let dir_name = "bubble_sort";
    let _ = create_dir(dir_name);
    //let mut pixels = [0u32; WIDTH * HEIGHT];
    let mut pixels = vec![0; WIDTH * HEIGHT];
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
