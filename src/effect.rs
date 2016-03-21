use super::image::{Pixel, Image};

/// Any effect on an `Image` must implement trait
pub trait Effect {
    fn apply(&self, &Image) -> Image;
}

#[derive(Default)]
pub struct EdgeDetection {
    pub threshold: u8,
}

impl EdgeDetection {
    fn compute_grey_scale(&self, img: &Image, x: u32, y: u32) -> u8 {
        let mut neighbors = [0i8; 8];

        neighbors[0] = self.compute_from_neighbor(img.get_pixel(x + 1, y));
        neighbors[1] = self.compute_from_neighbor(img.get_pixel(x - 1, y));
        neighbors[2] = self.compute_from_neighbor(img.get_pixel(x, y + 1));
        neighbors[3] = self.compute_from_neighbor(img.get_pixel(x, y - 1));
        neighbors[4] = self.compute_from_neighbor(img.get_pixel(x + 1, y + 1));
        neighbors[5] = self.compute_from_neighbor(img.get_pixel(x - 1, y + 1));
        neighbors[6] = self.compute_from_neighbor(img.get_pixel(x + 1, y - 1));
        neighbors[7] = self.compute_from_neighbor(img.get_pixel(x - 1, y - 1));

        let v_x = (-1) * neighbors[0] + (-2) * neighbors[1] + (-1) * neighbors[2] +
                  (1) * neighbors[5] + (2) * neighbors[6] + (1) * neighbors[7];

        let v_y = (-1) * neighbors[0] + (1) * neighbors[2] + (-2) * neighbors[3] +
                  (2) * neighbors[4] + (-1) * neighbors[5] + (1) * neighbors[7];

        // TODO: figure out this weird type stuff
        (((v_x.pow(2) + v_y.pow(2)) as f32).sqrt()) as u8
    }

    fn compute_from_neighbor(&self, p: Pixel) -> i8 {
        ((p.b + p.g + p.r) / 3) as i8
    }
}

impl Effect for EdgeDetection {
    fn apply(&self, img: &Image) -> Image {
        let (width, height) = img.get_size();
        let mut new_img = Image::with_size(width, height);

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let grey_scale = self.compute_grey_scale(&img, x, y);
                let alpha_val = img.get_pixel(x, y).a;

                let pixel = Pixel::new(grey_scale, grey_scale, grey_scale, alpha_val);
                new_img.set_pixel(x, y, pixel);
            }
        }

        new_img
    }
}
