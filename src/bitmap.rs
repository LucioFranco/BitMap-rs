use std::io::{Write, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::image::{Image, Pixel};
use super::Error;

#[derive(Debug, PartialEq, Eq)]
// TODO: remove allow dead_code
#[allow(non_snake_case, dead_code)]
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
        try!(buf.read_u16::<LittleEndian>());
        self.bfSize = try!(buf.read_u32::<LittleEndian>());
        self.zero = try!(buf.read_u32::<LittleEndian>());
        self.bfOffBits = try!(buf.read_u32::<LittleEndian>());

        self.biSize = try!(buf.read_u32::<LittleEndian>());
        self.biWidth = try!(buf.read_u32::<LittleEndian>());
        self.biHeight = try!(buf.read_u32::<LittleEndian>());

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
}

pub struct Body {
    image: Image,
    bit_count: u16,
}

impl Body {
    pub fn new(bit_count: u16) -> Self {
        Body {
            image: Image::new(),
            bit_count: bit_count,
        }
    }

    pub fn load<B: Read + Seek>(&mut self, buf: &mut B) -> Result<(), Error> {
        // TODO: write load
        Ok(())
    }

    fn load_u32(&mut self, reverse: bool) {}
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

        assert_eq!((1000, 666), header.get_size())
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
}
