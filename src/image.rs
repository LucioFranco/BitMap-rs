use core::ops::{Deref, DerefMut};

/// `Image` represents the internal storage of the image
#[derive(Debug)]
pub struct Image {
    data: Box<[u8]>,
    width: u32,
    height: u32,
}

impl Image {
    /// Create Empty Image
    pub fn new() -> Self {
        Image {
            data: Box::new([0; 0]),
            width: 0,
            height: 0,
        }
    }

    /// Create Image with certain height and width
    pub fn with_size(width: u32, height: u32) -> Self {
        let v = vec![0u8; (height * width * 4) as usize];

        Image {
            data: v.into_boxed_slice(),
            width: width,
            height: height,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let data = &*self.data;

        let pixel = (data[(y * self.height * 4 + x * self.width * 4) as usize],
                     data[(y * self.height * 4 + x * self.width * 4 + 1) as usize],
                     data[(y * self.height * 4 + x * self.width * 4 + 2) as usize],
                     data[(y * self.height * 4 + x * self.width * 4 + 3) as usize]);

        Pixel::from(pixel)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, p: Pixel) {
        let data = &mut *self.data;

        data[(y * self.width * 4 + x * 4) as usize] = p.b;
        data[(y * self.width * 4 + x * 4 + 1) as usize] = p.g;
        data[(y * self.width * 4 + x * 4 + 2) as usize] = p.r;
        data[(y * self.width * 4 + x * 4 + 3) as usize] = p.a;
    }
}

impl Deref for Image {
    type Target = Box<[u8]>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Box<[u8]> {
        &mut self.data
    }
}

/// `Pixel` represents the B, G, R, A format of BitMap images
#[derive(Debug, Clone, Default)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new(b: u8, g: u8, r: u8, a: u8) -> Self {
        Pixel {
            b: b,
            g: g,
            r: r,
            a: a,
        }
    }
}

/// From data in the form of B, G, R, A 
impl From<(u8, u8, u8, u8)> for Pixel {
    fn from(p: (u8, u8, u8, u8)) -> Self {
        Pixel {
            b: p.0,
            g: p.1,
            r: p.2,
            a: p.3,
        }
    }
}
