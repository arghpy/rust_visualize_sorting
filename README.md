# rust-visualize-sorting

[![Super-Linter](https://github.com/arghpy/rust-visualize-sorting/actions/workflows/manage_pull_requests.yaml/badge.svg)](https://github.com/marketplace/actions/super-linter)

This is a project trying to reproduce famous videos show-casing how sorting algorithms work.
Example: [15 Sorting Algorithms in 6 Minutes](https://www.youtube.com/watch?v=kPRA0W1kECg).

To compile it:

```console
$> rustc main.rs
```

To run it:

```console
$> ./main
```

To convert it to video:

```console
$> ffmpeg -y -framerate 60 -i "bubble_sort/round-%d.ppm" out.mp4
```

And this is a format required to send it via [WhatsApp - Web](https://web.whatsapp.com/):


```console
$> ffmpeg -y -i bubble_sort/round-%d.ppm -c:v libx264 -profile:v baseline -level 3.0 -pix_fmt yuv420p out.mp4
```
