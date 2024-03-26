use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn CPY(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn cmp_data(cpu: &mut CPU, data: &mut CPUData) {
            cpu.temp16 = (cpu.y as u16) - (data.pins.data as u16);
            cpu.ps.set(StatusFlag::C, cpu.y >= data.pins.data);
            cpu.ps.set(StatusFlag::Z, (cpu.temp16 & 0xFF) == 0x00);
            cpu.ps.set(StatusFlag::N, cpu.temp16 & 0x80 > 0);
        }

        match mode {
            AddressingMode::Immediate => {
                let mut map = addressing::immediate();
                map.insert(1, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, cmp_data);

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
    fn CPY_IM() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xD0;

        data.mem.data[0x0000] = opcode::CPY_IM;
        data.mem.data[0x0001] = 0x07;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert!(cpu.ps.contains(StatusFlag::C));
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn CPY_ZP() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xD0;

        data.mem.data[0x0000] = opcode::CPY_ZP;
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

        assert!(cpu.ps.contains(StatusFlag::C));
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn CPY_ABS() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xD0;

        data.mem.data[0x0000] = opcode::CPY_ABS;
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

        assert!(cpu.ps.contains(StatusFlag::C));
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn CPY_EQ() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xD0;

        data.mem.data[0x0000] = opcode::CPY_IM;
        data.mem.data[0x0001] = 0xD0;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert!(cpu.ps.contains(StatusFlag::C));
        assert!(!cpu.ps.contains(StatusFlag::N));
        assert!(cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }
}
