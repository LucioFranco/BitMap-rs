use std::io::{Write, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::image::{Image, Pixel};
use super::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
/// `Header` represents a bitmap header
pub struct Header {
    pub bfSize: u32,
    pub zero: u32,
    pub bfOffBits: u32,

    pub biSize: u32,
    pub biWidth: u32,
    pub biHeight: u32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: u32,
    pub biYPelsPerMeter: u32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}

impl Header {
    pub fn new() -> Self {
        Header {
            bfSize: 0,
            zero: 0,
            bfOffBits: 52,

            biSize: 40,
            biWidth: 0,
            biHeight: 0,
            biPlanes: 1,
            biBitCount: 32, // TODO: update so you can create 24-bit images aswell
            biCompression: 0,
            biSizeImage: 0,
            biXPelsPerMeter: 2835,
            biYPelsPerMeter: 2835,
            biClrUsed: 0,
            biClrImportant: 0,
        }
    }

    pub fn load<B: Write + Read + Seek>(&mut self, buf: &mut B) -> Result<(), Error> {
        try!(buf.seek(SeekFrom::Start(0)));

        try!(buf.read_u16::<LittleEndian>());
        self.bfSize = try!(buf.read_u32::<LittleEndian>());
        self.zero = try!(buf.read_u32::<LittleEndian>());
        self.bfOffBits = try!(buf.read_u32::<LittleEndian>());

        self.biSize = try!(buf.read_u32::<LittleEndian>());
        self.biWidth = try!(buf.read_u32::<LittleEndian>());
        self.biHeight = Self::handle_reverse_height(try!(buf.read_i32::<LittleEndian>()));

        self.biPlanes = try!(buf.read_u16::<LittleEndian>());
        self.biBitCount = try!(buf.read_u16::<LittleEndian>());

        self.biCompression = try!(buf.read_u32::<LittleEndian>());
        self.biSizeImage = try!(buf.read_u32::<LittleEndian>());
        self.biXPelsPerMeter = try!(buf.read_u32::<LittleEndian>());
        self.biYPelsPerMeter = try!(buf.read_u32::<LittleEndian>());
        self.biClrUsed = try!(buf.read_u32::<LittleEndian>());
        self.biClrImportant = try!(buf.read_u32::<LittleEndian>());

        Ok(())
    }

    pub fn save<B: Write + Read + Seek>(&mut self, buf: &mut B) -> Result<(), Error> {
        try!(buf.seek(SeekFrom::Start(0)));

        try!(buf.write_u16::<LittleEndian>(19778));

        try!(buf.write_u32::<LittleEndian>(self.bfSize));
        try!(buf.write_u32::<LittleEndian>(self.zero));
        try!(buf.write_u32::<LittleEndian>(self.bfOffBits));

        try!(buf.write_u32::<LittleEndian>(self.biSize));
        try!(buf.write_u32::<LittleEndian>(self.biWidth));
        try!(buf.write_u32::<LittleEndian>(self.biHeight));

        try!(buf.write_u16::<LittleEndian>(self.biPlanes));
        try!(buf.write_u16::<LittleEndian>(self.biBitCount));

        try!(buf.write_u32::<LittleEndian>(self.biCompression));
        try!(buf.write_u32::<LittleEndian>(self.biSizeImage));
        try!(buf.write_u32::<LittleEndian>(self.biXPelsPerMeter));

        try!(buf.write_u32::<LittleEndian>(self.biYPelsPerMeter));
        try!(buf.write_u32::<LittleEndian>(self.biClrUsed));
        try!(buf.write_u32::<LittleEndian>(self.biClrImportant));

        Ok(())
    }

    /// Returns (width, height)
    pub fn get_size(&self) -> (u32, u32) {
        (self.biWidth, self.biHeight)
    }

    fn handle_reverse_height(height: i32) -> u32 {
        if height < 0 {
            (height * -1) as u32
        } else {
            height as u32
        }
    }
}

/// `Body` represents a bitmap header and `Image` of that `BitMap` image
pub struct Body {
    image: Image,
    header: Header,
}

impl Body {
    pub fn new(header: Header) -> Self {
        Body {
            image: Image::with_size(header.biWidth, header.biHeight),
            header: header,
        }
    }

    pub fn default() -> Self {
        Body {
            image: Image::new(),
            header: Header::new(),
        }
    }

    pub fn load<B: Read + Seek>(&mut self, mut buf: &mut B) -> Result<(), Error> {
        try!(buf.seek(SeekFrom::Start((self.header.bfOffBits as u64))));

        let width = self.header.biWidth;
        let height = self.header.biHeight;

        if self.header.biBitCount == 32u16 {
            try!(self.load_32(&mut buf, width, height));
        }
        // TODO: Write 24bit load

        Ok(())
    }

    fn load_32<B: Read + Seek>(&mut self,
                               buf: &mut B,
                               width: u32,
                               height: u32)
                               -> Result<(), Error> {
        for y in 0..height {
            for x in 0..width {
                let mut pixel = Pixel::default();

                pixel.b = try!(buf.read_u8());
                pixel.g = try!(buf.read_u8());
                pixel.r = try!(buf.read_u8());
                pixel.a = try!(buf.read_u8());

                self.image.set_pixel(x, y, pixel);
            }
        }

        Ok(())
    }

    pub fn save<B: Write + Seek>(&mut self, mut buf: &mut B) -> Result<(), Error> {
        try!(buf.seek(SeekFrom::Start(self.header.bfOffBits as u64)));

        let width = self.header.biWidth;
        let height = self.header.biHeight;

        for y in 0..height {
            for x in 0..width {
                let pixel = self.image.get_pixel(x, y);

                try!(buf.write_u8(pixel.b));
                try!(buf.write_u8(pixel.g));
                try!(buf.write_u8(pixel.r));
                try!(buf.write_u8(pixel.a));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use super::*;

    #[test]
    fn header_load() {
        let mut buf = File::open("test/train.bmp").unwrap();

        let mut header = Header::new();
        header.load(&mut buf);

        assert_eq!((1000, 666), header.get_size());

        let mut buf2 = File::open("test/mountain.bmp").unwrap();
        let mut header = Header::new();

        header.load(&mut buf2);

        assert_eq!((259, 194), header.get_size());
    }

    #[test]
    fn header_save() {
        let mut header1 = Header::new();
        header1.biWidth = 100;
        header1.biHeight = 200;

        let mut buf1 = File::create("target/test1.bmp").unwrap();
        header1.save(&mut buf1);

        let mut header2 = Header::new();
        let mut buf2 = File::open("target/test1.bmp").unwrap();
        header2.load(&mut buf2);

        assert_eq!(header1, header2);
    }

    #[test]
    fn body_load_32bit() {
        let mut img = File::open("test/mountain.bmp").unwrap();

        let mut header = Header::new();
        assert!(header.load(&mut img).is_ok());

        let mut body = Body::new(header);
        assert!(body.load(&mut img).is_ok());
    }

    #[test]
    fn body_save_32bit() {
        let mut img = File::open("test/mountain.bmp").unwrap();

        let mut header = Header::new();
        assert!(header.load(&mut img).is_ok());

        let mut body = Body::new(header.clone());
        assert!(body.load(&mut img).is_ok());

        let mut img2 = File::create("target/mountain new.bmp").unwrap();
        assert!(header.save(&mut img2).is_ok());
        assert!(body.save(&mut img2).is_ok());
    }
}
