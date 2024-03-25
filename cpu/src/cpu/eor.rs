use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn EOR(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn eor_a_and_data(cpu: &mut CPU, data: &mut CPUData) {
            cpu.a ^= data.pins.data;
            cpu.ps.set(StatusFlag::Z, cpu.a == 0);
            cpu.ps.set(StatusFlag::N, (cpu.a & 0x80) > 0);
            cpu.instruction_finish();
        }

        match mode {
            AddressingMode::Immediate => {
                let mut map = addressing::immediate();
                map.insert(1, eor_a_and_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, eor_a_and_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(3, eor_a_and_data);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, eor_a_and_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, eor_a_and_data)
                });
                map.insert(4, eor_a_and_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteY => {
                let mut map = addressing::absolute_y_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, eor_a_and_data)
                });
                map.insert(4, eor_a_and_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndexedIndirect => {
                let mut map = addressing::indirect_x();
                map.insert(5, eor_a_and_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndirectIndexed => {
                let mut map = addressing::indirect_y_page();
                map.insert(4, |cpu, data| {
                    addressing::methods::store_if_overflow_or_end(cpu, data, eor_a_and_data);
                });
                map.insert(5, eor_a_and_data);

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::{opcode, CPUData, ReadWrite, CPU};

    #[test]
    fn EOR_IM() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_IM;
        data.mem.data[0x0001] = 0xCD;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn EOR_ZP() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_ZP;
        data.mem.data[0x0001] = 0x12;
        data.mem.data[0x0012] = 0xCD;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn EOR_ZPX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xAB;
        cpu.x = 0x05;

        data.mem.data[0x0000] = opcode::EOR_ZPX;
        data.mem.data[0x0001] = 0x12;
        data.mem.data[0x0017] = 0xCD;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn EOR_ABS() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_ABS;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD009] = 0xCD;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn EOR_ABX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.a = 0xAB;
        cpu.x = 0x05;

        data.mem.data[0x0000] = opcode::EOR_ABX;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD00E] = 0xCD;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn EOR_ABX_page() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.x = 0xF7;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_ABX;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD100] = 0xCD;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn EOR_ABY() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0x05;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_ABY;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD00E] = 0xCD;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn EOR_ABY_page() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xF7;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_ABY;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD100] = 0xCD;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn EOR_INX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.x = 0x05;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_INX;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x00D2] = 0x09;
        data.mem.data[0x00D3] = 0xD0;
        data.mem.data[0xD009] = 0xCD;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn EOR_INY() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0x05;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_INY;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x00CD] = 0x09;
        data.mem.data[0x00CE] = 0xD0;
        data.mem.data[0xD00E] = 0xCD;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn EOR_INY_page() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xF7;
        cpu.a = 0xAB;

        data.mem.data[0x0000] = opcode::EOR_INY;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x00CD] = 0x09;
        data.mem.data[0x00CE] = 0xD0;
        data.mem.data[0xD100] = 0xCD;

        for _ in 0..=15 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.a, 0x66);
        assert_eq!(cpu.pc, 0x0002);
    }
}
