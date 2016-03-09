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
pub use image::{Image, Pixel};

/// `BitMap` represents a bitmap image with a `Header` and `Body`
pub struct BitMap {
    header: Header,
    body: Body,
}

// TODO: have bitmap support any buffer
impl BitMap {
    pub fn new() -> Self {
        BitMap {
            header: Header::new(),
            body: Body::default(),
        }
    }

    pub fn load<B: Write + Read + Seek>(mut buf: &mut B) -> Result<Self, Error> {
        let mut header = Header::new();
        try!(header.load(&mut buf));

        Ok(BitMap {
            header: header,
            body: Body::default(),
        })
    }
}

/// `Error` are errors that can happen during runtime
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
