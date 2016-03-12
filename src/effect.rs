use super::image::Image;

pub trait Effect {
    fn effect(&Image) -> Image;
}

pub struct EdgeDetection {
    threshold: u8,
}

impl Effect for EdgeDetection {
    fn effect(img: &Image) -> Image {
        unimplemented!();
    }
}
