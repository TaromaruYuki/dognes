use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn LDX(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn data_to_x_reg(cpu: &mut CPU, data: &mut CPUData) {
            cpu.x = data.pins.data;
            cpu.ps.set(StatusFlag::Z, cpu.x == 0);
            cpu.ps.set(StatusFlag::N, (cpu.x & 0x80) > 0);
            cpu.instruction_finish();
        }

        match mode {
            AddressingMode::Immediate => {
                let mut map = addressing::immediate();
                map.insert(1, data_to_x_reg);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, data_to_x_reg);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageY => {
                let mut map = addressing::zero_page_y();
                map.insert(3, data_to_x_reg);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, data_to_x_reg);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteY => {
                let mut map = addressing::absolute_y_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, data_to_x_reg)
                });
                map.insert(4, data_to_x_reg);

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
    fn LDX_IM() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::LDX_IM;
        data.mem.data[0x0001] = 0xAB;

        for _ in 0..=7 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.x, 0xAB);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn LDX_ZP() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::LDX_ZP;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x00CD] = 0xAB;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.x, 0xAB);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn LDX_ZPY() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0x05;

        data.mem.data[0x0000] = opcode::LDX_ZPY;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x00D2] = 0xAB;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.x, 0xAB);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn LDX_ABS() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::LDX_ABS;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD009] = 0xAB;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.x, 0xAB);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn LDX_ABY() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0x05;

        data.mem.data[0x0000] = opcode::LDX_ABY;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD00E] = 0xAB;

        for _ in 0..=11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.x, 0xAB);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn LDX_ABY_page() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xF7;

        data.mem.data[0x0000] = opcode::LDX_ABY;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;
        data.mem.data[0xD100] = 0xAB;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.x, 0xAB);
        assert_eq!(cpu.pc, 0x0003);
    }
}
