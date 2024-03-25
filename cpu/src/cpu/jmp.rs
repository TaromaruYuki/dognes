use super::{AddressingMode, CPUData, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn JMP(&mut self, mode: AddressingMode, data: &mut CPUData) {
        match mode {
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data_save_addr(cpu, data);
                    cpu.pc = cpu.temp16;
                    cpu.instruction_finish();
                });

                self.run_instruction(map, data);
            }
            AddressingMode::Indirect => {
                // No idea why it says it takes 4 cycles, I don't see how thats possible. So I made it take 5 cycles.
                let mut map = addressing::indirect();
                map.insert(4, |cpu, data| {
                    addressing::methods::create_address_from_data_save_addr(cpu, data);
                    cpu.pc = cpu.temp16;
                    cpu.instruction_finish();
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
    use crate::{opcode, CPUData, ReadWrite, CPU};

    #[test]
    fn JMP_ABS() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::JMP_ABS;
        data.mem.data[0x0001] = 0x07;
        data.mem.data[0x0002] = 0xD0;

        for _ in 0..=9 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.pc, 0xD007);
    }

    #[test]
    fn JMP_IND() {
        let mut data = CPUData::default();
        let mut cpu = CPU::default();
        cpu.reset(&mut data);
        cpu.state = crate::cpu::CPUState::Fetch;
        cpu.pc = 0x0000;

        data.mem.data[0x0000] = opcode::JMP_IND;
        data.mem.data[0x0001] = 0xCD;
        data.mem.data[0x0002] = 0xAB;
        data.mem.data[0xABCD] = 0x07;
        data.mem.data[0xABCE] = 0xD0;

        for _ in 0..=13 {
            cpu.tick(&mut data);

            data.clock.tick();

            match data.pins.rw {
                ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
                ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
            }
        }

        assert_eq!(cpu.pc, 0xD007);
    }
}
