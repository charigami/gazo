Learning rust by crunching numbers. Gazō (image in japanese) is a simple image processing library focused on single thread performance.

### Build instructions

Images are displayed using [minifb](https://github.com/emoon/rust_minifb). For Linux users, please note that you will need to install following dependencies to run the examples:

```bash
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```
Running an examples

```bash
cargo run --release --example grayscale
```