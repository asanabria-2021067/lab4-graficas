pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background = 0x0d1117;
        Framebuffer {
            width,
            height,
            buffer: vec![background; width * height],
            background,
            current_color: 0xffffff,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background);
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    /// Pinta un píxel con el color actual (si está dentro de la pantalla).
    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.buffer[y as usize * self.width + x as usize] = self.current_color;
        }
    }

    /// Bytes RGBA para actualizar la textura de raylib.
    pub fn to_rgba(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.width * self.height * 4);
        for &px in &self.buffer {
            out.extend_from_slice(&[(px >> 16) as u8, (px >> 8) as u8, px as u8, 255]);
        }
        out
    }

    pub fn save_png(&self, path: &str) -> Result<(), image::ImageError> {
        let mut img = image::RgbImage::new(self.width as u32, self.height as u32);
        for (i, &px) in self.buffer.iter().enumerate() {
            let (x, y) = ((i % self.width) as u32, (i / self.width) as u32);
            img.put_pixel(x, y, image::Rgb([(px >> 16) as u8, (px >> 8) as u8, px as u8]));
        }
        img.save(path)
    }
}
