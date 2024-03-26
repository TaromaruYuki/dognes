use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn INC(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn inc_data(cpu: &mut CPU, data: &mut CPUData) {
            (cpu.temp8, _) = data.pins.data.overflowing_add(1);
        }

        fn end_inc(cpu: &mut CPU, _data: &mut CPUData) {
            cpu.ps.set(StatusFlag::Z, cpu.temp8 == 0);
            cpu.ps.set(StatusFlag::N, (cpu.temp8 & 0x80) > 0);
            cpu.instruction_finish();
        }

        match mode {
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(1, addressing::methods::get_byte_from_data_save_addr);
                map.insert(2, inc_data);
                map.insert(3, addressing::methods::store_temp_8_in_temp16);
                map.insert(4, end_inc);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(2, addressing::methods::get_byte_from_temp8_save_addr);
                map.insert(3, inc_data);
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
                map.insert(3, inc_data);
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
                map.insert(3, inc_data);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, |_, _| {}); // Takes 7 cycles, the extra one comes from adding x.
                map.insert(6, end_inc);

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}
