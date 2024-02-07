use image::{
    imageops::{self, overlay},
    DynamicImage, GenericImage, GenericImageView, Rgba,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Image {
    image: DynamicImage,
}

#[wasm_bindgen]
impl Image {
    pub fn alloc_form_size(width: i32, height: i32) -> Self {
        Self {
            image: DynamicImage::new_rgba8(width as u32, height as u32),
        }
    }

    pub fn from_buffer(buffer: &[u8], start: i32, end: i32) -> Self {
        log::info!("from_image: start: {}, end: {}", start, end);

        Self {
            image: image::load_from_memory(&buffer[start as usize..end as usize]).unwrap(),
        }
    }

    pub fn cut_from_extents(&self, x: i32, y: i32, width: i32, height: i32) -> Self {
        log::info!(
            "from_image: x: {}, y: {}, width: {}, height: {}",
            x,
            y,
            width,
            height
        );
        Self {
            image: self
                .image
                .crop_imm(x as u32, y as u32, width as u32, height as u32),
        }
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
        log::info!("fill: r: {}, g: {}, b: {}, a: {:?}", r, g, b, a);

        let width = self.image.width();
        let height = self.image.height();

        for y in 0..height {
            for x in 0..width {
                self.image.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }
    }

    pub fn draw(&mut self, other: &Image, x: i32, y: i32) {
        log::info!("draw: x: {}, y: {}", x, y);

        overlay(&mut self.image, &other.image, x as u32, y as u32)
    }

    pub fn width(&self) -> i32 {
        self.image.width() as i32
    }

    pub fn height(&self) -> i32 {
        self.image.height() as i32
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        log::info!("resize: width: {}, height: {}", width, height);
        self.image
            .resize(width as u32, height as u32, imageops::FilterType::Nearest);
    }

    pub fn into_buffer(&self, format: &str) -> Vec<u8> {
        let mut buffer = Vec::new();
        match format {
            "jpeg" | "jpg" => {
                self.image
                    .write_to(&mut buffer, image::ImageOutputFormat::Jpeg(10))
                    .unwrap();
            }
            "png" => {
                self.image
                    .write_to(&mut buffer, image::ImageOutputFormat::Png)
                    .unwrap();
            }
            _ => log::error!("nut suport format: {}!!", format),
        }

        buffer
    }
}
