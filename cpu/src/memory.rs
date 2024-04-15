#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
    size: u32,
}

impl Memory {
    pub fn new(size: u32) -> Self {
        Self {
            data: vec![0; (size + 1) as usize],
            size,
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(1024 * 2)
    }
}
