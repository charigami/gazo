use gazo::{display, ImgRGBA, PointOperations};

fn main() {
    let mut img = ImgRGBA::from_file("images/jellyfish.png").unwrap();

    img.invert();

    let (width, height) = img.dimensions();
    let framebuffer = img.to_argb_framebuffer();

    display(&framebuffer, width as usize, height as usize);
}
