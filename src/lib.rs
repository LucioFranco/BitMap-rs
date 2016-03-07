//! `BitMap-rs` is a `crate` for bitmap image modification.
//! This library will also support edge detection.

#![crate_name = "bitmap"]
#![crate_type = "rlib"]

extern crate core;
extern crate byteorder;

mod image;
mod bitmap;

use std::io;
use std::io::{Write, Read, Seek};

pub use bitmap::{Header, Body};

pub struct BitMap {
    header: Header,
}

// TODO: have bitmap support any buffer
impl BitMap {
    pub fn load<B: Write + Read + Seek>(mut buf: &mut B) -> Result<Self, Error> {
        let mut header = Header::new();
        try!(header.load(&mut buf));

        Ok(BitMap { header: header })
    }
}

pub enum Error {
    ReadHeader,
    Byteorder(byteorder::Error),
    Io(io::Error),
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Self {
        Error::Byteorder(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
