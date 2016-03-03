//! `BitMap-rs` is a `crate` for bitmap image modification.
//! This library will also support edge detection.

#![crate_name = "bitmap"]
#![crate_type = "rlib"]

extern crate core;
extern crate byteorder;

pub mod image;
pub mod bitmap;

pub enum Error {
    ReadHeader,
    Byteorder(byteorder::Error),
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Self {
        Error::Byteorder(err)
    }
}
