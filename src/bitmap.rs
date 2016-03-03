use std::io::{Write, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
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
}

#[derive(Default)]
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
