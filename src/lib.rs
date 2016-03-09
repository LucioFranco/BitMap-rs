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
    body: Body,
}

// TODO: have bitmap support any buffer
impl BitMap {
    pub fn new() -> Self {
        BitMap { body: Body::default() }
    }

    pub fn load<B: Read + Seek>(mut buf: &mut B) -> Result<BitMap, Error> {
        let mut header = Header::new();
        try!(header.load(&mut buf));

        let mut body = Body::new(header);
        try!(body.load(&mut buf));

        Ok(BitMap { body: body })
    }

    pub fn save<B: Write + Seek>(&mut self, mut buf: &mut B) -> Result<(), Error> {
        self.body.save(&mut buf)
    }
}

/// `Error` are errors that can happen during runtime
#[derive(Debug)]
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
