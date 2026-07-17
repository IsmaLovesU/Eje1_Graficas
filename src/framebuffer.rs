use bmp::{Image, Pixel};
use raylib::prelude::Color;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    buffer: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::BLACK; (width * height) as usize],
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = (y * self.width + x) as usize;
            self.buffer[index] = self.current_color;
        }
    }

    pub fn render_to_file(&self, filename: &str) {
        let mut image = Image::new(self.width as u32, self.height as u32);

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.buffer[(y * self.width + x) as usize];

                image.set_pixel(
                    x as u32,
                    y as u32,
                    Pixel::new(color.r, color.g, color.b),
                );
            }
        }

        image.save(filename).unwrap();
    }
}