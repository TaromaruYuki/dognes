use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn CMP(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn cmp_data(cpu: &mut CPU, data: &mut CPUData) {
            cpu.temp16 = (cpu.a as u16) - (data.pins.data as u16);
            cpu.ps.set(StatusFlag::C, cpu.a >= data.pins.data);
            cpu.ps.set(StatusFlag::Z, (cpu.temp16 & 0xFF) == 0x00);
            cpu.ps.set(StatusFlag::N, cpu.temp16 & 0x80 > 0);
        }

        match mode {
            AddressingMode::Immediate => {
                let mut map = addressing::immediate();
                map.insert(1, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(3, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, cmp_data)
                });
                map.insert(4, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteY => {
                let mut map = addressing::absolute_y_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, cmp_data)
                });
                map.insert(4, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndexedIndirect => {
                let mut map = addressing::indirect_x();
                map.insert(5, cmp_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndirectIndexed => {
                let mut map = addressing::indirect_y_page();
                map.insert(4, |cpu, data| {
                    addressing::methods::store_if_overflow_or_end(cpu, data, cmp_data);
                });
                map.insert(5, cmp_data);

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}
