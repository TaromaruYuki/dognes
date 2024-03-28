use std::{cell::RefCell, rc::Rc};

#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Debug)]
pub struct PPU {
    cartridge: Option<Rc<RefCell<cartridge::Cartridge>>>,
    pub scanline: i16,
    pub cycle: i16,
}

impl PPU {
    pub fn tick(&mut self) {
        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 261 {
                self.scanline = -1;
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
