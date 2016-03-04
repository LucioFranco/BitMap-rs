//! `BitMap-rs` is a `crate` for bitmap image modification.
//! This library will also support edge detection.

#![crate_name = "bitmap"]
#![crate_type = "rlib"]

extern crate core;
extern crate byteorder;

pub mod image;
pub mod bitmap;

use std::fs::{OpenOptions, File};
use std::io;
use bitmap::BitMapHeader;

pub struct BitMap {
    header: BitMapHeader<File>,
}

impl BitMap {
    pub fn load(file: &str) -> Result<Self, Error> {
        let buf = try!(OpenOptions::new()
                           .read(true)
                           .open(file));

        let mut header = BitMapHeader::new(buf);
        try!(header.load());

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
