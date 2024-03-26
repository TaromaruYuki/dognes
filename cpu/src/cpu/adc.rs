use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn ADC(&mut self, mode: AddressingMode, data: &mut CPUData) {
        // OLC explains this instruction the best
        // https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp#L589-L657
        fn add_data(cpu: &mut CPU, data: &mut CPUData) {
            cpu.temp16 = (cpu.a as u16) + (data.pins.data as u16) + ((cpu.ps.bits() & 0x1) as u16);

            cpu.ps.set(StatusFlag::C, cpu.temp16 > 255);
            cpu.ps.set(StatusFlag::Z, (cpu.temp16 & 0xFF) == 0);
            cpu.ps.set(StatusFlag::N, (cpu.temp16 & 0x80) > 0);
            cpu.ps.set(
                StatusFlag::V,
                (!((cpu.a as u16) ^ (data.pins.data as u16)) & ((cpu.a as u16) ^ (cpu.temp16)))
                    & 0x0080
                    > 0,
            );

            cpu.a = (cpu.temp16 & 0xFF) as u8;
        }

        match mode {
            AddressingMode::Immediate => {
                let mut map = addressing::immediate();
                map.insert(1, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::ZeroPageX => {
                let mut map = addressing::zero_page_x();
                map.insert(3, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteX => {
                let mut map = addressing::absolute_x_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, add_data)
                });
                map.insert(4, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::AbsoluteY => {
                let mut map = addressing::absolute_y_page();
                map.insert(3, |cpu, data| {
                    addressing::methods::get_data_or_return(cpu, data, add_data)
                });
                map.insert(4, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndexedIndirect => {
                let mut map = addressing::indirect_x();
                map.insert(5, add_data);

                self.run_instruction(map, data);
            }
            AddressingMode::IndirectIndexed => {
                let mut map = addressing::indirect_y_page();
                map.insert(4, |cpu, data| {
                    addressing::methods::store_if_overflow_or_end(cpu, data, add_data);
                });
                map.insert(5, add_data);

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}
