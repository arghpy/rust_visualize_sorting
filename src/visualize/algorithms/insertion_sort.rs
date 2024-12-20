use crate::visualize::media::{
    bars_array, convert_to_whatsapp_compatible_video, convert_video_to_gif, save_as_ppm,
};
use std::fs::{create_dir, remove_dir_all};

pub use crate::HEIGHT;
pub use crate::WIDTH;

pub fn insertion_sort_visualization(arr: &mut [usize]) {
    let dir_name = "insertion_sort";
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

    let mut i = 1;
    let mut j;
    while i < arr.len() {
        j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;

            bars_array(&mut pixels, arr);
            save_as_ppm(
                format!("{}/round-{:0>4}.ppm", dir_name, nr).as_str(),
                &pixels,
            )
            .unwrap();
            nr += 1;
        }
        i += 1;
    }
    convert_to_whatsapp_compatible_video(dir_name);
    convert_video_to_gif(dir_name);
    let _ = remove_dir_all(dir_name);
}
