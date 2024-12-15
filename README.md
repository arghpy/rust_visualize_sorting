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
$> ffmpeg -framerate 500 -i "bubble_sort/round-%d.ppm" out.mp4
```
