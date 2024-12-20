use std::fs::{create_dir, remove_dir_all};
pub use rust_visualize_sorting::WIDTH;

mod media;

pub fn insertion_sort_visualization(arr: &mut [usize]) {
    let dir_name = "insertion_sort";
    let _ = create_dir(dir_name);
    let mut pixels = [0u32; WIDTH * HEIGHT];
    let mut nr = 0;

    // Set by default pixels to a bright green color
    // because we want to color all of them and this
    // green will indicate if we covered all pixels or not
    pixels.fill(0x00FF00);
    media::bars_array(&mut pixels, arr);
    media::save_as_ppm(
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

            media::bars_array(&mut pixels, arr);
            media::save_as_ppm(
                format!("{}/round-{:0>4}.ppm", dir_name, nr).as_str(),
                &pixels,
            )
            .unwrap();
            nr += 1;
        }
        i += 1;
    }
    media::convert_to_whatsapp_compatible_video(dir_name);
    media::convert_video_to_gif(dir_name);
    let _ = remove_dir_all(dir_name);
}

pub fn bubble_sort_visualization(arr: &mut [usize]) {
    let dir_name = "bubble_sort";
    let _ = create_dir(dir_name);
    let mut pixels = [0u32; WIDTH * HEIGHT];
    let mut nr = 0;

    // Set by default pixels to a bright green color
    // because we want to color all of them and this
    // green will indicate if we covered all pixels or not
    pixels.fill(0x00FF00);
    media::bars_array(&mut pixels, arr);
    media::save_as_ppm(
        format!("{}/round-{:0>4}.ppm", dir_name, nr).as_str(),
        &pixels,
    )
    .unwrap();
    for _ in 0..arr.len() {
        for i in 0..(arr.len() - 1) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);

                media::bars_array(&mut pixels, arr);
                nr += 1;
                media::save_as_ppm(
                    format!("{}/round-{:0>4}.ppm", dir_name, nr).as_str(),
                    &pixels,
                )
                .unwrap();
            }
        }
    }
    media::convert_to_whatsapp_compatible_video(dir_name);
    media::convert_video_to_gif(dir_name);
    let _ = remove_dir_all(dir_name);
}
