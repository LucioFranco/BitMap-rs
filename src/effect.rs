use super::image::Image;

pub trait Effect {
    fn effect(&self, &Image) -> Image;
}

pub struct EdgeDetection {
    threshold: u8,
}

impl Effect for EdgeDetection {
    fn effect(&self, img: &Image) -> Image {
        unimplemented!();
    }
}
