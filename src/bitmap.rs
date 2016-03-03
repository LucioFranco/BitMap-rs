use std::io::{Write, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};
use super::Error;

pub struct BitMapHeader<B: Write + Read + Seek> {
    buf: B,
    meta_data: BitMapHeaderMetadata,
}

impl<B: Write + Read + Seek> BitMapHeader<B> {
    pub fn new(buf: B) -> Self {
        BitMapHeader {
            buf: buf,
            meta_data: BitMapHeaderMetadata::default(),
        }
    }

    pub fn load(&mut self) -> Result<(), Error> {
        let mut header = &mut self.meta_data;
        let mut buf = &mut self.buf;

        try!(buf.read_u16::<LittleEndian>());
        header.bfSize = try!(buf.read_u32::<LittleEndian>());
        header.zero = try!(buf.read_u32::<LittleEndian>());
        header.bfOffBits = try!(buf.read_u32::<LittleEndian>());

        header.biSize = try!(buf.read_u32::<LittleEndian>());
        header.biWidth = try!(buf.read_u32::<LittleEndian>());
        header.biHeight = try!(buf.read_u32::<LittleEndian>());
        header.biPlanes = try!(buf.read_u16::<LittleEndian>());
        header.biBitCount = try!(buf.read_u16::<LittleEndian>());
        header.biCompression = try!(buf.read_u32::<LittleEndian>());
        header.biSizeImage = try!(buf.read_u32::<LittleEndian>());
        header.biXPelsPerMeter = try!(buf.read_u32::<LittleEndian>());
        header.biYPelsPerMeter = try!(buf.read_u32::<LittleEndian>());
        header.biClrUsed = try!(buf.read_u32::<LittleEndian>());
        header.biClrImportant = try!(buf.read_u32::<LittleEndian>());

        Ok(())
    }

    /// Returns (width, height)
    pub fn get_size(&self) -> (u32, u32) {
        (self.meta_data.biWidth, self.meta_data.biHeight)
    }
}

#[derive(Default, Debug)]
// TODO: remove allow dead_code
#[allow(non_snake_case, dead_code)]
struct BitMapHeaderMetadata {
    bfSize: u32,
    zero: u32,
    bfOffBits: u32,

    biSize: u32,
    biWidth: u32,
    biHeight: u32,
    biPlanes: u16,
    biBitCount: u16,
    biCompression: u32,
    biSizeImage: u32,
    biXPelsPerMeter: u32,
    biYPelsPerMeter: u32,
    biClrUsed: u32,
    biClrImportant: u32,
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use super::*;

    #[test]
    fn header() {
        let mut buf = File::open("test/train.bmp").unwrap();

        let mut header = BitMapHeader::new(buf);
        header.load();

        assert_eq!((1000, 666), header.get_size())
    }
}
