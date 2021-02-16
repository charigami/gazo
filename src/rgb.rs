use std::fs::File;
use std::path::Path;

use crate::{
    common, pack_u32, pack_u8, rgba_to_argb, unpack_u32, unpack_usize, Histogram,
    PointOperations,
};

/// Represents a image in RGBA color format, where all channels are contained in a single u32 value
#[derive(Debug, Clone)]
pub struct ImgRGBA {
    data: Vec<u32>,
    /// Image width
    width: u32,
    /// Image height
    height: u32,
    /// Image width off by one
    w: usize,
    /// Image height off by one
    h: usize,
    /// Size of the data vector
    len: usize,
}

impl ImgRGBA {
    /// Reads an image from file and converts it into [`ImgRGBA`](crate::rgb::ImgRGBA)
    /// For now only png file formats with RGB and RGBA color types are supported.
    pub fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let decoder = png::Decoder::new(File::open(path)?);
        let (info, mut reader) = decoder.read_info()?;
        let mut img_data = vec![0; info.buffer_size()];
        let data_len = img_data.len();

        reader.next_frame(&mut img_data)?;

        let data = match info.color_type {
            png::ColorType::RGB => {
                let mut data: Vec<u32> = Vec::with_capacity(data_len / 3);

                for px in img_data.chunks(3) {
                    let px = pack_u8(px[0], px[1], px[2], 255u8);
                    data.push(px);
                }

                data
            }
            png::ColorType::RGBA => {
                let mut data: Vec<u32> = Vec::with_capacity(data_len / 4);

                for px in img_data.chunks(4) {
                    let px = pack_u8(px[0], px[1], px[2], px[3]);
                    data.push(px);
                }

                data
            }
            _ => unreachable!("uncovered color type"),
        };

        let len = data.len();

        Ok(Self {
            data,
            len,
            width: info.width,
            height: info.height,
            w: (info.width - 1) as usize,
            h: (info.height - 1) as usize,
        })
    }

    /// Creates a new [`ImgRGBA`](crate::rgb::ImgRGBA) from a given bytes slice
    pub fn from_bytes(
        bytes: &[u8],
        color_type: common::ColorType,
        width: u32,
        height: u32,
    ) -> Self {
        let data_len = bytes.len();

        let data = match color_type {
            common::ColorType::RGB => {
                let mut data: Vec<u32> = Vec::with_capacity(data_len / 3);

                for px in bytes.chunks(3) {
                    let px = pack_u8(px[0], px[1], px[2], 255u8);
                    data.push(px);
                }

                data
            }
            common::ColorType::RGBA => {
                let mut data: Vec<u32> = Vec::with_capacity(data_len / 4);

                for px in bytes.chunks(4) {
                    let px = pack_u8(px[0], px[1], px[2], px[3]);
                    data.push(px);
                }

                data
            }
        };

        let len = data.len();

        Self {
            data,
            len,
            width,
            height,
            w: (width - 1) as usize,
            h: (height - 1) as usize,
        }
    }

    /// Converts [`ImgRGBA`](crate::rgb::ImgRGBA) into a ARGB 32 bit framebuffer
    pub fn to_argb_framebuffer(&self) -> Vec<u32> {
        let mut vec = Vec::with_capacity(self.len);
        for pixel in &self.data {
            vec.push(rgba_to_argb(*pixel));
        }
        vec
    }

    pub fn get_px(&self, x: usize, y: usize) -> u32 {
        self.data[x * self.w + y * self.h]
    }

    pub fn get_px_unpacked_u32(&self, x: usize, y: usize) -> (u32, u32, u32, u32) {
        unpack_u32(self.data[x * self.w + y * self.h])
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn histogram(&self) -> (Histogram, Histogram, Histogram) {
        let mut r_data: [u32; 256] = [0; 256];
        let mut g_data: [u32; 256] = [0; 256];
        let mut b_data: [u32; 256] = [0; 256];

        &self.data.iter().for_each(|px| {
            let (r, g, b, _) = unpack_usize(*px);
            r_data[r] += 1;
            g_data[g] += 1;
            b_data[b] += 1;
        });

        (
            Histogram::from(r_data),
            Histogram::from(g_data),
            Histogram::from(b_data),
        )
    }
}

impl PointOperations for ImgRGBA {
    fn grayscale(&mut self) {
        self.data.iter_mut().for_each(|px| {
            let (r, g, b, a) = unpack_u32(*px);
            let l = (r + g + b) / 3u32;
            *px = pack_u32(l, l, l, a);
        })
    }

    fn invert(&mut self) {
        self.data.iter_mut().for_each(|px| {
            let (r, g, b, a) = unpack_u32(*px);
            *px = pack_u32(255u32 - r, 255u32 - g, 255u32 - b, a);
        })
    }

    fn trashold(&mut self, limit: u32) {
        self.data.iter_mut().for_each(|px| {
            let (mut r, mut g, mut b, a) = unpack_u32(*px);
            r = if r > limit { 255u32 } else { 0u32 };
            g = if g > limit { 255u32 } else { 0u32 };
            b = if b > limit { 255u32 } else { 0u32 };
            *px = pack_u32(r, g, b, a);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grayscale() {
        let mut img = ImgRGBA::from_bytes(&[15, 30, 15], crate::ColorType::RGB, 1, 1);

        img.grayscale();

        let px = img.get_px_unpacked_u32(0, 0);

        assert_eq!(px, (20, 20, 20, 255));
    }

    #[test]
    fn test_invert() {
        let mut img = ImgRGBA::from_bytes(&[255, 255, 255], crate::ColorType::RGB, 1, 1);

        img.invert();

        let px = img.get_px_unpacked_u32(0, 0);

        assert_eq!(px, (0, 0, 0, 255));
    }

    #[test]
    fn test_trashold() {
        let mut img =
            ImgRGBA::from_bytes(&[100, 100, 100, 200, 200, 200], crate::ColorType::RGB, 2, 1);

        img.trashold(100);

        let px1 = img.get_px_unpacked_u32(0, 0);
        let px2 = img.get_px_unpacked_u32(1, 0);

        assert_eq!(px1, (0, 0, 0, 255));
        assert_eq!(px2, (255, 255, 255, 255));
    }
}
