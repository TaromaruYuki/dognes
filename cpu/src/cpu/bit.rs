use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BIT(&mut self, mode: AddressingMode, data: &mut CPUData) {
        match mode {
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, |cpu, data| {
                    let res = cpu.a & data.pins.data;
                    cpu.ps.set(StatusFlag::Z, res == 0);
                    cpu.ps.set(StatusFlag::V, ((res & 0x40) >> 6) > 0);
                    cpu.ps.set(StatusFlag::N, ((res & 0x80) >> 7) > 0);
                });

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, |cpu, data| {
                    let res = cpu.a & data.pins.data;
                    cpu.ps.set(StatusFlag::Z, res == 0);
                    cpu.ps.set(StatusFlag::V, ((res & 0x40) >> 6) > 0);
                    cpu.ps.set(StatusFlag::N, ((res & 0x80) >> 7) > 0);
                });

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
    fn BIT_ZP() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.a = 0xFF;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::BIT_ZP;
        data.mem.data[0x0001] = 0xAB;
        data.mem.data[0x00AB] = 0xC0;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert!(cpu.ps.contains(StatusFlag::V));
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn BIT_ZP_zero() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.a = 0xFF;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::BIT_ZP;
        data.mem.data[0x0001] = 0xAB;
        data.mem.data[0x00AB] = 0x00;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert!(!cpu.ps.contains(StatusFlag::V));
        assert!(!cpu.ps.contains(StatusFlag::N));
        assert!(cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn BIT_ABS() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.a = 0xFF;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::BIT_ABS;
        data.mem.data[0x0001] = 0x07;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD007] = 0xC0;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert!(cpu.ps.contains(StatusFlag::V));
        assert!(cpu.ps.contains(StatusFlag::N));
        assert!(!cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn BIT_ABS_zero() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.a = 0xFF;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::BIT_ABS;
        data.mem.data[0x0001] = 0x07;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD007] = 0x00;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert!(!cpu.ps.contains(StatusFlag::V));
        assert!(!cpu.ps.contains(StatusFlag::N));
        assert!(cpu.ps.contains(StatusFlag::Z));
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 0x0003);
    }
}
