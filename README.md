Learning rust by crunching numbers. Gazō (image in japanese) is a simple image processing library.

### Example

```rust
use gazo::{display, ImgRGBA, PointOperations};

fn main() {
    let mut img = ImgRGBA::from_file("images/jellyfish.png").unwrap();

    img.invert();
    img.grayscale();

    let (width, height) = img.dimensions();
    let framebuffer = img.to_argb_framebuffer();

    display(&framebuffer, width as usize, height as usize);
}
```

Running an examples

```bash
cargo run --release --example grayscale
```

### Build instructions

Images are displayed using [minifb](https://github.com/emoon/rust_minifb). For Linux users, please note that you will need to install following dependencies to run the examples:

```bash
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```
