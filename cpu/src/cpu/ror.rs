use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn ROR(&mut self, mode: AddressingMode, data: &mut CPUData) {
        fn flags(res: u16, cpu: &mut CPU) {
            cpu.ps.set(StatusFlag::Z, (res & 0xFF) == 0);
            cpu.ps.set(StatusFlag::N, (res & 0x80) > 0);
        }

        fn rotate_right(cpu: &mut CPU, data: &mut CPUData) {
            let res = (data.pins.data as u16) >> 1 | (cpu.ps.contains(StatusFlag::C) as u16) << 7;
            cpu.ps.set(StatusFlag::C, (data.pins.data & 0x1) > 0);
            flags(res, cpu);
            cpu.temp8 = (res & 0xFF) as u8;
        }

        fn end_inc(cpu: &mut CPU, _data: &mut CPUData) {
            cpu.instruction_finish();
        }

        match mode {
            AddressingMode::Accumulator => {
                let mut map = addressing::implied();
                map.insert(0, |cpu, _| {
                    let res = (cpu.a as u16) >> 1 | (cpu.ps.contains(StatusFlag::C) as u16) << 7;
                    cpu.ps.set(StatusFlag::C, (cpu.a & 0x1) > 0);
                    flags(res, cpu);
                    cpu.a = (res & 0xFF) as u8;
                });
                map.insert(1, |cpu, _| {
                    cpu.instruction_finish();
                });

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(1, addressing::methods::get_byte_from_data_save_addr);
                map.insert(2, rotate_right);
                map.insert(3, addressing::methods::store_temp_8_in_temp16);
                map.insert(4, end_inc);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(2, addressing::methods::get_byte_from_temp8_save_addr);
                map.insert(3, rotate_right);
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
                map.insert(3, rotate_right);
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
                map.insert(3, rotate_right);
                map.insert(4, addressing::methods::store_temp_8_in_temp16);
                map.insert(5, |_, _| {}); // Takes 7 cycles, the extra one comes from adding x.
                map.insert(6, end_inc);

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}
