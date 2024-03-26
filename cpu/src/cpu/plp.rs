use super::{CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn PLP(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, _| {
            cpu.sp += 1;
        });
        map.insert(1, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(2, |cpu, data| {
            cpu.ps = StatusFlag::from_bits_retain(data.pins.data);
        });
        map.insert(3, |cpu, _| {
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}
