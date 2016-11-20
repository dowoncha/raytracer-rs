pub struct Bitmap {
    width: i32,
    height: i32,
    pixels: Vec<u8>,
    row_bytes: usize
}

impl Bitmap {
    pub fn reset(&mut self) {
        self.width = 0;
        self.height = 0;
        self.pixels.clear();
        self.row_bytes = 0;
    }

    pub fn get_address(&self, x: i32, y: i32) -> i32 {
        return x + (y * self.row_bytes as i32 >> 2);
    }
}
