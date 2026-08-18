pub struct Buffer {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn fill_all(&mut self, color: u32) {
        for i in self.buffer.iter_mut() {
            *i = color;
        }
    }

    pub fn try_fill_pixel(&mut self, (x, y): (i32, i32), color: u32) -> Option<()> {
        if !(0..self.height as i32).contains(&x) {
            return None;
        };
        if !(0..self.width as i32).contains(&y) {
            return None;
        };

        self.fill_pixel((x as usize, y as usize), color);
        Some(())
    }

    pub fn fill_pixel(&mut self, (x, y): (usize, usize), color: u32) {
        self.buffer[x + self.width * y] = color;
    }
}
