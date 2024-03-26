use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn ROR(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn flags(res: u16, cpu: &mut CPU) {
            cpu.ps.set(StatusFlag::Z, (res & 0xFF) == 0);
            cpu.ps.set(StatusFlag::N, (res & 0x80) > 0);
        }

        fn rotate_right(cpu: &mut CPU, data: &mut CPUData) {
            let res = (data.pins.data as u16) >> 1 | (cpu.ps.contains(StatusFlag::C) as u16) << 7;
            cpu.ps.set(StatusFlag::C, (data.pins.data & 0x1) > 0);
            flags(res, cpu);
            cpu.temp8 = (res & 0xFF) as u8;
        }

        fn end_inc(cpu: &mut CPU, _data: &mut CPUData) {
            cpu.instruction_finish();
        }

        match mode {
            AddressingMode::Accumulator => {
                let mut map = addressing::implied();
                map.insert(0, |cpu, _| {
                    let res = (cpu.a as u16) >> 1 | (cpu.ps.contains(StatusFlag::C) as u16) << 7;
                    cpu.ps.set(StatusFlag::C, (cpu.a & 0x1) > 0);
                    flags(res, cpu);
                    cpu.a = (res & 0xFF) as u8;
                });
                map.insert(1, |cpu, _| {
                    cpu.instruction_finish();
                });

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(1, addressing::methods::get_byte_from_data_save_addr);
                map.insert(2, rotate_right);
                map.insert(3, addressing::methods::store_temp_8_in_temp16);
                map.insert(4, end_inc);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(2, addressing::methods::get_byte_from_temp8_save_addr);
                map.insert(3, rotate_right);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, end_inc);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data_save_addr(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, rotate_right);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, end_inc);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_add_x(cpu, data);
                    data.pins.address = cpu.temp16;
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, rotate_right);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, |_, _| {}); // Takes 7 cycles, the extra one comes from adding x.
                map.insert(6, end_inc);

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
    fn ROR_A() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0x12;
        cpu.ps.set(StatusFlag::C, true);

        data.mem.data[0x0000] = opcode::ROR_AC;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x89);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0001);
    }

    #[test]
    fn ROR_ZP() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::C, true);

        data.mem.data[0x0000] = opcode::ROR_ZP;
        data.mem.data[0x0001] = 0xAB;
        data.mem.data[0x00AB] = 0x12;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0x00AB], 0x89);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn ROR_ZPX() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.x = 0x5;
        cpu.ps.set(StatusFlag::C, true);

        data.mem.data[0x0000] = opcode::ROR_ZPX;
        data.mem.data[0x0001] = 0xAB;
        data.mem.data[0x00B0] = 0x12;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0x00B0], 0x89);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn INC_ABS() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.ps.set(StatusFlag::C, true);

        data.mem.data[0x0000] = opcode::ROR_ABS;
        data.mem.data[0x0001] = 0x07;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD007] = 0x12;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0xD007], 0x89);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn INC_ABX() {
        let mut data = CPUData {
            mem: crate::memory::Memory::new(0xFFFF),
            ..Default::default()
        };
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.x = 0x09;
        cpu.ps.set(StatusFlag::C, true);

        data.mem.data[0x0000] = opcode::ROR_ABX;
        data.mem.data[0x0001] = 0x07;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD010] = 0x12;

        for _ in 0..=17 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0xD010], 0x89);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }
}
