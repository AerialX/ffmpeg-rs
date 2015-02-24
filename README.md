# ffmpeg-rs

FFmpeg bindings for Rust. Based on [rust-media](https://github.com/pcwalton/rust-media)
with added support for `libavformat`.

## TODO

- `Drop` for `AvCodecContext` and `AvioContext`. FFmpeg takes ownership of the
  pointers in certain cases, so they should not be free'd indiscriminately.
- Clean up `src/util`, it doesn't exactly contain the best interfaces. They're
  useful though.
