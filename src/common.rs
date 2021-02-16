use minifb::{Key, ScaleMode, Window, WindowOptions};

pub enum ColorType {
    RGB,
    RGBA,
}

pub trait PointOperations {
    /// Converts the image into [grayscale](https://en.wikipedia.org/wiki/Grayscale) image where each color channel will
    /// become a weighted sum of the linear-intensity values
    fn grayscale(&mut self);
    /// Inverts all image channels
    fn invert(&mut self);
    /// [Thresholding](https://en.wikipedia.org/wiki/Thresholding_(image_processing)) operation.
    /// Each color channel below or equal to the limit, will be set zu black (0).
    /// All other color values will be set to white (255).
    fn trashold(&mut self, limit: u32);
}

/// Displays an 32 bit argb buffer using minifb
#[allow(dead_code)]
pub fn display(buffer: &[u32], width: usize, height: usize) {
    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

/// Constructs a single u32 value from R,G,B,A u32 values
#[allow(dead_code)]
#[inline(always)]
pub fn pack_u32(r: u32, g: u32, b: u32, a: u32) -> u32 {
    let mut px: u32 = 0;
    px |= r << 24;
    px |= g << 16;
    px |= b << 8;
    px |= a << 0;
    px
}

/// Deconstructs R,G,B,A u32 values from a single u32 value
#[allow(dead_code)]
#[inline(always)]
pub fn unpack_u32(px: u32) -> (u32, u32, u32, u32) {
    let r: u32 = px >> 24;
    let g: u32 = px >> 16 & 0xFF;
    let b: u32 = px >> 8 & 0x0FF;
    let a: u32 = px >> 0 & 0x00FF;
    (r, g, b, a)
}

/// Constructs a single u32 value from R,G,B,A u8 values
#[allow(dead_code)]
#[inline(always)]
pub fn pack_u8(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let mut px: u32 = 0;
    px |= u32::from(r) << 24;
    px |= u32::from(g) << 16;
    px |= u32::from(b) << 8;
    px |= u32::from(a) << 0;
    px
}

/// Deconstructs R,G,B,A u8 values from a single u32 value
#[allow(dead_code)]
#[inline(always)]
pub fn unpack_u8(px: u32) -> (u8, u8, u8, u8) {
    let r = (px >> 24) as u8;
    let g = (px >> 16 & 0xFF) as u8;
    let b = (px >> 8 & 0x0FF) as u8;
    let a = (px >> 0 & 0x00FF) as u8;
    (r, g, b, a)
}

/// Deconstructs R,G,B,A usize values from a single u32 value
#[allow(dead_code)]
#[inline(always)]
pub fn unpack_usize(px: u32) -> (usize, usize, usize, usize) {
    let r = (px >> 24) as usize;
    let g = (px >> 16 & 0xFF) as usize;
    let b = (px >> 8 & 0x0FF) as usize;
    let a = (px >> 0 & 0x00FF) as usize;
    (r, g, b, a)
}

/// Converts a single u32 value in RGBA format into a single u32 value in ARGB format
#[allow(dead_code)]
#[inline(always)]
pub fn rgba_to_argb(px: u32) -> u32 {
    let r: u32 = px >> 24;
    let g: u32 = px >> 16 & 0xFF;
    let b: u32 = px >> 8 & 0x0FF;
    let a: u32 = px >> 0 & 0x00FF;
    let mut argb = 0u32;
    argb |= u32::from(a) << 24;
    argb |= u32::from(r) << 16;
    argb |= u32::from(g) << 8;
    argb |= u32::from(b) << 0;
    argb
}

/// Deconstructs A,R,G,B u32 values from a single u32 value in ARGB format
#[allow(dead_code)]
#[inline(always)]
pub fn unpack_argb_u32(px: u32) -> (u32, u32, u32, u32) {
    let a: u32 = px >> 24;
    let r: u32 = px >> 16 & 0xFF;
    let g: u32 = px >> 8 & 0x0FF;
    let b: u32 = px >> 0 & 0x00FF;
    (a, r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack_u32() {
        let px = pack_u32(11, 22, 33, 44);
        let rgba = unpack_u32(px);
        assert_eq!(rgba, (11, 22, 33, 44));
    }

    #[test]
    fn test_pack_unpack_u8() {
        let px = pack_u8(11, 22, 33, 44);
        let rgba = unpack_u8(px);
        assert_eq!(rgba, (11, 22, 33, 44));
    }

    #[test]
    fn test_rgba_to_argb() {
        let px = pack_u8(11, 22, 33, 44);
        let argb = rgba_to_argb(px);
        let (a, r, g, b) = unpack_argb_u32(argb);
        assert_eq!((a, r, g, b), (44, 11, 22, 33));
    }
}
