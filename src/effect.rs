use super::image::Image;

/// Any effect on an `Image` must implement trait
pub trait Effect {
    fn apply(&self, &Image) -> Image;
}

pub struct EdgeDetection {
    threshold: u8,
}

impl Effect for EdgeDetection {
    fn apply(&self, img: &Image) -> Image {
        unimplemented!();
    }
}
