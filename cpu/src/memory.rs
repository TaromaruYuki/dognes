#[derive(Debug)]
pub struct Memory {
    pub data: Vec<u8>,
    size: u32,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(1024 * 2)
    }
}

impl Memory {
    pub fn new(size: u32) -> Self {
        Self {
            data: vec![0; size as usize],
            size,
        }
    }

    pub fn reset(&mut self) {
        self.data = vec![0; self.size as usize];
    }
}
