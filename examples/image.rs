extern crate bitmap;

use bitmap::{BitMap, Image};

fn main() {
    let mut img = Image::new();

    let data: &mut Box<[u8]> = &mut *img;
    let img = &*data;
    img[0];
}
