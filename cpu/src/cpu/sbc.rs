use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn SBC(&mut self, mode: AddressingMode, data: &mut CPUData) {
        // OLC explains this instruction the best
        // https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp#L688-L712
        fn sub_data(cpu: &mut CPU, data: &mut CPUData) {
            let value = (data.pins.data as u16) ^ 0xFF;
            cpu.temp16 = (cpu.a as u16) + value + 1 + ((cpu.ps.bits() & 0x1) as u16);

            cpu.ps.set(StatusFlag::C, cpu.temp16 & 0xFF00 > 0);
            cpu.ps.set(StatusFlag::Z, (cpu.temp16 & 0xFF) == 0);
            cpu.ps.set(StatusFlag::N, (cpu.temp16 & 0x80) > 0);
            cpu.ps.set(
                StatusFlag::V,
                (cpu.temp16 ^ (cpu.a as u16)) & (cpu.temp16 ^ value) & 0x0080 > 0,
            );

            cpu.a = (cpu.temp16 & 0xFF) as u8;
        }

        match mode {
            AddressingMode::Immediate => {
                let mut map = addressing::immediate();
                map.insert(1, sub_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, sub_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(3, sub_data);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, sub_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, sub_data)
                });
                map.insert(4, sub_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteY => {
                let mut map = addressing::absolute_y_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, sub_data)
                });
                map.insert(4, sub_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndexedIndirect => {
                let mut map = addressing::indirect_x();
                map.insert(5, sub_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndirectIndexed => {
                let mut map = addressing::indirect_y_page();
                map.insert(4, |cpu, data| {
                    addressing::methods::store_if_overflow_or_end(cpu, data, sub_data);
                });
                map.insert(5, sub_data);

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
    fn SBC_IM() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_IM;
        data.mem.data[0x0001] = 0x07;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0xD0 - 0x07);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn SBC_ZP() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_ZP;
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

        assert_eq!(cpu.a, 0xC9);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn SBC_ZPX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.a = 0xD0;
        cpu.x = 0x05;

        data.mem.data[0x0000] = opcode::SBC_ZPX;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn SBC_ABS() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_ABS;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn SBC_ABX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.a = 0xD0;
        cpu.x = 0x05;

        data.mem.data[0x0000] = opcode::SBC_ABX;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn SBC_ABX_page() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.x = 0xF7;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_ABX;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn SBC_ABY() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.y = 0x05;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_ABY;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn SBC_ABY_page() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.y = 0xF7;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_ABY;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn SBC_INX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.x = 0x05;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_INX;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn SBC_INY() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.y = 0x05;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_INY;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn SBC_INY_page() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.pc = 0x0000;
        cpu.y = 0xF7;
        cpu.a = 0xD0;

        data.mem.data[0x0000] = opcode::SBC_INY;
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

        assert_eq!(cpu.a, 0xC9);
        assert!(cpu.ps.contains(StatusFlag::N));
        assert_eq!(cpu.pc, 0x0002);
    }
}
