use gazo::ImgRGBA;

fn main() {
    let img = ImgRGBA::from_file("images/jellyfish.png").unwrap();

    let (r_h, g_h, b_h) = img.histogram();

    dbg!(r_h, g_h, b_h);
}
