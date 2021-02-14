use gazo::{display, ImgRGBA, PointOperations};

fn main() {
    let mut img = ImgRGBA::from_file("images/jellyfish.png").unwrap();

    img.grayscale();

    let framebuffer = img.to_argb_framebuffer();

    display(&framebuffer, img.width as usize, img.height as usize);
}
