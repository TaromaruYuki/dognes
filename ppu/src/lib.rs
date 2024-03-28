use std::{cell::RefCell, rc::Rc};

const PAL_WIDTH: i32 = 256;
const PAL_HEIGHT: i32 = 240;

pub type PPUBuf = [[u8; PAL_WIDTH as usize]; PAL_HEIGHT as usize];

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct PPU {
    cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub scanline: i16,
    pub cycle: i16,
    pub frame_complete: bool,
    pub buf: PPUBuf,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            cartridge: None,
            scanline: 0,
            cycle: 0,
            frame_complete: false,
            buf: [[0x0F; PAL_WIDTH as usize]; PAL_HEIGHT as usize],
        }
    }
}

impl PPU {
    pub fn tick(&mut self) {
        let (x, y) = ((self.cycle - 1) as i32, self.scanline as i32);

        if (0..PAL_WIDTH).contains(&x) && (0..PAL_HEIGHT).contains(&y) {
            self.buf[y as usize][x as usize] = if rand::random::<bool>() { 0x20 } else { 0x0F }
        }

        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
    }

    pub fn attach_cart(&mut self, cart: Rc<RefCell<cartridge::Cartridge>>) {
        self.cartridge = Some(cart);
    }

    pub fn cpu_read(&self, address: u16) -> u8 {
        0x00
    }
    pub fn cpu_write(&self, address: u16, data: u8) {
        // Do nothing
    }

    pub fn ppu_read(&self, mut address: u16) -> u8 {
        address &= 0x3FFF;
        let mut cart_info = cartridge::CartridgeInfo::new(address);

        self.cartridge
            .as_ref()
            .unwrap()
            .borrow()
            .ppu_read(&mut cart_info);

        cart_info.data
    }
    pub fn ppu_write(&self, mut address: u16, data: u8) {
        address &= 0x3FFF;
        let mut cart_info = cartridge::CartridgeInfo::new(address);
        cart_info.data = data;

        self.cartridge
            .as_ref()
            .unwrap()
            .borrow()
            .ppu_read(&mut cart_info);
    }
}
