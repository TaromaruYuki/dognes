use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn TXA(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, _| {
            cpu.a = cpu.x;
        });
        map.insert(1, |cpu, _| {
            cpu.ps.set(StatusFlag::Z, cpu.a == 0);
            cpu.ps.set(StatusFlag::N, (cpu.a & 0x80) > 0);
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}
