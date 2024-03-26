use super::{AddressingMode, CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn BIT(&mut self, mode: AddressingMode, data: &mut CPUData) {
        match mode {
            AddressingMode::ZeroPage => {
                let mut map = addressing::zero_page();
                map.insert(2, |cpu, data| {
                    let res = cpu.a & data.pins.data;
                    cpu.ps.set(StatusFlag::Z, res == 0);
                    cpu.ps.set(StatusFlag::V, ((res & 0x40) >> 6) > 0);
                    cpu.ps.set(StatusFlag::N, ((res & 0x80) >> 7) > 0);
                });

                self.run_instruction(map, data);
            }
            AddressingMode::Absolute => {
                let mut map = addressing::absolute();
                map.insert(2, |cpu, data| {
                    addressing::methods::create_address_from_data(cpu, data);
                    data.pins.rw = ReadWrite::R;
                });
                map.insert(3, |cpu, data| {
                    let res = cpu.a & data.pins.data;
                    cpu.ps.set(StatusFlag::Z, res == 0);
                    cpu.ps.set(StatusFlag::V, ((res & 0x40) >> 6) > 0);
                    cpu.ps.set(StatusFlag::N, ((res & 0x80) >> 7) > 0);
                });

                self.run_instruction(map, data);
            }
            _ => panic!("Should never reach."),
        }
    }
}
