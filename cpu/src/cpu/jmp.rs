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
