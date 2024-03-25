use super::{AddressingMode, CPUData, ReadWrite, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn STY(&mut self, mode: AddressingMode, data: &mut CPUData) {
        let finish: addressing::CaseFunction = |cpu, _| cpu.instruction_finish();
        match mode {
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(1, addressing::methods::store_zero_page_y);
                map.insert(2, finish);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(2, addressing::methods::store_zero_page_temp_y);
                map.insert(3, finish);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.data = cpu.y;
                    data.pins.rw = ReadWrite::W;
                });
                map.insert(3, finish);

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
    fn STY_ZP() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xAB;

        data.mem.data[0x0000] = opcode::STY_ZP;
        data.mem.data[0x0001] = 0xCD;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0x00CD], 0xAB);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn STY_ZPX() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xAB;
        cpu.x = 0x3;

        data.mem.data[0x0000] = opcode::STY_ZPX;
        data.mem.data[0x0001] = 0xCD;

        for _ in 0..11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0x00D0], 0xAB);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn STY_ABS() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;
        cpu.y = 0xAB;

        data.mem.data[0x0000] = opcode::STY_ABS;
        data.mem.data[0x0001] = 0x09;
        data.mem.data[0x0002] = 0xD0;

        for _ in 0..11 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(data.mem.data[0xD009], 0xAB);
        assert_eq!(cpu.pc, 0x0003);
    }
}
