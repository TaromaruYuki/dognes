use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn ADC(&mut self, mode: AddressingMode, data: &mut CPUData) {
        // OLC explains this instruction the best
        // https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp#L589-L657
        fn add_data(cpu: &mut CPU, data: &mut CPUData) {
            cpu.temp16 = (cpu.a as u16) + (data.pins.data as u16) + ((cpu.ps.bits() & 0x1) as u16);

            cpu.ps.set(StatusFlag::C, cpu.temp16 > 255);
            cpu.ps.set(StatusFlag::Z, (cpu.temp16 & 0xFF) == 0);
            cpu.ps.set(StatusFlag::N, (cpu.temp16 & 0x80) > 0);
            cpu.ps.set(
                StatusFlag::V,
                (!((cpu.a as u16) ^ (data.pins.data as u16)) & ((cpu.a as u16) ^ (cpu.temp16)))
                    & 0x0080
                    > 0,
            );

            cpu.a = (cpu.temp16 & 0xFF) as u8;
        }

        match mode {
            AddressingMode::Immediate => {
                let mut map = addressing::immediate();
                map.insert(1, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(3, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, add_data)
                });
                map.insert(4, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteY => {
                let mut map = addressing::absolute_y_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, add_data)
                });
                map.insert(4, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndexedIndirect => {
                let mut map = addressing::indirect_x();
                map.insert(5, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndirectIndexed => {
                let mut map = addressing::indirect_y_page();
                map.insert(4, |cpu, data| {
                    addressing::methods::store_if_overflow_or_end(cpu, data, add_data);
                });
                map.insert(5, add_data);

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::{opcode, CPUData, ReadWrite, StatusFlag, CPU};

    #[test]
    fn ADC_IM() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_IM;
        data.mem.data[0x0001] = 0x07;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ADC_ZP() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_ZP;
        data.mem.data[0x0001] = 0x12;
        data.mem.data[0x0012] = 0x07;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ADC_ZPX() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xD0;
        cpu.x = 0x05;

        data.mem.data[0x0000] = opcode::ADC_ZPX;
        data.mem.data[0x0001] = 0x12;
        data.mem.data[0x0017] = 0x07;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ADC_ABS() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_ABS;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD009] = 0x07;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn ADC_ABX() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xD0;
        cpu.x = 0x05;

        data.mem.data[0x0000] = opcode::ADC_ABX;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD00E] = 0x07;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn ADC_ABX_page() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.x = 0xF7;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_ABX;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD100] = 0x07;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn ADC_ABY() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0x05;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_ABY;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD00E] = 0x07;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn ADC_ABY_page() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xF7;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_ABY;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD100] = 0x07;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn ADC_INX() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.x = 0x05;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_INX;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x00D2] = 0x09;
        data.mem.data[0x00D3] = 0xD0;
        data.mem.data[0xD009] = 0x07;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ADC_INY() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0x05;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_INY;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x00CD] = 0x09;
        data.mem.data[0x00CE] = 0xD0;
        data.mem.data[0xD00E] = 0x07;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ADC_INY_page() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xF7;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::ADC_INY;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x00CD] = 0x09;
        data.mem.data[0x00CE] = 0xD0;
        data.mem.data[0xD100] = 0x07;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD7);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ADC_NO_FLAGS() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0x01;

        data.mem.data[0x0000] = opcode::ADC_IM;
        data.mem.data[0x0001] = 0x05;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x06);
        assert_eq!(cpu.ps.bits(), 0x00);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ADC_CARRY_AND_ZERO() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xFF;

        data.mem.data[0x0000] = opcode::ADC_IM;
        data.mem.data[0x0001] = 0x01;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x00);
        assert!(cpu.ps.contains(StatusFlag::Z));
        assert!(cpu.ps.contains(StatusFlag::C));
        assert!(!cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::V));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ADC_OF_AND_NEG() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0x7F;

        data.mem.data[0x0000] = opcode::ADC_IM;
        data.mem.data[0x0001] = 0x01;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x80);
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert!(!cpu.ps.contains(StatusFlag::C));
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(cpu.ps.contains(StatusFlag::V));
        assert_eq!(cpu.pc, 0x0002);
    }
}
