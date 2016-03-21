#![feature(test)]

extern crate test;
use test::Bencher;

extern crate bitmap;
use bitmap::BitMap;
use bitmap::EdgeDetection;
use std::fs::File;

#[bench]
fn detect_edges(b: &mut Bencher) {
    let mut buf = File::open("images/mountain.bmp").unwrap();
    let mut bm = BitMap::load(&mut buf).unwrap();

    b.iter(|| {
        bm.apply_effect::<EdgeDetection>(EdgeDetection::default());
    });

}
