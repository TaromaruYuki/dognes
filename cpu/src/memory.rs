const MAX_MEM: usize = (1024 * 64);

pub struct Memory {
    pub data: [u8; MAX_MEM],
}

impl Default for Memory {
    fn default() -> Self {
        Self { data: [0; MAX_MEM] }
    }
}

impl Memory {
    pub fn reset(&mut self) {
        self.data = [0; MAX_MEM];
    }
}
