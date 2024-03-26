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
