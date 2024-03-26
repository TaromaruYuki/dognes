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
