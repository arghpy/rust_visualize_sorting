# rust_visualize_sorting

[![Super-Linter](https://github.com/arghpy/rust_visualize_sorting/actions/workflows/manage_pull_requests.yaml/badge.svg)](https://github.com/marketplace/actions/super-linter)

This is a project trying to reproduce famous videos show-casing how sorting algorithms work.
Example: [15 Sorting Algorithms in 6 Minutes](https://www.youtube.com/watch?v=kPRA0W1kECg).

## Prerequisites

In order for the program to work you will need to install:
- rust
- cargo
- ffmpeg

## Run

Compile it:

```console
cargo build --release
```

Run it:

```console
cargo run
```

Example - Bubble sort:

![Bubble sort](./assets/bubble_sort.gif)

## Notes

At the moment, two formats will be generated by default:
- GIF
- mp4

Implemented algorithms:
- bubble sort
- insertion sort

In the future, more algorithms will be implemented and the user will be able to opt for them
as well as for the video format.
