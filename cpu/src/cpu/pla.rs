use super::{CPUData, ReadWrite, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn PLA(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, _| {
            cpu.sp += 1;
        });
        map.insert(1, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.rw = ReadWrite::R;
        });
        map.insert(2, |cpu, data| {
            cpu.a = data.pins.data;
        });
        map.insert(3, |cpu, _| {
            cpu.ps.set(StatusFlag::Z, cpu.a == 0);
            cpu.ps.set(StatusFlag::N, (cpu.a & 0x80) > 0);
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}
