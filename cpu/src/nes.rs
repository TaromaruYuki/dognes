use std::cell::RefCell;
use std::rc::Rc;

use emulator_6502::*;

use cartridge::Cartridge;
pub use ppu::PPU;

#[allow(clippy::upper_case_acronyms, dead_code)]
#[derive(Debug)]
pub struct NES {
    cpu: Rc<RefCell<MOS6502>>,
    memory: Vec<u8>,
    pub ppu: PPU,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
    ticks: u32,
}

impl Interface6502 for NES {
    fn read(&mut self, address: u16) -> u8 {
        if let Some(data) = self.cartridge.as_ref().unwrap().borrow().cpu_read(address) {
            return data;
        } else if (0x0000..=0x1FFF).contains(&address) {
            return self.memory[(address & 0x07FF) as usize];
        } else if (0x2000..=0x3FFF).contains(&address) {
            return self.ppu.cpu_read(address & 0x0007);
        }

        0x00
    }

    fn write(&mut self, address: u16, data: u8) {
        if self
            .cartridge
            .as_mut()
            .unwrap()
            .borrow_mut()
            .cpu_write(address, data)
            .is_some()
        {
        } else if (0x0000..=0x1FFF).contains(&address) {
            self.memory[(address & 0x07FF) as usize] = data;
        } else if (0x2000..=0x3FFF).contains(&address) {
            self.ppu.cpu_write(address & 0x0007, data);
        }
    }
}

impl Default for NES {
    fn default() -> Self {
        Self {
            cpu: Rc::default(),
            memory: vec![0; (1024 * 2 + 1) as usize],
            ppu: PPU::default(),
            cartridge: Option::default(),
            ticks: u32::default(),
        }
    }
}

impl NES {
    pub fn reset(&mut self) {
        let cpu = Rc::clone(&self.cpu);
        cpu.borrow_mut().reset(self);
        self.ticks = 0;
    }

    pub fn get_pc(&self) -> u16 {
        let cpu = Rc::clone(&self.cpu);
        let res = cpu.borrow().get_program_counter();

        res
    }

    pub fn cpu_complete(&mut self) -> bool {
        let cpu = Rc::clone(&self.cpu);
        let cycles_left = cpu.borrow().get_remaining_cycles();

        cycles_left == 0
    }

    pub fn program_counter(&mut self) -> u16 {
        let cpu = Rc::clone(&self.cpu);
        let pc = cpu.borrow().get_program_counter();

        pc
    }

    pub fn attach_cart(&mut self, cart: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cart.clone());
        self.ppu.attach_cart(cart);
    }

    pub fn tick(&mut self) {
        self.ppu.tick();

        if self.ticks % 3 == 0 {
            let cpu = Rc::clone(&self.cpu);
            cpu.borrow_mut().cycle(self);

            if self.ppu.nmi {
                self.ppu.nmi = false;
                cpu.borrow_mut().non_maskable_interrupt_request();
            }

            self.ticks = 0;
        }

        self.ticks += 1;
    }
}
