extern crate bitmap;

use bitmap::BitMap;
use std::fs::File;

fn main() {
    let mut buf = File::open("images/train.bmp").unwrap();
    let mut bm = BitMap::load(&mut buf).unwrap();

    let mut buf = File::open("images/train.bmp").unwrap();
    let mut bm = BitMap::load(&mut buf).unwrap();

    let mut new_buf = File::create("target/train example.bmp").unwrap();
    bm.save(&mut new_buf);
}
