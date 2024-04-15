use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::{CPUData, CPU};
use crate::{Memory, ReadWrite};
use cartridge::{Cartridge, CartridgeInfo};
use ppu::PPU;

#[allow(clippy::upper_case_acronyms, dead_code)]
#[derive(Default, Debug)]
pub struct NES {
    pub cpu: CPU,
    data: CPUData,
    memory: Memory,
    pub ppu: PPU,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
    ticks: u32,
}

impl NES {
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.ticks = 0;
    }

    pub fn attach_cart(&mut self, cart: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cart.clone());
        self.ppu.attach_cart(cart);
    }

    pub fn tick(&mut self) {
        self.ppu.tick();

        if self.ticks % 3 == 0 {
            self.cpu.tick(&mut self.data);

            match self.data.pins.rw {
                ReadWrite::R => self.data.pins.data = self.cpu_read(self.data.pins.address),
                ReadWrite::W => self.cpu_write(self.data.pins.address, self.data.pins.data),
            }

            self.ticks = 0;
        }

        self.ticks += 1;
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let mut cart_info = CartridgeInfo::new(address);

        if self
            .cartridge
            .as_ref()
            .unwrap()
            .borrow()
            .cpu_read(&mut cart_info)
        {
            return cart_info.data;
        } else if (0x0000..0x1FFF).contains(&address) {
            return self.memory.data[address as usize];
        } else if (0x2000..=0x3FFF).contains(&address) {
            return self.ppu.cpu_read(address & 0x0007);
        }

        0x00
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        let mut cart_info = CartridgeInfo::new(address);
        cart_info.data = data;

        if self
            .cartridge
            .as_mut()
            .unwrap()
            .borrow_mut()
            .cpu_write(&mut cart_info)
        {
        } else if (0x0000..0x1FFF).contains(&address) {
            self.memory.data[address as usize] = data;
        } else if (0x2000..0x3FFF).contains(&address) {
            self.ppu.cpu_write(address & 0x0007, data);
        }
    }
}
