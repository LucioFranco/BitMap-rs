extern crate bitmap;

use bitmap::{BitMap, EdgeDetection};
use std::fs::File;

fn main() {
    let mut buf = File::open("images/train.bmp").unwrap();
    let mut bm = BitMap::load(&mut buf).unwrap();

    let mut new_bm = BitMap::with_image(bm.apply_effect(EdgeDetection::default()));

    let mut new_buf = File::create("target/train edge.bmp").unwrap();
    new_bm.save(&mut new_buf);
}
