use super::{AddressingMode, CPUData, ReadWrite, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn STA(&mut self, mode: AddressingMode, data: &mut CPUData) {
        let finish: addressing::CaseFunction = |cpu, _| cpu.instruction_finish();
        match mode {
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(1, addressing::methods::store_zero_page_a);
                map.insert(2, finish);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(2, addressing::methods::store_zero_page_temp_a);
                map.insert(3, finish);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.data = cpu.a;
                    data.pins.rw = ReadWrite::W;
                });
                map.insert(3, finish);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x();
                map.insert(3, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.data = cpu.a;
                    data.pins.rw = ReadWrite::W;
                });
                map.insert(4, finish);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteY => {
                let mut map = addressing::absolute_y();
                map.insert(3, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.data = cpu.a;
                    data.pins.rw = ReadWrite::W;
                });
                map.insert(4, finish);

                self.run_instruction(map, data);
            }
            AddressingMode::IndexedIndirect => {
                let mut map = addressing::indirect_x();
                map.insert(4, addressing::methods::store_a);
                map.insert(5, finish);

                self.run_instruction(map, data);
            }
            AddressingMode::IndirectIndexed => {
                let mut map = addressing::indirect_y();
                map.insert(4, addressing::methods::store_a);
                map.insert(5, finish);

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}
