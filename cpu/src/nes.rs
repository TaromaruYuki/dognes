use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::Memory;
use cartridge::{Cartridge, CartridgeInfo};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct NES {
    cpu: CPU,
    memory: Memory,
    cartridge: Option<Cartridge>,
    ticks: u32,
}

impl Default for NES {
    fn default() -> Self {
        let nes = Rc::new(RefCell::new(Self {
            cpu: CPU::default(),
            memory: Memory::default(),
            cartridge: None,
            ticks: 0,
        }));

        nes.borrow_mut().cpu.attach_nes(Rc::clone(&nes));

        Rc::try_unwrap(nes).unwrap().into_inner()
    }
}

impl NES {
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.ticks = 0;
    }

    pub fn cpu_read(&self, address: u16) -> u8 {
        let mut cart_info = CartridgeInfo::new(address);

        if self.cartridge.as_ref().unwrap().cpu_read(&mut cart_info) {
            return cart_info.data;
        } else if (0x0000..0x1FFF).contains(&address) {
            return self.memory.data[address as usize];
        } else if (0x2000..=0x3FFF).contains(&address) {
            // PPU
            todo!();
        }

        0x00
    }
}
