use super::{CPUData, StatusFlag, CPU};
use crate::addressing;

#[allow(non_snake_case)]
impl CPU {
    pub(super) fn TAY(&mut self, data: &mut CPUData) {
        let mut map = addressing::implied();
        map.insert(0, |cpu, _| {
            cpu.y = cpu.a;
        });
        map.insert(1, |cpu, _| {
            cpu.ps.set(StatusFlag::Z, cpu.y == 0);
            cpu.ps.set(StatusFlag::N, (cpu.y & 0x80) > 0);
            cpu.instruction_finish();
        });

        self.run_instruction(map, data);
    }
}
