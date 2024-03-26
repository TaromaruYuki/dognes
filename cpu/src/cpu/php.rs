use super::{CPUData, ReadWrite, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn PHP(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, data| {
            data.pins.address = (0x01_u16 << 8) | cpu.sp as u16;
            data.pins.data = cpu.ps.bits();
            data.pins.rw = ReadWrite::W;
        });
        map.insert(1, |cpu, _| {
            cpu.sp -= 1;
        });
        map.insert(2, |cpu, _| {
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}
